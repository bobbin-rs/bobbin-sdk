
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct SpiPeriph(pub usize); 

impl SpiPeriph {
    #[doc="Get the MCR Register."]
    #[inline] pub fn mcr_reg(&self) -> Register<Mcr> { 
        Register::new(self.0 as *mut Mcr, 0x0)
    }

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        self.mcr_reg().read()
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn write_mcr(&self, value: Mcr) -> &Self { 
        self.mcr_reg().write(value);
        self
    }

    #[doc="Set the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().set(f);
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().with(f);
        self
    }

    #[doc="Get the TCR Register."]
    #[inline] pub fn tcr_reg(&self) -> Register<Tcr> { 
        Register::new(self.0 as *mut Tcr, 0x8)
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const Tcr { 
        self.tcr_reg().ptr()
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        self.tcr_reg().read()
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn write_tcr(&self, value: Tcr) -> &Self { 
        self.tcr_reg().write(value);
        self
    }

    #[doc="Set the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().set(f);
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        self.tcr_reg().with(f);
        self
    }

    #[doc="Get the CTAR Register."]
    #[inline] pub fn ctar_reg(&self) -> RegisterArray<Ctar, bits::R2> { 
        RegisterArray::new(self.0 as *mut Ctar, 0xc, 0x4)
    }

    #[doc="Get the *mut pointer for the CTAR register."]
    #[inline] pub fn ctar_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Ctar { 
        self.ctar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CTAR register."]
    #[inline] pub fn ctar_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Ctar { 
        self.ctar_reg().ptr(index.into())
    }

    #[doc="Read the CTAR register."]
    #[inline] pub fn ctar<I: Into<bits::R2>>(&self, index: I) -> Ctar { 
        self.ctar_reg().read(index.into())
    }

    #[doc="Write the CTAR register."]
    #[inline] pub fn write_ctar<I: Into<bits::R2>>(&self, index: I, value: Ctar) -> &Self {
        self.ctar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CTAR register."]
    #[inline] pub fn set_ctar<I: Into<bits::R2>, F: FnOnce(Ctar) -> Ctar>(&self, index: I, f: F) -> &Self {
        self.ctar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CTAR register."]
    #[inline] pub fn with_ctar<I: Into<bits::R2> + Copy, F: FnOnce(Ctar) -> Ctar>(&self, index: I, f: F) -> &Self {
        self.ctar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CTAR_SLAVE Register."]
    #[inline] pub fn ctar_slave_reg(&self) -> Register<CtarSlave> { 
        Register::new(self.0 as *mut CtarSlave, 0xc)
    }

    #[doc="Get the *mut pointer for the CTAR_SLAVE register."]
    #[inline] pub fn ctar_slave_mut(&self) -> *mut CtarSlave { 
        self.ctar_slave_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTAR_SLAVE register."]
    #[inline] pub fn ctar_slave_ptr(&self) -> *const CtarSlave { 
        self.ctar_slave_reg().ptr()
    }

    #[doc="Read the CTAR_SLAVE register."]
    #[inline] pub fn ctar_slave(&self) -> CtarSlave { 
        self.ctar_slave_reg().read()
    }

    #[doc="Write the CTAR_SLAVE register."]
    #[inline] pub fn write_ctar_slave(&self, value: CtarSlave) -> &Self { 
        self.ctar_slave_reg().write(value);
        self
    }

    #[doc="Set the CTAR_SLAVE register."]
    #[inline] pub fn set_ctar_slave<F: FnOnce(CtarSlave) -> CtarSlave>(&self, f: F) -> &Self {
        self.ctar_slave_reg().set(f);
        self
    }

    #[doc="Modify the CTAR_SLAVE register."]
    #[inline] pub fn with_ctar_slave<F: FnOnce(CtarSlave) -> CtarSlave>(&self, f: F) -> &Self {
        self.ctar_slave_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> Register<Sr> { 
        Register::new(self.0 as *mut Sr, 0x2c)
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

    #[doc="Get the RSER Register."]
    #[inline] pub fn rser_reg(&self) -> Register<Rser> { 
        Register::new(self.0 as *mut Rser, 0x30)
    }

    #[doc="Get the *mut pointer for the RSER register."]
    #[inline] pub fn rser_mut(&self) -> *mut Rser { 
        self.rser_reg().ptr()
    }

    #[doc="Get the *const pointer for the RSER register."]
    #[inline] pub fn rser_ptr(&self) -> *const Rser { 
        self.rser_reg().ptr()
    }

    #[doc="Read the RSER register."]
    #[inline] pub fn rser(&self) -> Rser { 
        self.rser_reg().read()
    }

    #[doc="Write the RSER register."]
    #[inline] pub fn write_rser(&self, value: Rser) -> &Self { 
        self.rser_reg().write(value);
        self
    }

    #[doc="Set the RSER register."]
    #[inline] pub fn set_rser<F: FnOnce(Rser) -> Rser>(&self, f: F) -> &Self {
        self.rser_reg().set(f);
        self
    }

    #[doc="Modify the RSER register."]
    #[inline] pub fn with_rser<F: FnOnce(Rser) -> Rser>(&self, f: F) -> &Self {
        self.rser_reg().with(f);
        self
    }

    #[doc="Get the PUSHR Register."]
    #[inline] pub fn pushr_reg(&self) -> Register<Pushr> { 
        Register::new(self.0 as *mut Pushr, 0x34)
    }

    #[doc="Get the *mut pointer for the PUSHR register."]
    #[inline] pub fn pushr_mut(&self) -> *mut Pushr { 
        self.pushr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUSHR register."]
    #[inline] pub fn pushr_ptr(&self) -> *const Pushr { 
        self.pushr_reg().ptr()
    }

    #[doc="Read the PUSHR register."]
    #[inline] pub fn pushr(&self) -> Pushr { 
        self.pushr_reg().read()
    }

    #[doc="Write the PUSHR register."]
    #[inline] pub fn write_pushr(&self, value: Pushr) -> &Self { 
        self.pushr_reg().write(value);
        self
    }

    #[doc="Set the PUSHR register."]
    #[inline] pub fn set_pushr<F: FnOnce(Pushr) -> Pushr>(&self, f: F) -> &Self {
        self.pushr_reg().set(f);
        self
    }

    #[doc="Modify the PUSHR register."]
    #[inline] pub fn with_pushr<F: FnOnce(Pushr) -> Pushr>(&self, f: F) -> &Self {
        self.pushr_reg().with(f);
        self
    }

    #[doc="Get the PUSHR_SLAVE Register."]
    #[inline] pub fn pushr_slave_reg(&self) -> Register<PushrSlave> { 
        Register::new(self.0 as *mut PushrSlave, 0x34)
    }

    #[doc="Get the *mut pointer for the PUSHR_SLAVE register."]
    #[inline] pub fn pushr_slave_mut(&self) -> *mut PushrSlave { 
        self.pushr_slave_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUSHR_SLAVE register."]
    #[inline] pub fn pushr_slave_ptr(&self) -> *const PushrSlave { 
        self.pushr_slave_reg().ptr()
    }

    #[doc="Read the PUSHR_SLAVE register."]
    #[inline] pub fn pushr_slave(&self) -> PushrSlave { 
        self.pushr_slave_reg().read()
    }

    #[doc="Write the PUSHR_SLAVE register."]
    #[inline] pub fn write_pushr_slave(&self, value: PushrSlave) -> &Self { 
        self.pushr_slave_reg().write(value);
        self
    }

    #[doc="Set the PUSHR_SLAVE register."]
    #[inline] pub fn set_pushr_slave<F: FnOnce(PushrSlave) -> PushrSlave>(&self, f: F) -> &Self {
        self.pushr_slave_reg().set(f);
        self
    }

    #[doc="Modify the PUSHR_SLAVE register."]
    #[inline] pub fn with_pushr_slave<F: FnOnce(PushrSlave) -> PushrSlave>(&self, f: F) -> &Self {
        self.pushr_slave_reg().with(f);
        self
    }

    #[doc="Get the POPR Register."]
    #[inline] pub fn popr_reg(&self) -> Register<Popr> { 
        Register::new(self.0 as *mut Popr, 0x38)
    }

    #[doc="Get the *mut pointer for the POPR register."]
    #[inline] pub fn popr_mut(&self) -> *mut Popr { 
        self.popr_reg().ptr()
    }

    #[doc="Get the *const pointer for the POPR register."]
    #[inline] pub fn popr_ptr(&self) -> *const Popr { 
        self.popr_reg().ptr()
    }

    #[doc="Read the POPR register."]
    #[inline] pub fn popr(&self) -> Popr { 
        self.popr_reg().read()
    }

    #[doc="Get the TXFR Register."]
    #[inline] pub fn txfr_reg(&self) -> RegisterArray<Txfr, bits::R4> { 
        RegisterArray::new(self.0 as *mut Txfr, 0x3c, 0x4)
    }

    #[doc="Get the *mut pointer for the TXFR register."]
    #[inline] pub fn txfr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Txfr { 
        self.txfr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the TXFR register."]
    #[inline] pub fn txfr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Txfr { 
        self.txfr_reg().ptr(index.into())
    }

    #[doc="Read the TXFR register."]
    #[inline] pub fn txfr<I: Into<bits::R4>>(&self, index: I) -> Txfr { 
        self.txfr_reg().read(index.into())
    }

    #[doc="Get the RXFR Register."]
    #[inline] pub fn rxfr_reg(&self) -> RegisterArray<Rxfr, bits::R4> { 
        RegisterArray::new(self.0 as *mut Rxfr, 0x7c, 0x4)
    }

    #[doc="Get the *mut pointer for the RXFR register."]
    #[inline] pub fn rxfr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Rxfr { 
        self.rxfr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the RXFR register."]
    #[inline] pub fn rxfr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Rxfr { 
        self.rxfr_reg().ptr(index.into())
    }

    #[doc="Read the RXFR register."]
    #[inline] pub fn rxfr<I: Into<bits::R4>>(&self, index: I) -> Rxfr { 
        self.rxfr_reg().read(index.into())
    }

}

#[doc="Module Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="Halt"]
    #[inline] pub fn halt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sample Point"]
    #[inline] pub fn smpl_pt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if SMPL_PT != 0"]
    #[inline] pub fn test_smpl_pt(&self) -> bool {
        self.smpl_pt() != 0
    }

    #[doc="Sets the SMPL_PT field."]
    #[inline] pub fn set_smpl_pt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flushes the RX FIFO"]
    #[inline] pub fn clr_rxf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CLR_RXF != 0"]
    #[inline] pub fn test_clr_rxf(&self) -> bool {
        self.clr_rxf() != 0
    }

    #[doc="Sets the CLR_RXF field."]
    #[inline] pub fn set_clr_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clear TX FIFO"]
    #[inline] pub fn clr_txf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLR_TXF != 0"]
    #[inline] pub fn test_clr_txf(&self) -> bool {
        self.clr_txf() != 0
    }

    #[doc="Sets the CLR_TXF field."]
    #[inline] pub fn set_clr_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Disable Receive FIFO"]
    #[inline] pub fn dis_rxf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DIS_RXF != 0"]
    #[inline] pub fn test_dis_rxf(&self) -> bool {
        self.dis_rxf() != 0
    }

    #[doc="Sets the DIS_RXF field."]
    #[inline] pub fn set_dis_rxf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Disable Transmit FIFO"]
    #[inline] pub fn dis_txf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DIS_TXF != 0"]
    #[inline] pub fn test_dis_txf(&self) -> bool {
        self.dis_txf() != 0
    }

    #[doc="Sets the DIS_TXF field."]
    #[inline] pub fn set_dis_txf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Module Disable"]
    #[inline] pub fn mdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if MDIS != 0"]
    #[inline] pub fn test_mdis(&self) -> bool {
        self.mdis() != 0
    }

    #[doc="Sets the MDIS field."]
    #[inline] pub fn set_mdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Doze Enable"]
    #[inline] pub fn doze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DOZE != 0"]
    #[inline] pub fn test_doze(&self) -> bool {
        self.doze() != 0
    }

    #[doc="Sets the DOZE field."]
    #[inline] pub fn set_doze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Peripheral Chip Select x Inactive State"]
    #[inline] pub fn pcsis(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if PCSIS != 0"]
    #[inline] pub fn test_pcsis(&self) -> bool {
        self.pcsis() != 0
    }

    #[doc="Sets the PCSIS field."]
    #[inline] pub fn set_pcsis<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive FIFO Overflow Overwrite Enable"]
    #[inline] pub fn rooe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ROOE != 0"]
    #[inline] pub fn test_rooe(&self) -> bool {
        self.rooe() != 0
    }

    #[doc="Sets the ROOE field."]
    #[inline] pub fn set_rooe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Peripheral Chip Select Strobe Enable"]
    #[inline] pub fn pcsse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PCSSE != 0"]
    #[inline] pub fn test_pcsse(&self) -> bool {
        self.pcsse() != 0
    }

    #[doc="Sets the PCSSE field."]
    #[inline] pub fn set_pcsse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Modified Timing Format Enable"]
    #[inline] pub fn mtfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if MTFE != 0"]
    #[inline] pub fn test_mtfe(&self) -> bool {
        self.mtfe() != 0
    }

    #[doc="Sets the MTFE field."]
    #[inline] pub fn set_mtfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Freeze"]
    #[inline] pub fn frz(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FRZ != 0"]
    #[inline] pub fn test_frz(&self) -> bool {
        self.frz() != 0
    }

    #[doc="Sets the FRZ field."]
    #[inline] pub fn set_frz<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="SPI Configuration."]
    #[inline] pub fn dconf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if DCONF != 0"]
    #[inline] pub fn test_dconf(&self) -> bool {
        self.dconf() != 0
    }

    #[doc="Sets the DCONF field."]
    #[inline] pub fn set_dconf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Continuous SCK Enable"]
    #[inline] pub fn cont_scke(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CONT_SCKE != 0"]
    #[inline] pub fn test_cont_scke(&self) -> bool {
        self.cont_scke() != 0
    }

    #[doc="Sets the CONT_SCKE field."]
    #[inline] pub fn set_cont_scke<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Master/Slave Mode Select"]
    #[inline] pub fn mstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if MSTR != 0"]
    #[inline] pub fn test_mstr(&self) -> bool {
        self.mstr() != 0
    }

    #[doc="Sets the MSTR field."]
    #[inline] pub fn set_mstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
    }
}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.halt() != 0 { try!(write!(f, " halt"))}
        if self.smpl_pt() != 0 { try!(write!(f, " smpl_pt=0x{:x}", self.smpl_pt()))}
        if self.clr_rxf() != 0 { try!(write!(f, " clr_rxf"))}
        if self.clr_txf() != 0 { try!(write!(f, " clr_txf"))}
        if self.dis_rxf() != 0 { try!(write!(f, " dis_rxf"))}
        if self.dis_txf() != 0 { try!(write!(f, " dis_txf"))}
        if self.mdis() != 0 { try!(write!(f, " mdis"))}
        if self.doze() != 0 { try!(write!(f, " doze"))}
        if self.pcsis() != 0 { try!(write!(f, " pcsis=0x{:x}", self.pcsis()))}
        if self.rooe() != 0 { try!(write!(f, " rooe"))}
        if self.pcsse() != 0 { try!(write!(f, " pcsse"))}
        if self.mtfe() != 0 { try!(write!(f, " mtfe"))}
        if self.frz() != 0 { try!(write!(f, " frz"))}
        if self.dconf() != 0 { try!(write!(f, " dconf=0x{:x}", self.dconf()))}
        if self.cont_scke() != 0 { try!(write!(f, " cont_scke"))}
        if self.mstr() != 0 { try!(write!(f, " mstr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transfer Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc="SPI Transfer Counter"]
    #[inline] pub fn spi_tcnt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if SPI_TCNT != 0"]
    #[inline] pub fn test_spi_tcnt(&self) -> bool {
        self.spi_tcnt() != 0
    }

    #[doc="Sets the SPI_TCNT field."]
    #[inline] pub fn set_spi_tcnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Tcr {
    #[inline]
    fn from(other: u32) -> Self {
         Tcr(other)
    }
}

impl ::core::fmt::Display for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spi_tcnt() != 0 { try!(write!(f, " spi_tcnt=0x{:x}", self.spi_tcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock and Transfer Attributes Register (In Master Mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctar(pub u32);
impl Ctar {
    #[doc="Baud Rate Scaler"]
    #[inline] pub fn br(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br(&self) -> bool {
        self.br() != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Delay After Transfer Scaler"]
    #[inline] pub fn dt(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DT != 0"]
    #[inline] pub fn test_dt(&self) -> bool {
        self.dt() != 0
    }

    #[doc="Sets the DT field."]
    #[inline] pub fn set_dt<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="After SCK Delay Scaler"]
    #[inline] pub fn asc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if ASC != 0"]
    #[inline] pub fn test_asc(&self) -> bool {
        self.asc() != 0
    }

    #[doc="Sets the ASC field."]
    #[inline] pub fn set_asc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PCS to SCK Delay Scaler"]
    #[inline] pub fn cssck(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if CSSCK != 0"]
    #[inline] pub fn test_cssck(&self) -> bool {
        self.cssck() != 0
    }

    #[doc="Sets the CSSCK field."]
    #[inline] pub fn set_cssck<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Baud Rate Prescaler"]
    #[inline] pub fn pbr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PBR != 0"]
    #[inline] pub fn test_pbr(&self) -> bool {
        self.pbr() != 0
    }

    #[doc="Sets the PBR field."]
    #[inline] pub fn set_pbr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Delay after Transfer Prescaler"]
    #[inline] pub fn pdt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if PDT != 0"]
    #[inline] pub fn test_pdt(&self) -> bool {
        self.pdt() != 0
    }

    #[doc="Sets the PDT field."]
    #[inline] pub fn set_pdt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="After SCK Delay Prescaler"]
    #[inline] pub fn pasc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if PASC != 0"]
    #[inline] pub fn test_pasc(&self) -> bool {
        self.pasc() != 0
    }

    #[doc="Sets the PASC field."]
    #[inline] pub fn set_pasc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="PCS to SCK Delay Prescaler"]
    #[inline] pub fn pcssck(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if PCSSCK != 0"]
    #[inline] pub fn test_pcssck(&self) -> bool {
        self.pcssck() != 0
    }

    #[doc="Sets the PCSSCK field."]
    #[inline] pub fn set_pcssck<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="LSB First"]
    #[inline] pub fn lsbfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if LSBFE != 0"]
    #[inline] pub fn test_lsbfe(&self) -> bool {
        self.lsbfe() != 0
    }

    #[doc="Sets the LSBFE field."]
    #[inline] pub fn set_lsbfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Frame Size"]
    #[inline] pub fn fmsz(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0xf) as u8) } // [30:27]
    }

    #[doc="Returns true if FMSZ != 0"]
    #[inline] pub fn test_fmsz(&self) -> bool {
        self.fmsz() != 0
    }

    #[doc="Sets the FMSZ field."]
    #[inline] pub fn set_fmsz<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Double Baud Rate"]
    #[inline] pub fn dbr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DBR != 0"]
    #[inline] pub fn test_dbr(&self) -> bool {
        self.dbr() != 0
    }

    #[doc="Sets the DBR field."]
    #[inline] pub fn set_dbr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ctar {
    #[inline]
    fn from(other: u32) -> Self {
         Ctar(other)
    }
}

impl ::core::fmt::Display for Ctar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
        if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
        if self.asc() != 0 { try!(write!(f, " asc=0x{:x}", self.asc()))}
        if self.cssck() != 0 { try!(write!(f, " cssck=0x{:x}", self.cssck()))}
        if self.pbr() != 0 { try!(write!(f, " pbr=0x{:x}", self.pbr()))}
        if self.pdt() != 0 { try!(write!(f, " pdt=0x{:x}", self.pdt()))}
        if self.pasc() != 0 { try!(write!(f, " pasc=0x{:x}", self.pasc()))}
        if self.pcssck() != 0 { try!(write!(f, " pcssck=0x{:x}", self.pcssck()))}
        if self.lsbfe() != 0 { try!(write!(f, " lsbfe"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
        if self.dbr() != 0 { try!(write!(f, " dbr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock and Transfer Attributes Register (In Slave Mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CtarSlave(pub u32);
impl CtarSlave {
    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Frame Size"]
    #[inline] pub fn fmsz(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1f) as u8) } // [31:27]
    }

    #[doc="Returns true if FMSZ != 0"]
    #[inline] pub fn test_fmsz(&self) -> bool {
        self.fmsz() != 0
    }

    #[doc="Sets the FMSZ field."]
    #[inline] pub fn set_fmsz<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for CtarSlave {
    #[inline]
    fn from(other: u32) -> Self {
         CtarSlave(other)
    }
}

impl ::core::fmt::Display for CtarSlave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CtarSlave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Pop Next Pointer"]
    #[inline] pub fn popnxtptr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if POPNXTPTR != 0"]
    #[inline] pub fn test_popnxtptr(&self) -> bool {
        self.popnxtptr() != 0
    }

    #[doc="Sets the POPNXTPTR field."]
    #[inline] pub fn set_popnxtptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX FIFO Counter"]
    #[inline] pub fn rxctr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if RXCTR != 0"]
    #[inline] pub fn test_rxctr(&self) -> bool {
        self.rxctr() != 0
    }

    #[doc="Sets the RXCTR field."]
    #[inline] pub fn set_rxctr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Next Pointer"]
    #[inline] pub fn txnxtptr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if TXNXTPTR != 0"]
    #[inline] pub fn test_txnxtptr(&self) -> bool {
        self.txnxtptr() != 0
    }

    #[doc="Sets the TXNXTPTR field."]
    #[inline] pub fn set_txnxtptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TX FIFO Counter"]
    #[inline] pub fn txctr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if TXCTR != 0"]
    #[inline] pub fn test_txctr(&self) -> bool {
        self.txctr() != 0
    }

    #[doc="Sets the TXCTR field."]
    #[inline] pub fn set_txctr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receive FIFO Drain Flag"]
    #[inline] pub fn rfdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RFDF != 0"]
    #[inline] pub fn test_rfdf(&self) -> bool {
        self.rfdf() != 0
    }

    #[doc="Sets the RFDF field."]
    #[inline] pub fn set_rfdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receive FIFO Overflow Flag"]
    #[inline] pub fn rfof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RFOF != 0"]
    #[inline] pub fn test_rfof(&self) -> bool {
        self.rfof() != 0
    }

    #[doc="Sets the RFOF field."]
    #[inline] pub fn set_rfof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transmit FIFO Fill Flag"]
    #[inline] pub fn tfff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TFFF != 0"]
    #[inline] pub fn test_tfff(&self) -> bool {
        self.tfff() != 0
    }

    #[doc="Sets the TFFF field."]
    #[inline] pub fn set_tfff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Transmit FIFO Underflow Flag"]
    #[inline] pub fn tfuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TFUF != 0"]
    #[inline] pub fn test_tfuf(&self) -> bool {
        self.tfuf() != 0
    }

    #[doc="Sets the TFUF field."]
    #[inline] pub fn set_tfuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="End of Queue Flag"]
    #[inline] pub fn eoqf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if EOQF != 0"]
    #[inline] pub fn test_eoqf(&self) -> bool {
        self.eoqf() != 0
    }

    #[doc="Sets the EOQF field."]
    #[inline] pub fn set_eoqf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="TX and RX Status"]
    #[inline] pub fn txrxs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if TXRXS != 0"]
    #[inline] pub fn test_txrxs(&self) -> bool {
        self.txrxs() != 0
    }

    #[doc="Sets the TXRXS field."]
    #[inline] pub fn set_txrxs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Transfer Complete Flag"]
    #[inline] pub fn tcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TCF != 0"]
    #[inline] pub fn test_tcf(&self) -> bool {
        self.tcf() != 0
    }

    #[doc="Sets the TCF field."]
    #[inline] pub fn set_tcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.popnxtptr() != 0 { try!(write!(f, " popnxtptr=0x{:x}", self.popnxtptr()))}
        if self.rxctr() != 0 { try!(write!(f, " rxctr=0x{:x}", self.rxctr()))}
        if self.txnxtptr() != 0 { try!(write!(f, " txnxtptr=0x{:x}", self.txnxtptr()))}
        if self.txctr() != 0 { try!(write!(f, " txctr=0x{:x}", self.txctr()))}
        if self.rfdf() != 0 { try!(write!(f, " rfdf"))}
        if self.rfof() != 0 { try!(write!(f, " rfof"))}
        if self.tfff() != 0 { try!(write!(f, " tfff"))}
        if self.tfuf() != 0 { try!(write!(f, " tfuf"))}
        if self.eoqf() != 0 { try!(write!(f, " eoqf"))}
        if self.txrxs() != 0 { try!(write!(f, " txrxs"))}
        if self.tcf() != 0 { try!(write!(f, " tcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA/Interrupt Request Select and Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rser(pub u32);
impl Rser {
    #[doc="Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline] pub fn rfdf_dirs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RFDF_DIRS != 0"]
    #[inline] pub fn test_rfdf_dirs(&self) -> bool {
        self.rfdf_dirs() != 0
    }

    #[doc="Sets the RFDF_DIRS field."]
    #[inline] pub fn set_rfdf_dirs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive FIFO Drain Request Enable"]
    #[inline] pub fn rfdf_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RFDF_RE != 0"]
    #[inline] pub fn test_rfdf_re(&self) -> bool {
        self.rfdf_re() != 0
    }

    #[doc="Sets the RFDF_RE field."]
    #[inline] pub fn set_rfdf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receive FIFO Overflow Request Enable"]
    #[inline] pub fn rfof_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RFOF_RE != 0"]
    #[inline] pub fn test_rfof_re(&self) -> bool {
        self.rfof_re() != 0
    }

    #[doc="Sets the RFOF_RE field."]
    #[inline] pub fn set_rfof_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline] pub fn tfff_dirs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TFFF_DIRS != 0"]
    #[inline] pub fn test_tfff_dirs(&self) -> bool {
        self.tfff_dirs() != 0
    }

    #[doc="Sets the TFFF_DIRS field."]
    #[inline] pub fn set_tfff_dirs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Transmit FIFO Fill Request Enable"]
    #[inline] pub fn tfff_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TFFF_RE != 0"]
    #[inline] pub fn test_tfff_re(&self) -> bool {
        self.tfff_re() != 0
    }

    #[doc="Sets the TFFF_RE field."]
    #[inline] pub fn set_tfff_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Transmit FIFO Underflow Request Enable"]
    #[inline] pub fn tfuf_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TFUF_RE != 0"]
    #[inline] pub fn test_tfuf_re(&self) -> bool {
        self.tfuf_re() != 0
    }

    #[doc="Sets the TFUF_RE field."]
    #[inline] pub fn set_tfuf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Finished Request Enable"]
    #[inline] pub fn eoqf_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if EOQF_RE != 0"]
    #[inline] pub fn test_eoqf_re(&self) -> bool {
        self.eoqf_re() != 0
    }

    #[doc="Sets the EOQF_RE field."]
    #[inline] pub fn set_eoqf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Transmission Complete Request Enable"]
    #[inline] pub fn tcf_re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TCF_RE != 0"]
    #[inline] pub fn test_tcf_re(&self) -> bool {
        self.tcf_re() != 0
    }

    #[doc="Sets the TCF_RE field."]
    #[inline] pub fn set_tcf_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Rser {
    #[inline]
    fn from(other: u32) -> Self {
         Rser(other)
    }
}

impl ::core::fmt::Display for Rser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfdf_dirs() != 0 { try!(write!(f, " rfdf_dirs"))}
        if self.rfdf_re() != 0 { try!(write!(f, " rfdf_re"))}
        if self.rfof_re() != 0 { try!(write!(f, " rfof_re"))}
        if self.tfff_dirs() != 0 { try!(write!(f, " tfff_dirs"))}
        if self.tfff_re() != 0 { try!(write!(f, " tfff_re"))}
        if self.tfuf_re() != 0 { try!(write!(f, " tfuf_re"))}
        if self.eoqf_re() != 0 { try!(write!(f, " eoqf_re"))}
        if self.tcf_re() != 0 { try!(write!(f, " tcf_re"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PUSH TX FIFO Register In Master Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pushr(pub u32);
impl Pushr {
    #[doc="Transmit Data"]
    #[inline] pub fn txdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Select which PCS signals are to be asserted for the transfer"]
    #[inline] pub fn pcs(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if PCS != 0"]
    #[inline] pub fn test_pcs(&self) -> bool {
        self.pcs() != 0
    }

    #[doc="Sets the PCS field."]
    #[inline] pub fn set_pcs<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clear Transfer Counter"]
    #[inline] pub fn ctcnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CTCNT != 0"]
    #[inline] pub fn test_ctcnt(&self) -> bool {
        self.ctcnt() != 0
    }

    #[doc="Sets the CTCNT field."]
    #[inline] pub fn set_ctcnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="End Of Queue"]
    #[inline] pub fn eoq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if EOQ != 0"]
    #[inline] pub fn test_eoq(&self) -> bool {
        self.eoq() != 0
    }

    #[doc="Sets the EOQ field."]
    #[inline] pub fn set_eoq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Clock and Transfer Attributes Select"]
    #[inline] pub fn ctas(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if CTAS != 0"]
    #[inline] pub fn test_ctas(&self) -> bool {
        self.ctas() != 0
    }

    #[doc="Sets the CTAS field."]
    #[inline] pub fn set_ctas<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Continuous Peripheral Chip Select Enable"]
    #[inline] pub fn cont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CONT != 0"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Sets the CONT field."]
    #[inline] pub fn set_cont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pushr {
    #[inline]
    fn from(other: u32) -> Self {
         Pushr(other)
    }
}

impl ::core::fmt::Display for Pushr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pushr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
        if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
        if self.ctcnt() != 0 { try!(write!(f, " ctcnt"))}
        if self.eoq() != 0 { try!(write!(f, " eoq"))}
        if self.ctas() != 0 { try!(write!(f, " ctas=0x{:x}", self.ctas()))}
        if self.cont() != 0 { try!(write!(f, " cont"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="PUSH TX FIFO Register In Slave Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PushrSlave(pub u32);
impl PushrSlave {
    #[doc="Transmit Data"]
    #[inline] pub fn txdata(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for PushrSlave {
    #[inline]
    fn from(other: u32) -> Self {
         PushrSlave(other)
    }
}

impl ::core::fmt::Display for PushrSlave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PushrSlave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="POP RX FIFO Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Popr(pub u32);
impl Popr {
    #[doc="Received Data"]
    #[inline] pub fn rxdata(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Popr {
    #[inline]
    fn from(other: u32) -> Self {
         Popr(other)
    }
}

impl ::core::fmt::Display for Popr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Popr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit FIFO Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txfr(pub u32);
impl Txfr {
    #[doc="Transmit Data"]
    #[inline] pub fn txdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TXDATA != 0"]
    #[inline] pub fn test_txdata(&self) -> bool {
        self.txdata() != 0
    }

    #[doc="Sets the TXDATA field."]
    #[inline] pub fn set_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Command or Transmit Data"]
    #[inline] pub fn txcmd_txdata(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TXCMD_TXDATA != 0"]
    #[inline] pub fn test_txcmd_txdata(&self) -> bool {
        self.txcmd_txdata() != 0
    }

    #[doc="Sets the TXCMD_TXDATA field."]
    #[inline] pub fn set_txcmd_txdata<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Txfr {
    #[inline]
    fn from(other: u32) -> Self {
         Txfr(other)
    }
}

impl ::core::fmt::Display for Txfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
        if self.txcmd_txdata() != 0 { try!(write!(f, " txcmd_txdata=0x{:x}", self.txcmd_txdata()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive FIFO Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxfr(pub u32);
impl Rxfr {
    #[doc="Receive Data"]
    #[inline] pub fn rxdata(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RXDATA != 0"]
    #[inline] pub fn test_rxdata(&self) -> bool {
        self.rxdata() != 0
    }

    #[doc="Sets the RXDATA field."]
    #[inline] pub fn set_rxdata<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rxfr {
    #[inline]
    fn from(other: u32) -> Self {
         Rxfr(other)
    }
}

impl ::core::fmt::Display for Rxfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

