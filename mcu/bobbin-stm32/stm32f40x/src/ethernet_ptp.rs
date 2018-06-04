
::bobbin_mcu::periph!( ETHERNET_PTP, EthernetPtp, ETHERNET_PTP_PERIPH, EthernetPtpPeriph, ETHERNET_PTP_OWNED, ETHERNET_PTP_REF_COUNT, 0x40028700, 0x00, 0x06);


#[doc="Ethernet: Precision time protocol"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EthernetPtpPeriph(pub usize);
impl EthernetPtpPeriph {
    #[doc="Get the PTPTSCR Register."]
    #[inline] pub fn ptptscr_reg(&self) -> ::bobbin_mcu::register::Register<Ptptscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptscr, 0x0)
    }

    #[doc="Get the *mut pointer for the PTPTSCR register."]
    #[inline] pub fn ptptscr_mut(&self) -> *mut Ptptscr { 
        self.ptptscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSCR register."]
    #[inline] pub fn ptptscr_ptr(&self) -> *const Ptptscr { 
        self.ptptscr_reg().ptr()
    }

    #[doc="Read the PTPTSCR register."]
    #[inline] pub fn ptptscr(&self) -> Ptptscr { 
        self.ptptscr_reg().read()
    }

    #[doc="Write the PTPTSCR register."]
    #[inline] pub fn write_ptptscr(&self, value: Ptptscr) -> &Self { 
        self.ptptscr_reg().write(value);
        self
    }

    #[doc="Set the PTPTSCR register."]
    #[inline] pub fn set_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&self, f: F) -> &Self {
        self.ptptscr_reg().set(f);
        self
    }

    #[doc="Modify the PTPTSCR register."]
    #[inline] pub fn with_ptptscr<F: FnOnce(Ptptscr) -> Ptptscr>(&self, f: F) -> &Self {
        self.ptptscr_reg().with(f);
        self
    }

    #[doc="Get the PTPSSIR Register."]
    #[inline] pub fn ptpssir_reg(&self) -> ::bobbin_mcu::register::Register<Ptpssir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptpssir, 0x4)
    }

    #[doc="Get the *mut pointer for the PTPSSIR register."]
    #[inline] pub fn ptpssir_mut(&self) -> *mut Ptpssir { 
        self.ptpssir_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPSSIR register."]
    #[inline] pub fn ptpssir_ptr(&self) -> *const Ptpssir { 
        self.ptpssir_reg().ptr()
    }

    #[doc="Read the PTPSSIR register."]
    #[inline] pub fn ptpssir(&self) -> Ptpssir { 
        self.ptpssir_reg().read()
    }

    #[doc="Write the PTPSSIR register."]
    #[inline] pub fn write_ptpssir(&self, value: Ptpssir) -> &Self { 
        self.ptpssir_reg().write(value);
        self
    }

    #[doc="Set the PTPSSIR register."]
    #[inline] pub fn set_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&self, f: F) -> &Self {
        self.ptpssir_reg().set(f);
        self
    }

    #[doc="Modify the PTPSSIR register."]
    #[inline] pub fn with_ptpssir<F: FnOnce(Ptpssir) -> Ptpssir>(&self, f: F) -> &Self {
        self.ptpssir_reg().with(f);
        self
    }

    #[doc="Get the PTPTSHR Register."]
    #[inline] pub fn ptptshr_reg(&self) -> ::bobbin_mcu::register::Register<Ptptshr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptshr, 0x8)
    }

    #[doc="Get the *mut pointer for the PTPTSHR register."]
    #[inline] pub fn ptptshr_mut(&self) -> *mut Ptptshr { 
        self.ptptshr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSHR register."]
    #[inline] pub fn ptptshr_ptr(&self) -> *const Ptptshr { 
        self.ptptshr_reg().ptr()
    }

    #[doc="Read the PTPTSHR register."]
    #[inline] pub fn ptptshr(&self) -> Ptptshr { 
        self.ptptshr_reg().read()
    }

    #[doc="Get the PTPTSLR Register."]
    #[inline] pub fn ptptslr_reg(&self) -> ::bobbin_mcu::register::Register<Ptptslr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptslr, 0xc)
    }

    #[doc="Get the *mut pointer for the PTPTSLR register."]
    #[inline] pub fn ptptslr_mut(&self) -> *mut Ptptslr { 
        self.ptptslr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSLR register."]
    #[inline] pub fn ptptslr_ptr(&self) -> *const Ptptslr { 
        self.ptptslr_reg().ptr()
    }

    #[doc="Read the PTPTSLR register."]
    #[inline] pub fn ptptslr(&self) -> Ptptslr { 
        self.ptptslr_reg().read()
    }

    #[doc="Get the PTPTSHUR Register."]
    #[inline] pub fn ptptshur_reg(&self) -> ::bobbin_mcu::register::Register<Ptptshur> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptshur, 0x10)
    }

    #[doc="Get the *mut pointer for the PTPTSHUR register."]
    #[inline] pub fn ptptshur_mut(&self) -> *mut Ptptshur { 
        self.ptptshur_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSHUR register."]
    #[inline] pub fn ptptshur_ptr(&self) -> *const Ptptshur { 
        self.ptptshur_reg().ptr()
    }

    #[doc="Read the PTPTSHUR register."]
    #[inline] pub fn ptptshur(&self) -> Ptptshur { 
        self.ptptshur_reg().read()
    }

    #[doc="Write the PTPTSHUR register."]
    #[inline] pub fn write_ptptshur(&self, value: Ptptshur) -> &Self { 
        self.ptptshur_reg().write(value);
        self
    }

    #[doc="Set the PTPTSHUR register."]
    #[inline] pub fn set_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&self, f: F) -> &Self {
        self.ptptshur_reg().set(f);
        self
    }

    #[doc="Modify the PTPTSHUR register."]
    #[inline] pub fn with_ptptshur<F: FnOnce(Ptptshur) -> Ptptshur>(&self, f: F) -> &Self {
        self.ptptshur_reg().with(f);
        self
    }

    #[doc="Get the PTPTSLUR Register."]
    #[inline] pub fn ptptslur_reg(&self) -> ::bobbin_mcu::register::Register<Ptptslur> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptslur, 0x14)
    }

    #[doc="Get the *mut pointer for the PTPTSLUR register."]
    #[inline] pub fn ptptslur_mut(&self) -> *mut Ptptslur { 
        self.ptptslur_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSLUR register."]
    #[inline] pub fn ptptslur_ptr(&self) -> *const Ptptslur { 
        self.ptptslur_reg().ptr()
    }

    #[doc="Read the PTPTSLUR register."]
    #[inline] pub fn ptptslur(&self) -> Ptptslur { 
        self.ptptslur_reg().read()
    }

    #[doc="Write the PTPTSLUR register."]
    #[inline] pub fn write_ptptslur(&self, value: Ptptslur) -> &Self { 
        self.ptptslur_reg().write(value);
        self
    }

    #[doc="Set the PTPTSLUR register."]
    #[inline] pub fn set_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&self, f: F) -> &Self {
        self.ptptslur_reg().set(f);
        self
    }

    #[doc="Modify the PTPTSLUR register."]
    #[inline] pub fn with_ptptslur<F: FnOnce(Ptptslur) -> Ptptslur>(&self, f: F) -> &Self {
        self.ptptslur_reg().with(f);
        self
    }

    #[doc="Get the PTPTSAR Register."]
    #[inline] pub fn ptptsar_reg(&self) -> ::bobbin_mcu::register::Register<Ptptsar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptsar, 0x18)
    }

    #[doc="Get the *mut pointer for the PTPTSAR register."]
    #[inline] pub fn ptptsar_mut(&self) -> *mut Ptptsar { 
        self.ptptsar_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSAR register."]
    #[inline] pub fn ptptsar_ptr(&self) -> *const Ptptsar { 
        self.ptptsar_reg().ptr()
    }

    #[doc="Read the PTPTSAR register."]
    #[inline] pub fn ptptsar(&self) -> Ptptsar { 
        self.ptptsar_reg().read()
    }

    #[doc="Write the PTPTSAR register."]
    #[inline] pub fn write_ptptsar(&self, value: Ptptsar) -> &Self { 
        self.ptptsar_reg().write(value);
        self
    }

    #[doc="Set the PTPTSAR register."]
    #[inline] pub fn set_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&self, f: F) -> &Self {
        self.ptptsar_reg().set(f);
        self
    }

    #[doc="Modify the PTPTSAR register."]
    #[inline] pub fn with_ptptsar<F: FnOnce(Ptptsar) -> Ptptsar>(&self, f: F) -> &Self {
        self.ptptsar_reg().with(f);
        self
    }

    #[doc="Get the PTPTTHR Register."]
    #[inline] pub fn ptptthr_reg(&self) -> ::bobbin_mcu::register::Register<Ptptthr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptthr, 0x1c)
    }

    #[doc="Get the *mut pointer for the PTPTTHR register."]
    #[inline] pub fn ptptthr_mut(&self) -> *mut Ptptthr { 
        self.ptptthr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTTHR register."]
    #[inline] pub fn ptptthr_ptr(&self) -> *const Ptptthr { 
        self.ptptthr_reg().ptr()
    }

    #[doc="Read the PTPTTHR register."]
    #[inline] pub fn ptptthr(&self) -> Ptptthr { 
        self.ptptthr_reg().read()
    }

    #[doc="Write the PTPTTHR register."]
    #[inline] pub fn write_ptptthr(&self, value: Ptptthr) -> &Self { 
        self.ptptthr_reg().write(value);
        self
    }

    #[doc="Set the PTPTTHR register."]
    #[inline] pub fn set_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&self, f: F) -> &Self {
        self.ptptthr_reg().set(f);
        self
    }

    #[doc="Modify the PTPTTHR register."]
    #[inline] pub fn with_ptptthr<F: FnOnce(Ptptthr) -> Ptptthr>(&self, f: F) -> &Self {
        self.ptptthr_reg().with(f);
        self
    }

    #[doc="Get the PTPTTLR Register."]
    #[inline] pub fn ptpttlr_reg(&self) -> ::bobbin_mcu::register::Register<Ptpttlr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptpttlr, 0x20)
    }

    #[doc="Get the *mut pointer for the PTPTTLR register."]
    #[inline] pub fn ptpttlr_mut(&self) -> *mut Ptpttlr { 
        self.ptpttlr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTTLR register."]
    #[inline] pub fn ptpttlr_ptr(&self) -> *const Ptpttlr { 
        self.ptpttlr_reg().ptr()
    }

    #[doc="Read the PTPTTLR register."]
    #[inline] pub fn ptpttlr(&self) -> Ptpttlr { 
        self.ptpttlr_reg().read()
    }

    #[doc="Write the PTPTTLR register."]
    #[inline] pub fn write_ptpttlr(&self, value: Ptpttlr) -> &Self { 
        self.ptpttlr_reg().write(value);
        self
    }

    #[doc="Set the PTPTTLR register."]
    #[inline] pub fn set_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&self, f: F) -> &Self {
        self.ptpttlr_reg().set(f);
        self
    }

    #[doc="Modify the PTPTTLR register."]
    #[inline] pub fn with_ptpttlr<F: FnOnce(Ptpttlr) -> Ptpttlr>(&self, f: F) -> &Self {
        self.ptpttlr_reg().with(f);
        self
    }

    #[doc="Get the PTPTSSR Register."]
    #[inline] pub fn ptptssr_reg(&self) -> ::bobbin_mcu::register::Register<Ptptssr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptptssr, 0x28)
    }

    #[doc="Get the *mut pointer for the PTPTSSR register."]
    #[inline] pub fn ptptssr_mut(&self) -> *mut Ptptssr { 
        self.ptptssr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPTSSR register."]
    #[inline] pub fn ptptssr_ptr(&self) -> *const Ptptssr { 
        self.ptptssr_reg().ptr()
    }

    #[doc="Read the PTPTSSR register."]
    #[inline] pub fn ptptssr(&self) -> Ptptssr { 
        self.ptptssr_reg().read()
    }

    #[doc="Get the PTPPPSCR Register."]
    #[inline] pub fn ptpppscr_reg(&self) -> ::bobbin_mcu::register::Register<Ptpppscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptpppscr, 0x2c)
    }

    #[doc="Get the *mut pointer for the PTPPPSCR register."]
    #[inline] pub fn ptpppscr_mut(&self) -> *mut Ptpppscr { 
        self.ptpppscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTPPPSCR register."]
    #[inline] pub fn ptpppscr_ptr(&self) -> *const Ptpppscr { 
        self.ptpppscr_reg().ptr()
    }

    #[doc="Read the PTPPPSCR register."]
    #[inline] pub fn ptpppscr(&self) -> Ptpppscr { 
        self.ptpppscr_reg().read()
    }

}

#[doc="Ethernet PTP time stamp control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptscr(pub u32);
impl Ptptscr {
    #[doc="no description available"]
    #[inline] pub fn tse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TSE != 0"]
    #[inline] pub fn test_tse(&self) -> bool {
        self.tse() != 0
    }

    #[doc="Sets the TSE field."]
    #[inline] pub fn set_tse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsfcu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSFCU != 0"]
    #[inline] pub fn test_tsfcu(&self) -> bool {
        self.tsfcu() != 0
    }

    #[doc="Sets the TSFCU field."]
    #[inline] pub fn set_tsfcu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsptppsv2e(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TSPTPPSV2E != 0"]
    #[inline] pub fn test_tsptppsv2e(&self) -> bool {
        self.tsptppsv2e() != 0
    }

    #[doc="Sets the TSPTPPSV2E field."]
    #[inline] pub fn set_tsptppsv2e<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssptpoefe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TSSPTPOEFE != 0"]
    #[inline] pub fn test_tssptpoefe(&self) -> bool {
        self.tssptpoefe() != 0
    }

    #[doc="Sets the TSSPTPOEFE field."]
    #[inline] pub fn set_tssptpoefe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssipv6fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TSSIPV6FE != 0"]
    #[inline] pub fn test_tssipv6fe(&self) -> bool {
        self.tssipv6fe() != 0
    }

    #[doc="Sets the TSSIPV6FE field."]
    #[inline] pub fn set_tssipv6fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssipv4fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TSSIPV4FE != 0"]
    #[inline] pub fn test_tssipv4fe(&self) -> bool {
        self.tssipv4fe() != 0
    }

    #[doc="Sets the TSSIPV4FE field."]
    #[inline] pub fn set_tssipv4fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsseme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TSSEME != 0"]
    #[inline] pub fn test_tsseme(&self) -> bool {
        self.tsseme() != 0
    }

    #[doc="Sets the TSSEME field."]
    #[inline] pub fn set_tsseme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssmrme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TSSMRME != 0"]
    #[inline] pub fn test_tssmrme(&self) -> bool {
        self.tssmrme() != 0
    }

    #[doc="Sets the TSSMRME field."]
    #[inline] pub fn set_tssmrme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tscnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if TSCNT != 0"]
    #[inline] pub fn test_tscnt(&self) -> bool {
        self.tscnt() != 0
    }

    #[doc="Sets the TSCNT field."]
    #[inline] pub fn set_tscnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tspffmae(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TSPFFMAE != 0"]
    #[inline] pub fn test_tspffmae(&self) -> bool {
        self.tspffmae() != 0
    }

    #[doc="Sets the TSPFFMAE field."]
    #[inline] pub fn set_tspffmae<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssti(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TSSTI != 0"]
    #[inline] pub fn test_tssti(&self) -> bool {
        self.tssti() != 0
    }

    #[doc="Sets the TSSTI field."]
    #[inline] pub fn set_tssti<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsstu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TSSTU != 0"]
    #[inline] pub fn test_tsstu(&self) -> bool {
        self.tsstu() != 0
    }

    #[doc="Sets the TSSTU field."]
    #[inline] pub fn set_tsstu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsite(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TSITE != 0"]
    #[inline] pub fn test_tsite(&self) -> bool {
        self.tsite() != 0
    }

    #[doc="Sets the TSITE field."]
    #[inline] pub fn set_tsite<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn ttsaru(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TTSARU != 0"]
    #[inline] pub fn test_ttsaru(&self) -> bool {
        self.ttsaru() != 0
    }

    #[doc="Sets the TTSARU field."]
    #[inline] pub fn set_ttsaru<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tssarfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TSSARFE != 0"]
    #[inline] pub fn test_tssarfe(&self) -> bool {
        self.tssarfe() != 0
    }

    #[doc="Sets the TSSARFE field."]
    #[inline] pub fn set_tssarfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsssr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TSSSR != 0"]
    #[inline] pub fn test_tsssr(&self) -> bool {
        self.tsssr() != 0
    }

    #[doc="Sets the TSSSR field."]
    #[inline] pub fn set_tsssr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Ptptscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptscr(other)
    }
}

impl ::core::fmt::Display for Ptptscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tse() != 0 { try!(write!(f, " tse"))}
        if self.tsfcu() != 0 { try!(write!(f, " tsfcu"))}
        if self.tsptppsv2e() != 0 { try!(write!(f, " tsptppsv2e"))}
        if self.tssptpoefe() != 0 { try!(write!(f, " tssptpoefe"))}
        if self.tssipv6fe() != 0 { try!(write!(f, " tssipv6fe"))}
        if self.tssipv4fe() != 0 { try!(write!(f, " tssipv4fe"))}
        if self.tsseme() != 0 { try!(write!(f, " tsseme"))}
        if self.tssmrme() != 0 { try!(write!(f, " tssmrme"))}
        if self.tscnt() != 0 { try!(write!(f, " tscnt=0x{:x}", self.tscnt()))}
        if self.tspffmae() != 0 { try!(write!(f, " tspffmae"))}
        if self.tssti() != 0 { try!(write!(f, " tssti"))}
        if self.tsstu() != 0 { try!(write!(f, " tsstu"))}
        if self.tsite() != 0 { try!(write!(f, " tsite"))}
        if self.ttsaru() != 0 { try!(write!(f, " ttsaru"))}
        if self.tssarfe() != 0 { try!(write!(f, " tssarfe"))}
        if self.tsssr() != 0 { try!(write!(f, " tsssr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP subsecond increment register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpssir(pub u32);
impl Ptpssir {
    #[doc="no description available"]
    #[inline] pub fn stssi(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if STSSI != 0"]
    #[inline] pub fn test_stssi(&self) -> bool {
        self.stssi() != 0
    }

    #[doc="Sets the STSSI field."]
    #[inline] pub fn set_stssi<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptpssir {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpssir(other)
    }
}

impl ::core::fmt::Display for Ptpssir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpssir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stssi() != 0 { try!(write!(f, " stssi=0x{:x}", self.stssi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptshr(pub u32);
impl Ptptshr {
    #[doc="no description available"]
    #[inline] pub fn sts(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if STS != 0"]
    #[inline] pub fn test_sts(&self) -> bool {
        self.sts() != 0
    }

    #[doc="Sets the STS field."]
    #[inline] pub fn set_sts<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptshr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptshr(other)
    }
}

impl ::core::fmt::Display for Ptptshr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptshr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptslr(pub u32);
impl Ptptslr {
    #[doc="no description available"]
    #[inline] pub fn stss(&self) -> ::bobbin_bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Returns true if STSS != 0"]
    #[inline] pub fn test_stss(&self) -> bool {
        self.stss() != 0
    }

    #[doc="Sets the STSS field."]
    #[inline] pub fn set_stss<V: Into<::bobbin_bits::U31>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn stpns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if STPNS != 0"]
    #[inline] pub fn test_stpns(&self) -> bool {
        self.stpns() != 0
    }

    #[doc="Sets the STPNS field."]
    #[inline] pub fn set_stpns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ptptslr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptslr(other)
    }
}

impl ::core::fmt::Display for Ptptslr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptslr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stss() != 0 { try!(write!(f, " stss=0x{:x}", self.stss()))}
        if self.stpns() != 0 { try!(write!(f, " stpns"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp high update register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptshur(pub u32);
impl Ptptshur {
    #[doc="no description available"]
    #[inline] pub fn tsus(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TSUS != 0"]
    #[inline] pub fn test_tsus(&self) -> bool {
        self.tsus() != 0
    }

    #[doc="Sets the TSUS field."]
    #[inline] pub fn set_tsus<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptshur {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptshur(other)
    }
}

impl ::core::fmt::Display for Ptptshur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptshur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp low update register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptslur(pub u32);
impl Ptptslur {
    #[doc="no description available"]
    #[inline] pub fn tsuss(&self) -> ::bobbin_bits::U31 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7fffffff) as u32) } // [30:0]
    }

    #[doc="Returns true if TSUSS != 0"]
    #[inline] pub fn test_tsuss(&self) -> bool {
        self.tsuss() != 0
    }

    #[doc="Sets the TSUSS field."]
    #[inline] pub fn set_tsuss<V: Into<::bobbin_bits::U31>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U31 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsupns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if TSUPNS != 0"]
    #[inline] pub fn test_tsupns(&self) -> bool {
        self.tsupns() != 0
    }

    #[doc="Sets the TSUPNS field."]
    #[inline] pub fn set_tsupns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ptptslur {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptslur(other)
    }
}

impl ::core::fmt::Display for Ptptslur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptslur {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsuss() != 0 { try!(write!(f, " tsuss=0x{:x}", self.tsuss()))}
        if self.tsupns() != 0 { try!(write!(f, " tsupns"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp addend register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptsar(pub u32);
impl Ptptsar {
    #[doc="no description available"]
    #[inline] pub fn tsa(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TSA != 0"]
    #[inline] pub fn test_tsa(&self) -> bool {
        self.tsa() != 0
    }

    #[doc="Sets the TSA field."]
    #[inline] pub fn set_tsa<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptsar {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptsar(other)
    }
}

impl ::core::fmt::Display for Ptptsar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptsar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP target time high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptthr(pub u32);
impl Ptptthr {
    #[doc="0"]
    #[inline] pub fn ttsh(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TTSH != 0"]
    #[inline] pub fn test_ttsh(&self) -> bool {
        self.ttsh() != 0
    }

    #[doc="Sets the TTSH field."]
    #[inline] pub fn set_ttsh<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptptthr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptthr(other)
    }
}

impl ::core::fmt::Display for Ptptthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptthr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP target time low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpttlr(pub u32);
impl Ptpttlr {
    #[doc="no description available"]
    #[inline] pub fn ttsl(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TTSL != 0"]
    #[inline] pub fn test_ttsl(&self) -> bool {
        self.ttsl() != 0
    }

    #[doc="Sets the TTSL field."]
    #[inline] pub fn set_ttsl<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptpttlr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpttlr(other)
    }
}

impl ::core::fmt::Display for Ptpttlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpttlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP time stamp status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptptssr(pub u32);
impl Ptptssr {
    #[doc="no description available"]
    #[inline] pub fn tsso(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TSSO != 0"]
    #[inline] pub fn test_tsso(&self) -> bool {
        self.tsso() != 0
    }

    #[doc="Sets the TSSO field."]
    #[inline] pub fn set_tsso<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tsttr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSTTR != 0"]
    #[inline] pub fn test_tsttr(&self) -> bool {
        self.tsttr() != 0
    }

    #[doc="Sets the TSTTR field."]
    #[inline] pub fn set_tsttr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ptptssr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptptssr(other)
    }
}

impl ::core::fmt::Display for Ptptssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptptssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsso() != 0 { try!(write!(f, " tsso"))}
        if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Ethernet PTP PPS control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptpppscr(pub u32);
impl Ptpppscr {
    #[doc="TSSO"]
    #[inline] pub fn tsso(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TSSO != 0"]
    #[inline] pub fn test_tsso(&self) -> bool {
        self.tsso() != 0
    }

    #[doc="Sets the TSSO field."]
    #[inline] pub fn set_tsso<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TSTTR"]
    #[inline] pub fn tsttr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSTTR != 0"]
    #[inline] pub fn test_tsttr(&self) -> bool {
        self.tsttr() != 0
    }

    #[doc="Sets the TSTTR field."]
    #[inline] pub fn set_tsttr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Ptpppscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ptpppscr(other)
    }
}

impl ::core::fmt::Display for Ptpppscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptpppscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsso() != 0 { try!(write!(f, " tsso"))}
        if self.tsttr() != 0 { try!(write!(f, " tsttr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

