
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB Peripheral"]
pub struct UsbPeriph(pub usize); 

impl UsbPeriph {
    #[doc="Get the PERID Register."]
    #[inline] pub fn perid_reg(&self) -> Register<Perid> { 
        Register::new(self.0 as *mut Perid, 0x0)
    }

    #[doc="Get the *mut pointer for the PERID register."]
    #[inline] pub fn perid_mut(&self) -> *mut Perid { 
        self.perid_reg().ptr()
    }

    #[doc="Get the *const pointer for the PERID register."]
    #[inline] pub fn perid_ptr(&self) -> *const Perid { 
        self.perid_reg().ptr()
    }

    #[doc="Read the PERID register."]
    #[inline] pub fn perid(&self) -> Perid { 
        self.perid_reg().read()
    }

    #[doc="Get the IDCOMP Register."]
    #[inline] pub fn idcomp_reg(&self) -> Register<Idcomp> { 
        Register::new(self.0 as *mut Idcomp, 0x4)
    }

    #[doc="Get the *mut pointer for the IDCOMP register."]
    #[inline] pub fn idcomp_mut(&self) -> *mut Idcomp { 
        self.idcomp_reg().ptr()
    }

    #[doc="Get the *const pointer for the IDCOMP register."]
    #[inline] pub fn idcomp_ptr(&self) -> *const Idcomp { 
        self.idcomp_reg().ptr()
    }

    #[doc="Read the IDCOMP register."]
    #[inline] pub fn idcomp(&self) -> Idcomp { 
        self.idcomp_reg().read()
    }

    #[doc="Get the REV Register."]
    #[inline] pub fn rev_reg(&self) -> Register<Rev> { 
        Register::new(self.0 as *mut Rev, 0x8)
    }

    #[doc="Get the *mut pointer for the REV register."]
    #[inline] pub fn rev_mut(&self) -> *mut Rev { 
        self.rev_reg().ptr()
    }

    #[doc="Get the *const pointer for the REV register."]
    #[inline] pub fn rev_ptr(&self) -> *const Rev { 
        self.rev_reg().ptr()
    }

    #[doc="Read the REV register."]
    #[inline] pub fn rev(&self) -> Rev { 
        self.rev_reg().read()
    }

    #[doc="Get the ADDINFO Register."]
    #[inline] pub fn addinfo_reg(&self) -> Register<Addinfo> { 
        Register::new(self.0 as *mut Addinfo, 0xc)
    }

    #[doc="Get the *mut pointer for the ADDINFO register."]
    #[inline] pub fn addinfo_mut(&self) -> *mut Addinfo { 
        self.addinfo_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDINFO register."]
    #[inline] pub fn addinfo_ptr(&self) -> *const Addinfo { 
        self.addinfo_reg().ptr()
    }

    #[doc="Read the ADDINFO register."]
    #[inline] pub fn addinfo(&self) -> Addinfo { 
        self.addinfo_reg().read()
    }

    #[doc="Get the OTGISTAT Register."]
    #[inline] pub fn otgistat_reg(&self) -> Register<Otgistat> { 
        Register::new(self.0 as *mut Otgistat, 0x10)
    }

    #[doc="Get the *mut pointer for the OTGISTAT register."]
    #[inline] pub fn otgistat_mut(&self) -> *mut Otgistat { 
        self.otgistat_reg().ptr()
    }

    #[doc="Get the *const pointer for the OTGISTAT register."]
    #[inline] pub fn otgistat_ptr(&self) -> *const Otgistat { 
        self.otgistat_reg().ptr()
    }

    #[doc="Read the OTGISTAT register."]
    #[inline] pub fn otgistat(&self) -> Otgistat { 
        self.otgistat_reg().read()
    }

    #[doc="Write the OTGISTAT register."]
    #[inline] pub fn write_otgistat(&self, value: Otgistat) -> &Self { 
        self.otgistat_reg().write(value);
        self
    }

    #[doc="Set the OTGISTAT register."]
    #[inline] pub fn set_otgistat<F: FnOnce(Otgistat) -> Otgistat>(&self, f: F) -> &Self {
        self.otgistat_reg().set(f);
        self
    }

    #[doc="Modify the OTGISTAT register."]
    #[inline] pub fn with_otgistat<F: FnOnce(Otgistat) -> Otgistat>(&self, f: F) -> &Self {
        self.otgistat_reg().with(f);
        self
    }

    #[doc="Get the OTGICR Register."]
    #[inline] pub fn otgicr_reg(&self) -> Register<Otgicr> { 
        Register::new(self.0 as *mut Otgicr, 0x14)
    }

    #[doc="Get the *mut pointer for the OTGICR register."]
    #[inline] pub fn otgicr_mut(&self) -> *mut Otgicr { 
        self.otgicr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OTGICR register."]
    #[inline] pub fn otgicr_ptr(&self) -> *const Otgicr { 
        self.otgicr_reg().ptr()
    }

    #[doc="Read the OTGICR register."]
    #[inline] pub fn otgicr(&self) -> Otgicr { 
        self.otgicr_reg().read()
    }

    #[doc="Write the OTGICR register."]
    #[inline] pub fn write_otgicr(&self, value: Otgicr) -> &Self { 
        self.otgicr_reg().write(value);
        self
    }

    #[doc="Set the OTGICR register."]
    #[inline] pub fn set_otgicr<F: FnOnce(Otgicr) -> Otgicr>(&self, f: F) -> &Self {
        self.otgicr_reg().set(f);
        self
    }

    #[doc="Modify the OTGICR register."]
    #[inline] pub fn with_otgicr<F: FnOnce(Otgicr) -> Otgicr>(&self, f: F) -> &Self {
        self.otgicr_reg().with(f);
        self
    }

    #[doc="Get the OTGSTAT Register."]
    #[inline] pub fn otgstat_reg(&self) -> Register<Otgstat> { 
        Register::new(self.0 as *mut Otgstat, 0x18)
    }

    #[doc="Get the *mut pointer for the OTGSTAT register."]
    #[inline] pub fn otgstat_mut(&self) -> *mut Otgstat { 
        self.otgstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the OTGSTAT register."]
    #[inline] pub fn otgstat_ptr(&self) -> *const Otgstat { 
        self.otgstat_reg().ptr()
    }

    #[doc="Read the OTGSTAT register."]
    #[inline] pub fn otgstat(&self) -> Otgstat { 
        self.otgstat_reg().read()
    }

    #[doc="Write the OTGSTAT register."]
    #[inline] pub fn write_otgstat(&self, value: Otgstat) -> &Self { 
        self.otgstat_reg().write(value);
        self
    }

    #[doc="Set the OTGSTAT register."]
    #[inline] pub fn set_otgstat<F: FnOnce(Otgstat) -> Otgstat>(&self, f: F) -> &Self {
        self.otgstat_reg().set(f);
        self
    }

    #[doc="Modify the OTGSTAT register."]
    #[inline] pub fn with_otgstat<F: FnOnce(Otgstat) -> Otgstat>(&self, f: F) -> &Self {
        self.otgstat_reg().with(f);
        self
    }

    #[doc="Get the OTGCTL Register."]
    #[inline] pub fn otgctl_reg(&self) -> Register<Otgctl> { 
        Register::new(self.0 as *mut Otgctl, 0x1c)
    }

    #[doc="Get the *mut pointer for the OTGCTL register."]
    #[inline] pub fn otgctl_mut(&self) -> *mut Otgctl { 
        self.otgctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the OTGCTL register."]
    #[inline] pub fn otgctl_ptr(&self) -> *const Otgctl { 
        self.otgctl_reg().ptr()
    }

    #[doc="Read the OTGCTL register."]
    #[inline] pub fn otgctl(&self) -> Otgctl { 
        self.otgctl_reg().read()
    }

    #[doc="Write the OTGCTL register."]
    #[inline] pub fn write_otgctl(&self, value: Otgctl) -> &Self { 
        self.otgctl_reg().write(value);
        self
    }

    #[doc="Set the OTGCTL register."]
    #[inline] pub fn set_otgctl<F: FnOnce(Otgctl) -> Otgctl>(&self, f: F) -> &Self {
        self.otgctl_reg().set(f);
        self
    }

    #[doc="Modify the OTGCTL register."]
    #[inline] pub fn with_otgctl<F: FnOnce(Otgctl) -> Otgctl>(&self, f: F) -> &Self {
        self.otgctl_reg().with(f);
        self
    }

    #[doc="Get the ISTAT Register."]
    #[inline] pub fn istat_reg(&self) -> Register<Istat> { 
        Register::new(self.0 as *mut Istat, 0x80)
    }

    #[doc="Get the *mut pointer for the ISTAT register."]
    #[inline] pub fn istat_mut(&self) -> *mut Istat { 
        self.istat_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISTAT register."]
    #[inline] pub fn istat_ptr(&self) -> *const Istat { 
        self.istat_reg().ptr()
    }

    #[doc="Read the ISTAT register."]
    #[inline] pub fn istat(&self) -> Istat { 
        self.istat_reg().read()
    }

    #[doc="Write the ISTAT register."]
    #[inline] pub fn write_istat(&self, value: Istat) -> &Self { 
        self.istat_reg().write(value);
        self
    }

    #[doc="Set the ISTAT register."]
    #[inline] pub fn set_istat<F: FnOnce(Istat) -> Istat>(&self, f: F) -> &Self {
        self.istat_reg().set(f);
        self
    }

    #[doc="Modify the ISTAT register."]
    #[inline] pub fn with_istat<F: FnOnce(Istat) -> Istat>(&self, f: F) -> &Self {
        self.istat_reg().with(f);
        self
    }

    #[doc="Get the INTEN Register."]
    #[inline] pub fn inten_reg(&self) -> Register<Inten> { 
        Register::new(self.0 as *mut Inten, 0x84)
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        self.inten_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
        self.inten_reg().ptr()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        self.inten_reg().read()
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn write_inten(&self, value: Inten) -> &Self { 
        self.inten_reg().write(value);
        self
    }

    #[doc="Set the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        self.inten_reg().set(f);
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        self.inten_reg().with(f);
        self
    }

    #[doc="Get the ERRSTAT Register."]
    #[inline] pub fn errstat_reg(&self) -> Register<Errstat> { 
        Register::new(self.0 as *mut Errstat, 0x88)
    }

    #[doc="Get the *mut pointer for the ERRSTAT register."]
    #[inline] pub fn errstat_mut(&self) -> *mut Errstat { 
        self.errstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the ERRSTAT register."]
    #[inline] pub fn errstat_ptr(&self) -> *const Errstat { 
        self.errstat_reg().ptr()
    }

    #[doc="Read the ERRSTAT register."]
    #[inline] pub fn errstat(&self) -> Errstat { 
        self.errstat_reg().read()
    }

    #[doc="Write the ERRSTAT register."]
    #[inline] pub fn write_errstat(&self, value: Errstat) -> &Self { 
        self.errstat_reg().write(value);
        self
    }

    #[doc="Set the ERRSTAT register."]
    #[inline] pub fn set_errstat<F: FnOnce(Errstat) -> Errstat>(&self, f: F) -> &Self {
        self.errstat_reg().set(f);
        self
    }

    #[doc="Modify the ERRSTAT register."]
    #[inline] pub fn with_errstat<F: FnOnce(Errstat) -> Errstat>(&self, f: F) -> &Self {
        self.errstat_reg().with(f);
        self
    }

    #[doc="Get the ERREN Register."]
    #[inline] pub fn erren_reg(&self) -> Register<Erren> { 
        Register::new(self.0 as *mut Erren, 0x8c)
    }

    #[doc="Get the *mut pointer for the ERREN register."]
    #[inline] pub fn erren_mut(&self) -> *mut Erren { 
        self.erren_reg().ptr()
    }

    #[doc="Get the *const pointer for the ERREN register."]
    #[inline] pub fn erren_ptr(&self) -> *const Erren { 
        self.erren_reg().ptr()
    }

    #[doc="Read the ERREN register."]
    #[inline] pub fn erren(&self) -> Erren { 
        self.erren_reg().read()
    }

    #[doc="Write the ERREN register."]
    #[inline] pub fn write_erren(&self, value: Erren) -> &Self { 
        self.erren_reg().write(value);
        self
    }

    #[doc="Set the ERREN register."]
    #[inline] pub fn set_erren<F: FnOnce(Erren) -> Erren>(&self, f: F) -> &Self {
        self.erren_reg().set(f);
        self
    }

    #[doc="Modify the ERREN register."]
    #[inline] pub fn with_erren<F: FnOnce(Erren) -> Erren>(&self, f: F) -> &Self {
        self.erren_reg().with(f);
        self
    }

    #[doc="Get the STAT Register."]
    #[inline] pub fn stat_reg(&self) -> Register<Stat> { 
        Register::new(self.0 as *mut Stat, 0x90)
    }

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        self.stat_reg().ptr()
    }

    #[doc="Get the *const pointer for the STAT register."]
    #[inline] pub fn stat_ptr(&self) -> *const Stat { 
        self.stat_reg().ptr()
    }

    #[doc="Read the STAT register."]
    #[inline] pub fn stat(&self) -> Stat { 
        self.stat_reg().read()
    }

    #[doc="Get the CTL Register."]
    #[inline] pub fn ctl_reg(&self) -> Register<Ctl> { 
        Register::new(self.0 as *mut Ctl, 0x94)
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut Ctl { 
        self.ctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTL register."]
    #[inline] pub fn ctl_ptr(&self) -> *const Ctl { 
        self.ctl_reg().ptr()
    }

    #[doc="Read the CTL register."]
    #[inline] pub fn ctl(&self) -> Ctl { 
        self.ctl_reg().read()
    }

    #[doc="Write the CTL register."]
    #[inline] pub fn write_ctl(&self, value: Ctl) -> &Self { 
        self.ctl_reg().write(value);
        self
    }

    #[doc="Set the CTL register."]
    #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        self.ctl_reg().set(f);
        self
    }

    #[doc="Modify the CTL register."]
    #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        self.ctl_reg().with(f);
        self
    }

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> Register<Addr> { 
        Register::new(self.0 as *mut Addr, 0x98)
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

    #[doc="Get the BDTPAGE1 Register."]
    #[inline] pub fn bdtpage1_reg(&self) -> Register<Bdtpage1> { 
        Register::new(self.0 as *mut Bdtpage1, 0x9c)
    }

    #[doc="Get the *mut pointer for the BDTPAGE1 register."]
    #[inline] pub fn bdtpage1_mut(&self) -> *mut Bdtpage1 { 
        self.bdtpage1_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDTPAGE1 register."]
    #[inline] pub fn bdtpage1_ptr(&self) -> *const Bdtpage1 { 
        self.bdtpage1_reg().ptr()
    }

    #[doc="Read the BDTPAGE1 register."]
    #[inline] pub fn bdtpage1(&self) -> Bdtpage1 { 
        self.bdtpage1_reg().read()
    }

    #[doc="Write the BDTPAGE1 register."]
    #[inline] pub fn write_bdtpage1(&self, value: Bdtpage1) -> &Self { 
        self.bdtpage1_reg().write(value);
        self
    }

    #[doc="Set the BDTPAGE1 register."]
    #[inline] pub fn set_bdtpage1<F: FnOnce(Bdtpage1) -> Bdtpage1>(&self, f: F) -> &Self {
        self.bdtpage1_reg().set(f);
        self
    }

    #[doc="Modify the BDTPAGE1 register."]
    #[inline] pub fn with_bdtpage1<F: FnOnce(Bdtpage1) -> Bdtpage1>(&self, f: F) -> &Self {
        self.bdtpage1_reg().with(f);
        self
    }

    #[doc="Get the FRMNUML Register."]
    #[inline] pub fn frmnuml_reg(&self) -> Register<Frmnuml> { 
        Register::new(self.0 as *mut Frmnuml, 0xa0)
    }

    #[doc="Get the *mut pointer for the FRMNUML register."]
    #[inline] pub fn frmnuml_mut(&self) -> *mut Frmnuml { 
        self.frmnuml_reg().ptr()
    }

    #[doc="Get the *const pointer for the FRMNUML register."]
    #[inline] pub fn frmnuml_ptr(&self) -> *const Frmnuml { 
        self.frmnuml_reg().ptr()
    }

    #[doc="Read the FRMNUML register."]
    #[inline] pub fn frmnuml(&self) -> Frmnuml { 
        self.frmnuml_reg().read()
    }

    #[doc="Write the FRMNUML register."]
    #[inline] pub fn write_frmnuml(&self, value: Frmnuml) -> &Self { 
        self.frmnuml_reg().write(value);
        self
    }

    #[doc="Set the FRMNUML register."]
    #[inline] pub fn set_frmnuml<F: FnOnce(Frmnuml) -> Frmnuml>(&self, f: F) -> &Self {
        self.frmnuml_reg().set(f);
        self
    }

    #[doc="Modify the FRMNUML register."]
    #[inline] pub fn with_frmnuml<F: FnOnce(Frmnuml) -> Frmnuml>(&self, f: F) -> &Self {
        self.frmnuml_reg().with(f);
        self
    }

    #[doc="Get the FRMNUMH Register."]
    #[inline] pub fn frmnumh_reg(&self) -> Register<Frmnumh> { 
        Register::new(self.0 as *mut Frmnumh, 0xa4)
    }

    #[doc="Get the *mut pointer for the FRMNUMH register."]
    #[inline] pub fn frmnumh_mut(&self) -> *mut Frmnumh { 
        self.frmnumh_reg().ptr()
    }

    #[doc="Get the *const pointer for the FRMNUMH register."]
    #[inline] pub fn frmnumh_ptr(&self) -> *const Frmnumh { 
        self.frmnumh_reg().ptr()
    }

    #[doc="Read the FRMNUMH register."]
    #[inline] pub fn frmnumh(&self) -> Frmnumh { 
        self.frmnumh_reg().read()
    }

    #[doc="Write the FRMNUMH register."]
    #[inline] pub fn write_frmnumh(&self, value: Frmnumh) -> &Self { 
        self.frmnumh_reg().write(value);
        self
    }

    #[doc="Set the FRMNUMH register."]
    #[inline] pub fn set_frmnumh<F: FnOnce(Frmnumh) -> Frmnumh>(&self, f: F) -> &Self {
        self.frmnumh_reg().set(f);
        self
    }

    #[doc="Modify the FRMNUMH register."]
    #[inline] pub fn with_frmnumh<F: FnOnce(Frmnumh) -> Frmnumh>(&self, f: F) -> &Self {
        self.frmnumh_reg().with(f);
        self
    }

    #[doc="Get the TOKEN Register."]
    #[inline] pub fn token_reg(&self) -> Register<Token> { 
        Register::new(self.0 as *mut Token, 0xa8)
    }

    #[doc="Get the *mut pointer for the TOKEN register."]
    #[inline] pub fn token_mut(&self) -> *mut Token { 
        self.token_reg().ptr()
    }

    #[doc="Get the *const pointer for the TOKEN register."]
    #[inline] pub fn token_ptr(&self) -> *const Token { 
        self.token_reg().ptr()
    }

    #[doc="Read the TOKEN register."]
    #[inline] pub fn token(&self) -> Token { 
        self.token_reg().read()
    }

    #[doc="Write the TOKEN register."]
    #[inline] pub fn write_token(&self, value: Token) -> &Self { 
        self.token_reg().write(value);
        self
    }

    #[doc="Set the TOKEN register."]
    #[inline] pub fn set_token<F: FnOnce(Token) -> Token>(&self, f: F) -> &Self {
        self.token_reg().set(f);
        self
    }

    #[doc="Modify the TOKEN register."]
    #[inline] pub fn with_token<F: FnOnce(Token) -> Token>(&self, f: F) -> &Self {
        self.token_reg().with(f);
        self
    }

    #[doc="Get the SOFTHLD Register."]
    #[inline] pub fn softhld_reg(&self) -> Register<Softhld> { 
        Register::new(self.0 as *mut Softhld, 0xac)
    }

    #[doc="Get the *mut pointer for the SOFTHLD register."]
    #[inline] pub fn softhld_mut(&self) -> *mut Softhld { 
        self.softhld_reg().ptr()
    }

    #[doc="Get the *const pointer for the SOFTHLD register."]
    #[inline] pub fn softhld_ptr(&self) -> *const Softhld { 
        self.softhld_reg().ptr()
    }

    #[doc="Read the SOFTHLD register."]
    #[inline] pub fn softhld(&self) -> Softhld { 
        self.softhld_reg().read()
    }

    #[doc="Write the SOFTHLD register."]
    #[inline] pub fn write_softhld(&self, value: Softhld) -> &Self { 
        self.softhld_reg().write(value);
        self
    }

    #[doc="Set the SOFTHLD register."]
    #[inline] pub fn set_softhld<F: FnOnce(Softhld) -> Softhld>(&self, f: F) -> &Self {
        self.softhld_reg().set(f);
        self
    }

    #[doc="Modify the SOFTHLD register."]
    #[inline] pub fn with_softhld<F: FnOnce(Softhld) -> Softhld>(&self, f: F) -> &Self {
        self.softhld_reg().with(f);
        self
    }

    #[doc="Get the BDTPAGE2 Register."]
    #[inline] pub fn bdtpage2_reg(&self) -> Register<Bdtpage2> { 
        Register::new(self.0 as *mut Bdtpage2, 0xb0)
    }

    #[doc="Get the *mut pointer for the BDTPAGE2 register."]
    #[inline] pub fn bdtpage2_mut(&self) -> *mut Bdtpage2 { 
        self.bdtpage2_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDTPAGE2 register."]
    #[inline] pub fn bdtpage2_ptr(&self) -> *const Bdtpage2 { 
        self.bdtpage2_reg().ptr()
    }

    #[doc="Read the BDTPAGE2 register."]
    #[inline] pub fn bdtpage2(&self) -> Bdtpage2 { 
        self.bdtpage2_reg().read()
    }

    #[doc="Write the BDTPAGE2 register."]
    #[inline] pub fn write_bdtpage2(&self, value: Bdtpage2) -> &Self { 
        self.bdtpage2_reg().write(value);
        self
    }

    #[doc="Set the BDTPAGE2 register."]
    #[inline] pub fn set_bdtpage2<F: FnOnce(Bdtpage2) -> Bdtpage2>(&self, f: F) -> &Self {
        self.bdtpage2_reg().set(f);
        self
    }

    #[doc="Modify the BDTPAGE2 register."]
    #[inline] pub fn with_bdtpage2<F: FnOnce(Bdtpage2) -> Bdtpage2>(&self, f: F) -> &Self {
        self.bdtpage2_reg().with(f);
        self
    }

    #[doc="Get the BDTPAGE3 Register."]
    #[inline] pub fn bdtpage3_reg(&self) -> Register<Bdtpage3> { 
        Register::new(self.0 as *mut Bdtpage3, 0xb4)
    }

    #[doc="Get the *mut pointer for the BDTPAGE3 register."]
    #[inline] pub fn bdtpage3_mut(&self) -> *mut Bdtpage3 { 
        self.bdtpage3_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDTPAGE3 register."]
    #[inline] pub fn bdtpage3_ptr(&self) -> *const Bdtpage3 { 
        self.bdtpage3_reg().ptr()
    }

    #[doc="Read the BDTPAGE3 register."]
    #[inline] pub fn bdtpage3(&self) -> Bdtpage3 { 
        self.bdtpage3_reg().read()
    }

    #[doc="Write the BDTPAGE3 register."]
    #[inline] pub fn write_bdtpage3(&self, value: Bdtpage3) -> &Self { 
        self.bdtpage3_reg().write(value);
        self
    }

    #[doc="Set the BDTPAGE3 register."]
    #[inline] pub fn set_bdtpage3<F: FnOnce(Bdtpage3) -> Bdtpage3>(&self, f: F) -> &Self {
        self.bdtpage3_reg().set(f);
        self
    }

    #[doc="Modify the BDTPAGE3 register."]
    #[inline] pub fn with_bdtpage3<F: FnOnce(Bdtpage3) -> Bdtpage3>(&self, f: F) -> &Self {
        self.bdtpage3_reg().with(f);
        self
    }

    #[doc="Get the ENDPT Register."]
    #[inline] pub fn endpt_reg(&self) -> RegisterArray<Endpt, bits::R16> { 
        RegisterArray::new(self.0 as *mut Endpt, 0xc0, 0x4)
    }

    #[doc="Get the *mut pointer for the ENDPT register."]
    #[inline] pub fn endpt_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Endpt { 
        self.endpt_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the ENDPT register."]
    #[inline] pub fn endpt_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Endpt { 
        self.endpt_reg().ptr(index.into())
    }

    #[doc="Read the ENDPT register."]
    #[inline] pub fn endpt<I: Into<bits::R16>>(&self, index: I) -> Endpt { 
        self.endpt_reg().read(index.into())
    }

    #[doc="Write the ENDPT register."]
    #[inline] pub fn write_endpt<I: Into<bits::R16>>(&self, index: I, value: Endpt) -> &Self {
        self.endpt_reg().write(index.into(), value);
        self
    }

    #[doc="Set the ENDPT register."]
    #[inline] pub fn set_endpt<I: Into<bits::R16>, F: FnOnce(Endpt) -> Endpt>(&self, index: I, f: F) -> &Self {
        self.endpt_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the ENDPT register."]
    #[inline] pub fn with_endpt<I: Into<bits::R16> + Copy, F: FnOnce(Endpt) -> Endpt>(&self, index: I, f: F) -> &Self {
        self.endpt_reg().with(index.into(), f);
        self
    }

    #[doc="Get the USBCTRL Register."]
    #[inline] pub fn usbctrl_reg(&self) -> Register<Usbctrl> { 
        Register::new(self.0 as *mut Usbctrl, 0x100)
    }

    #[doc="Get the *mut pointer for the USBCTRL register."]
    #[inline] pub fn usbctrl_mut(&self) -> *mut Usbctrl { 
        self.usbctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the USBCTRL register."]
    #[inline] pub fn usbctrl_ptr(&self) -> *const Usbctrl { 
        self.usbctrl_reg().ptr()
    }

    #[doc="Read the USBCTRL register."]
    #[inline] pub fn usbctrl(&self) -> Usbctrl { 
        self.usbctrl_reg().read()
    }

    #[doc="Write the USBCTRL register."]
    #[inline] pub fn write_usbctrl(&self, value: Usbctrl) -> &Self { 
        self.usbctrl_reg().write(value);
        self
    }

    #[doc="Set the USBCTRL register."]
    #[inline] pub fn set_usbctrl<F: FnOnce(Usbctrl) -> Usbctrl>(&self, f: F) -> &Self {
        self.usbctrl_reg().set(f);
        self
    }

    #[doc="Modify the USBCTRL register."]
    #[inline] pub fn with_usbctrl<F: FnOnce(Usbctrl) -> Usbctrl>(&self, f: F) -> &Self {
        self.usbctrl_reg().with(f);
        self
    }

    #[doc="Get the OBSERVE Register."]
    #[inline] pub fn observe_reg(&self) -> Register<Observe> { 
        Register::new(self.0 as *mut Observe, 0x104)
    }

    #[doc="Get the *mut pointer for the OBSERVE register."]
    #[inline] pub fn observe_mut(&self) -> *mut Observe { 
        self.observe_reg().ptr()
    }

    #[doc="Get the *const pointer for the OBSERVE register."]
    #[inline] pub fn observe_ptr(&self) -> *const Observe { 
        self.observe_reg().ptr()
    }

    #[doc="Read the OBSERVE register."]
    #[inline] pub fn observe(&self) -> Observe { 
        self.observe_reg().read()
    }

    #[doc="Get the CONTROL Register."]
    #[inline] pub fn control_reg(&self) -> Register<Control> { 
        Register::new(self.0 as *mut Control, 0x108)
    }

    #[doc="Get the *mut pointer for the CONTROL register."]
    #[inline] pub fn control_mut(&self) -> *mut Control { 
        self.control_reg().ptr()
    }

    #[doc="Get the *const pointer for the CONTROL register."]
    #[inline] pub fn control_ptr(&self) -> *const Control { 
        self.control_reg().ptr()
    }

    #[doc="Read the CONTROL register."]
    #[inline] pub fn control(&self) -> Control { 
        self.control_reg().read()
    }

    #[doc="Write the CONTROL register."]
    #[inline] pub fn write_control(&self, value: Control) -> &Self { 
        self.control_reg().write(value);
        self
    }

    #[doc="Set the CONTROL register."]
    #[inline] pub fn set_control<F: FnOnce(Control) -> Control>(&self, f: F) -> &Self {
        self.control_reg().set(f);
        self
    }

    #[doc="Modify the CONTROL register."]
    #[inline] pub fn with_control<F: FnOnce(Control) -> Control>(&self, f: F) -> &Self {
        self.control_reg().with(f);
        self
    }

    #[doc="Get the USBTRC0 Register."]
    #[inline] pub fn usbtrc0_reg(&self) -> Register<Usbtrc0> { 
        Register::new(self.0 as *mut Usbtrc0, 0x10c)
    }

    #[doc="Get the *mut pointer for the USBTRC0 register."]
    #[inline] pub fn usbtrc0_mut(&self) -> *mut Usbtrc0 { 
        self.usbtrc0_reg().ptr()
    }

    #[doc="Get the *const pointer for the USBTRC0 register."]
    #[inline] pub fn usbtrc0_ptr(&self) -> *const Usbtrc0 { 
        self.usbtrc0_reg().ptr()
    }

    #[doc="Read the USBTRC0 register."]
    #[inline] pub fn usbtrc0(&self) -> Usbtrc0 { 
        self.usbtrc0_reg().read()
    }

    #[doc="Write the USBTRC0 register."]
    #[inline] pub fn write_usbtrc0(&self, value: Usbtrc0) -> &Self { 
        self.usbtrc0_reg().write(value);
        self
    }

    #[doc="Set the USBTRC0 register."]
    #[inline] pub fn set_usbtrc0<F: FnOnce(Usbtrc0) -> Usbtrc0>(&self, f: F) -> &Self {
        self.usbtrc0_reg().set(f);
        self
    }

    #[doc="Modify the USBTRC0 register."]
    #[inline] pub fn with_usbtrc0<F: FnOnce(Usbtrc0) -> Usbtrc0>(&self, f: F) -> &Self {
        self.usbtrc0_reg().with(f);
        self
    }

    #[doc="Get the USBFRMADJUST Register."]
    #[inline] pub fn usbfrmadjust_reg(&self) -> Register<Usbfrmadjust> { 
        Register::new(self.0 as *mut Usbfrmadjust, 0x114)
    }

    #[doc="Get the *mut pointer for the USBFRMADJUST register."]
    #[inline] pub fn usbfrmadjust_mut(&self) -> *mut Usbfrmadjust { 
        self.usbfrmadjust_reg().ptr()
    }

    #[doc="Get the *const pointer for the USBFRMADJUST register."]
    #[inline] pub fn usbfrmadjust_ptr(&self) -> *const Usbfrmadjust { 
        self.usbfrmadjust_reg().ptr()
    }

    #[doc="Read the USBFRMADJUST register."]
    #[inline] pub fn usbfrmadjust(&self) -> Usbfrmadjust { 
        self.usbfrmadjust_reg().read()
    }

    #[doc="Write the USBFRMADJUST register."]
    #[inline] pub fn write_usbfrmadjust(&self, value: Usbfrmadjust) -> &Self { 
        self.usbfrmadjust_reg().write(value);
        self
    }

    #[doc="Set the USBFRMADJUST register."]
    #[inline] pub fn set_usbfrmadjust<F: FnOnce(Usbfrmadjust) -> Usbfrmadjust>(&self, f: F) -> &Self {
        self.usbfrmadjust_reg().set(f);
        self
    }

    #[doc="Modify the USBFRMADJUST register."]
    #[inline] pub fn with_usbfrmadjust<F: FnOnce(Usbfrmadjust) -> Usbfrmadjust>(&self, f: F) -> &Self {
        self.usbfrmadjust_reg().with(f);
        self
    }

    #[doc="Get the CLK_RECOVER_CTRL Register."]
    #[inline] pub fn clk_recover_ctrl_reg(&self) -> Register<ClkRecoverCtrl> { 
        Register::new(self.0 as *mut ClkRecoverCtrl, 0x140)
    }

    #[doc="Get the *mut pointer for the CLK_RECOVER_CTRL register."]
    #[inline] pub fn clk_recover_ctrl_mut(&self) -> *mut ClkRecoverCtrl { 
        self.clk_recover_ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLK_RECOVER_CTRL register."]
    #[inline] pub fn clk_recover_ctrl_ptr(&self) -> *const ClkRecoverCtrl { 
        self.clk_recover_ctrl_reg().ptr()
    }

    #[doc="Read the CLK_RECOVER_CTRL register."]
    #[inline] pub fn clk_recover_ctrl(&self) -> ClkRecoverCtrl { 
        self.clk_recover_ctrl_reg().read()
    }

    #[doc="Write the CLK_RECOVER_CTRL register."]
    #[inline] pub fn write_clk_recover_ctrl(&self, value: ClkRecoverCtrl) -> &Self { 
        self.clk_recover_ctrl_reg().write(value);
        self
    }

    #[doc="Set the CLK_RECOVER_CTRL register."]
    #[inline] pub fn set_clk_recover_ctrl<F: FnOnce(ClkRecoverCtrl) -> ClkRecoverCtrl>(&self, f: F) -> &Self {
        self.clk_recover_ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CLK_RECOVER_CTRL register."]
    #[inline] pub fn with_clk_recover_ctrl<F: FnOnce(ClkRecoverCtrl) -> ClkRecoverCtrl>(&self, f: F) -> &Self {
        self.clk_recover_ctrl_reg().with(f);
        self
    }

    #[doc="Get the CLK_RECOVER_IRC_EN Register."]
    #[inline] pub fn clk_recover_irc_en_reg(&self) -> Register<ClkRecoverIrcEn> { 
        Register::new(self.0 as *mut ClkRecoverIrcEn, 0x144)
    }

    #[doc="Get the *mut pointer for the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn clk_recover_irc_en_mut(&self) -> *mut ClkRecoverIrcEn { 
        self.clk_recover_irc_en_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn clk_recover_irc_en_ptr(&self) -> *const ClkRecoverIrcEn { 
        self.clk_recover_irc_en_reg().ptr()
    }

    #[doc="Read the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn clk_recover_irc_en(&self) -> ClkRecoverIrcEn { 
        self.clk_recover_irc_en_reg().read()
    }

    #[doc="Write the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn write_clk_recover_irc_en(&self, value: ClkRecoverIrcEn) -> &Self { 
        self.clk_recover_irc_en_reg().write(value);
        self
    }

    #[doc="Set the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn set_clk_recover_irc_en<F: FnOnce(ClkRecoverIrcEn) -> ClkRecoverIrcEn>(&self, f: F) -> &Self {
        self.clk_recover_irc_en_reg().set(f);
        self
    }

    #[doc="Modify the CLK_RECOVER_IRC_EN register."]
    #[inline] pub fn with_clk_recover_irc_en<F: FnOnce(ClkRecoverIrcEn) -> ClkRecoverIrcEn>(&self, f: F) -> &Self {
        self.clk_recover_irc_en_reg().with(f);
        self
    }

    #[doc="Get the CLK_RECOVER_INT_STATUS Register."]
    #[inline] pub fn clk_recover_int_status_reg(&self) -> Register<ClkRecoverIntStatus> { 
        Register::new(self.0 as *mut ClkRecoverIntStatus, 0x15c)
    }

    #[doc="Get the *mut pointer for the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn clk_recover_int_status_mut(&self) -> *mut ClkRecoverIntStatus { 
        self.clk_recover_int_status_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn clk_recover_int_status_ptr(&self) -> *const ClkRecoverIntStatus { 
        self.clk_recover_int_status_reg().ptr()
    }

    #[doc="Read the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn clk_recover_int_status(&self) -> ClkRecoverIntStatus { 
        self.clk_recover_int_status_reg().read()
    }

    #[doc="Write the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn write_clk_recover_int_status(&self, value: ClkRecoverIntStatus) -> &Self { 
        self.clk_recover_int_status_reg().write(value);
        self
    }

    #[doc="Set the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn set_clk_recover_int_status<F: FnOnce(ClkRecoverIntStatus) -> ClkRecoverIntStatus>(&self, f: F) -> &Self {
        self.clk_recover_int_status_reg().set(f);
        self
    }

    #[doc="Modify the CLK_RECOVER_INT_STATUS register."]
    #[inline] pub fn with_clk_recover_int_status<F: FnOnce(ClkRecoverIntStatus) -> ClkRecoverIntStatus>(&self, f: F) -> &Self {
        self.clk_recover_int_status_reg().with(f);
        self
    }

}

#[doc="Peripheral ID register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Perid(pub u8);
impl Perid {
    #[doc="Peripheral Identification"]
    #[inline] pub fn id(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Perid {
    #[inline]
    fn from(other: u8) -> Self {
         Perid(other)
    }
}

impl ::core::fmt::Display for Perid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Perid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral ID Complement register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idcomp(pub u8);
impl Idcomp {
    #[doc="Ones\' complement of PERID[ID]. bits."]
    #[inline] pub fn nid(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if NID != 0"]
    #[inline] pub fn test_nid(&self) -> bool {
        self.nid() != 0
    }

    #[doc="Sets the NID field."]
    #[inline] pub fn set_nid<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Idcomp {
    #[inline]
    fn from(other: u8) -> Self {
         Idcomp(other)
    }
}

impl ::core::fmt::Display for Idcomp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idcomp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nid() != 0 { try!(write!(f, " nid=0x{:x}", self.nid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Revision register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rev(pub u8);
impl Rev {
    #[doc="Revision"]
    #[inline] pub fn rev(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if REV != 0"]
    #[inline] pub fn test_rev(&self) -> bool {
        self.rev() != 0
    }

    #[doc="Sets the REV field."]
    #[inline] pub fn set_rev<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rev {
    #[inline]
    fn from(other: u8) -> Self {
         Rev(other)
    }
}

impl ::core::fmt::Display for Rev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rev() != 0 { try!(write!(f, " rev=0x{:x}", self.rev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Additional Info register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addinfo(pub u8);
impl Addinfo {
    #[doc="This bit is set if host mode is enabled."]
    #[inline] pub fn iehost(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IEHOST != 0"]
    #[inline] pub fn test_iehost(&self) -> bool {
        self.iehost() != 0
    }

    #[doc="Sets the IEHOST field."]
    #[inline] pub fn set_iehost<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Assigned Interrupt Request Number"]
    #[inline] pub fn irqnum(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if IRQNUM != 0"]
    #[inline] pub fn test_irqnum(&self) -> bool {
        self.irqnum() != 0
    }

    #[doc="Sets the IRQNUM field."]
    #[inline] pub fn set_irqnum<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Addinfo {
    #[inline]
    fn from(other: u8) -> Self {
         Addinfo(other)
    }
}

impl ::core::fmt::Display for Addinfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addinfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iehost() != 0 { try!(write!(f, " iehost"))}
        if self.irqnum() != 0 { try!(write!(f, " irqnum=0x{:x}", self.irqnum()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG Interrupt Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Otgistat(pub u8);
impl Otgistat {
    #[doc="This bit is set when a change in VBUS is detected on an A device."]
    #[inline] pub fn avbuschg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AVBUSCHG != 0"]
    #[inline] pub fn test_avbuschg(&self) -> bool {
        self.avbuschg() != 0
    }

    #[doc="Sets the AVBUSCHG field."]
    #[inline] pub fn set_avbuschg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="This bit is set when a change in VBUS is detected on a B device."]
    #[inline] pub fn b_sess_chg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if B_SESS_CHG != 0"]
    #[inline] pub fn test_b_sess_chg(&self) -> bool {
        self.b_sess_chg() != 0
    }

    #[doc="Sets the B_SESS_CHG field."]
    #[inline] pub fn set_b_sess_chg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
    #[inline] pub fn sessvldchg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SESSVLDCHG != 0"]
    #[inline] pub fn test_sessvldchg(&self) -> bool {
        self.sessvldchg() != 0
    }

    #[doc="Sets the SESSVLDCHG field."]
    #[inline] pub fn set_sessvldchg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This interrupt is set when the USB line state (CTL[SE0] and CTL[JSTATE] bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline] pub fn line_state_chg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LINE_STATE_CHG != 0"]
    #[inline] pub fn test_line_state_chg(&self) -> bool {
        self.line_state_chg() != 0
    }

    #[doc="Sets the LINE_STATE_CHG field."]
    #[inline] pub fn set_line_state_chg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit is set when the 1 millisecond timer expires"]
    #[inline] pub fn onemsec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ONEMSEC != 0"]
    #[inline] pub fn test_onemsec(&self) -> bool {
        self.onemsec() != 0
    }

    #[doc="Sets the ONEMSEC field."]
    #[inline] pub fn set_onemsec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This bit is set when a change in the ID Signal from the USB connector is sensed."]
    #[inline] pub fn idchg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IDCHG != 0"]
    #[inline] pub fn test_idchg(&self) -> bool {
        self.idchg() != 0
    }

    #[doc="Sets the IDCHG field."]
    #[inline] pub fn set_idchg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Otgistat {
    #[inline]
    fn from(other: u8) -> Self {
         Otgistat(other)
    }
}

impl ::core::fmt::Display for Otgistat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Otgistat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avbuschg() != 0 { try!(write!(f, " avbuschg"))}
        if self.b_sess_chg() != 0 { try!(write!(f, " b_sess_chg"))}
        if self.sessvldchg() != 0 { try!(write!(f, " sessvldchg"))}
        if self.line_state_chg() != 0 { try!(write!(f, " line_state_chg"))}
        if self.onemsec() != 0 { try!(write!(f, " onemsec"))}
        if self.idchg() != 0 { try!(write!(f, " idchg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG Interrupt Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Otgicr(pub u8);
impl Otgicr {
    #[doc="A VBUS Valid Interrupt Enable"]
    #[inline] pub fn avbusen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AVBUSEN != 0"]
    #[inline] pub fn test_avbusen(&self) -> bool {
        self.avbusen() != 0
    }

    #[doc="Sets the AVBUSEN field."]
    #[inline] pub fn set_avbusen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="B Session END Interrupt Enable"]
    #[inline] pub fn bsessen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BSESSEN != 0"]
    #[inline] pub fn test_bsessen(&self) -> bool {
        self.bsessen() != 0
    }

    #[doc="Sets the BSESSEN field."]
    #[inline] pub fn set_bsessen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Session Valid Interrupt Enable"]
    #[inline] pub fn sessvlden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SESSVLDEN != 0"]
    #[inline] pub fn test_sessvlden(&self) -> bool {
        self.sessvlden() != 0
    }

    #[doc="Sets the SESSVLDEN field."]
    #[inline] pub fn set_sessvlden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Line State Change Interrupt Enable"]
    #[inline] pub fn linestateen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LINESTATEEN != 0"]
    #[inline] pub fn test_linestateen(&self) -> bool {
        self.linestateen() != 0
    }

    #[doc="Sets the LINESTATEEN field."]
    #[inline] pub fn set_linestateen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="One Millisecond Interrupt Enable"]
    #[inline] pub fn onemsecen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ONEMSECEN != 0"]
    #[inline] pub fn test_onemsecen(&self) -> bool {
        self.onemsecen() != 0
    }

    #[doc="Sets the ONEMSECEN field."]
    #[inline] pub fn set_onemsecen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ID Interrupt Enable"]
    #[inline] pub fn iden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if IDEN != 0"]
    #[inline] pub fn test_iden(&self) -> bool {
        self.iden() != 0
    }

    #[doc="Sets the IDEN field."]
    #[inline] pub fn set_iden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Otgicr {
    #[inline]
    fn from(other: u8) -> Self {
         Otgicr(other)
    }
}

impl ::core::fmt::Display for Otgicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Otgicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avbusen() != 0 { try!(write!(f, " avbusen"))}
        if self.bsessen() != 0 { try!(write!(f, " bsessen"))}
        if self.sessvlden() != 0 { try!(write!(f, " sessvlden"))}
        if self.linestateen() != 0 { try!(write!(f, " linestateen"))}
        if self.onemsecen() != 0 { try!(write!(f, " onemsecen"))}
        if self.iden() != 0 { try!(write!(f, " iden"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Otgstat(pub u8);
impl Otgstat {
    #[doc="A VBUS Valid"]
    #[inline] pub fn avbusvld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AVBUSVLD != 0"]
    #[inline] pub fn test_avbusvld(&self) -> bool {
        self.avbusvld() != 0
    }

    #[doc="Sets the AVBUSVLD field."]
    #[inline] pub fn set_avbusvld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="B Session End"]
    #[inline] pub fn bsessend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BSESSEND != 0"]
    #[inline] pub fn test_bsessend(&self) -> bool {
        self.bsessend() != 0
    }

    #[doc="Sets the BSESSEND field."]
    #[inline] pub fn set_bsessend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Session Valid"]
    #[inline] pub fn sess_vld(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SESS_VLD != 0"]
    #[inline] pub fn test_sess_vld(&self) -> bool {
        self.sess_vld() != 0
    }

    #[doc="Sets the SESS_VLD field."]
    #[inline] pub fn set_sess_vld<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond"]
    #[inline] pub fn linestatestable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LINESTATESTABLE != 0"]
    #[inline] pub fn test_linestatestable(&self) -> bool {
        self.linestatestable() != 0
    }

    #[doc="Sets the LINESTATESTABLE field."]
    #[inline] pub fn set_linestatestable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline] pub fn onemsecen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ONEMSECEN != 0"]
    #[inline] pub fn test_onemsecen(&self) -> bool {
        self.onemsecen() != 0
    }

    #[doc="Sets the ONEMSECEN field."]
    #[inline] pub fn set_onemsecen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Indicates the current state of the ID pin on the USB connector"]
    #[inline] pub fn id(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Otgstat {
    #[inline]
    fn from(other: u8) -> Self {
         Otgstat(other)
    }
}

impl ::core::fmt::Display for Otgstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Otgstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avbusvld() != 0 { try!(write!(f, " avbusvld"))}
        if self.bsessend() != 0 { try!(write!(f, " bsessend"))}
        if self.sess_vld() != 0 { try!(write!(f, " sess_vld"))}
        if self.linestatestable() != 0 { try!(write!(f, " linestatestable"))}
        if self.onemsecen() != 0 { try!(write!(f, " onemsecen"))}
        if self.id() != 0 { try!(write!(f, " id"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Otgctl(pub u8);
impl Otgctl {
    #[doc="On-The-Go pullup/pulldown resistor enable"]
    #[inline] pub fn otgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OTGEN != 0"]
    #[inline] pub fn test_otgen(&self) -> bool {
        self.otgen() != 0
    }

    #[doc="Sets the OTGEN field."]
    #[inline] pub fn set_otgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="D- Data Line pull-down resistor enable"]
    #[inline] pub fn dmlow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMLOW != 0"]
    #[inline] pub fn test_dmlow(&self) -> bool {
        self.dmlow() != 0
    }

    #[doc="Sets the DMLOW field."]
    #[inline] pub fn set_dmlow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="D+ Data Line pull-down resistor enable"]
    #[inline] pub fn dplow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DPLOW != 0"]
    #[inline] pub fn test_dplow(&self) -> bool {
        self.dplow() != 0
    }

    #[doc="Sets the DPLOW field."]
    #[inline] pub fn set_dplow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="D+ Data Line pullup resistor enable"]
    #[inline] pub fn dphigh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DPHIGH != 0"]
    #[inline] pub fn test_dphigh(&self) -> bool {
        self.dphigh() != 0
    }

    #[doc="Sets the DPHIGH field."]
    #[inline] pub fn set_dphigh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Otgctl {
    #[inline]
    fn from(other: u8) -> Self {
         Otgctl(other)
    }
}

impl ::core::fmt::Display for Otgctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Otgctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.otgen() != 0 { try!(write!(f, " otgen"))}
        if self.dmlow() != 0 { try!(write!(f, " dmlow"))}
        if self.dplow() != 0 { try!(write!(f, " dplow"))}
        if self.dphigh() != 0 { try!(write!(f, " dphigh"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Istat(pub u8);
impl Istat {
    #[doc="This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline] pub fn usbrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USBRST != 0"]
    #[inline] pub fn test_usbrst(&self) -> bool {
        self.usbrst() != 0
    }

    #[doc="Sets the USBRST field."]
    #[inline] pub fn set_usbrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline] pub fn error(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline] pub fn softok(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOFTOK != 0"]
    #[inline] pub fn test_softok(&self) -> bool {
        self.softok() != 0
    }

    #[doc="Sets the SOFTOK field."]
    #[inline] pub fn set_softok<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit is set when the current token being processed has completed"]
    #[inline] pub fn tokdne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOKDNE != 0"]
    #[inline] pub fn test_tokdne(&self) -> bool {
        self.tokdne() != 0
    }

    #[doc="Sets the TOKDNE field."]
    #[inline] pub fn set_tokdne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline] pub fn sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLEEP != 0"]
    #[inline] pub fn test_sleep(&self) -> bool {
        self.sleep() != 0
    }

    #[doc="Sets the SLEEP field."]
    #[inline] pub fn set_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline] pub fn resume(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESUME != 0"]
    #[inline] pub fn test_resume(&self) -> bool {
        self.resume() != 0
    }

    #[doc="Sets the RESUME field."]
    #[inline] pub fn set_resume<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Attach Interrupt"]
    #[inline] pub fn attach(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ATTACH != 0"]
    #[inline] pub fn test_attach(&self) -> bool {
        self.attach() != 0
    }

    #[doc="Sets the ATTACH field."]
    #[inline] pub fn set_attach<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Stall Interrupt"]
    #[inline] pub fn stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Istat {
    #[inline]
    fn from(other: u8) -> Self {
         Istat(other)
    }
}

impl ::core::fmt::Display for Istat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Istat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.softok() != 0 { try!(write!(f, " softok"))}
        if self.tokdne() != 0 { try!(write!(f, " tokdne"))}
        if self.sleep() != 0 { try!(write!(f, " sleep"))}
        if self.resume() != 0 { try!(write!(f, " resume"))}
        if self.attach() != 0 { try!(write!(f, " attach"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u8);
impl Inten {
    #[doc="USBRST Interrupt Enable"]
    #[inline] pub fn usbrsten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USBRSTEN != 0"]
    #[inline] pub fn test_usbrsten(&self) -> bool {
        self.usbrsten() != 0
    }

    #[doc="Sets the USBRSTEN field."]
    #[inline] pub fn set_usbrsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ERROR Interrupt Enable"]
    #[inline] pub fn erroren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERROREN != 0"]
    #[inline] pub fn test_erroren(&self) -> bool {
        self.erroren() != 0
    }

    #[doc="Sets the ERROREN field."]
    #[inline] pub fn set_erroren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SOFTOK Interrupt Enable"]
    #[inline] pub fn softoken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOFTOKEN != 0"]
    #[inline] pub fn test_softoken(&self) -> bool {
        self.softoken() != 0
    }

    #[doc="Sets the SOFTOKEN field."]
    #[inline] pub fn set_softoken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TOKDNE Interrupt Enable"]
    #[inline] pub fn tokdneen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOKDNEEN != 0"]
    #[inline] pub fn test_tokdneen(&self) -> bool {
        self.tokdneen() != 0
    }

    #[doc="Sets the TOKDNEEN field."]
    #[inline] pub fn set_tokdneen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SLEEP Interrupt Enable"]
    #[inline] pub fn sleepen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLEEPEN != 0"]
    #[inline] pub fn test_sleepen(&self) -> bool {
        self.sleepen() != 0
    }

    #[doc="Sets the SLEEPEN field."]
    #[inline] pub fn set_sleepen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RESUME Interrupt Enable"]
    #[inline] pub fn resumeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESUMEEN != 0"]
    #[inline] pub fn test_resumeen(&self) -> bool {
        self.resumeen() != 0
    }

    #[doc="Sets the RESUMEEN field."]
    #[inline] pub fn set_resumeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ATTACH Interrupt Enable"]
    #[inline] pub fn attachen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ATTACHEN != 0"]
    #[inline] pub fn test_attachen(&self) -> bool {
        self.attachen() != 0
    }

    #[doc="Sets the ATTACHEN field."]
    #[inline] pub fn set_attachen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="STALL Interrupt Enable"]
    #[inline] pub fn stallen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if STALLEN != 0"]
    #[inline] pub fn test_stallen(&self) -> bool {
        self.stallen() != 0
    }

    #[doc="Sets the STALLEN field."]
    #[inline] pub fn set_stallen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Inten {
    #[inline]
    fn from(other: u8) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usbrsten() != 0 { try!(write!(f, " usbrsten"))}
        if self.erroren() != 0 { try!(write!(f, " erroren"))}
        if self.softoken() != 0 { try!(write!(f, " softoken"))}
        if self.tokdneen() != 0 { try!(write!(f, " tokdneen"))}
        if self.sleepen() != 0 { try!(write!(f, " sleepen"))}
        if self.resumeen() != 0 { try!(write!(f, " resumeen"))}
        if self.attachen() != 0 { try!(write!(f, " attachen"))}
        if self.stallen() != 0 { try!(write!(f, " stallen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Errstat(pub u8);
impl Errstat {
    #[doc="This bit is set when the PID check field fails."]
    #[inline] pub fn piderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PIDERR != 0"]
    #[inline] pub fn test_piderr(&self) -> bool {
        self.piderr() != 0
    }

    #[doc="Sets the PIDERR field."]
    #[inline] pub fn set_piderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="This error interrupt has two functions"]
    #[inline] pub fn crc5eof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRC5EOF != 0"]
    #[inline] pub fn test_crc5eof(&self) -> bool {
        self.crc5eof() != 0
    }

    #[doc="Sets the CRC5EOF field."]
    #[inline] pub fn set_crc5eof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline] pub fn crc16(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRC16 != 0"]
    #[inline] pub fn test_crc16(&self) -> bool {
        self.crc16() != 0
    }

    #[doc="Sets the CRC16 field."]
    #[inline] pub fn set_crc16<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit is set if the data field received was not 8 bits in length"]
    #[inline] pub fn dfn8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DFN8 != 0"]
    #[inline] pub fn test_dfn8(&self) -> bool {
        self.dfn8() != 0
    }

    #[doc="Sets the DFN8 field."]
    #[inline] pub fn set_dfn8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit is set when a bus turnaround timeout error occurs"]
    #[inline] pub fn btoerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BTOERR != 0"]
    #[inline] pub fn test_btoerr(&self) -> bool {
        self.btoerr() != 0
    }

    #[doc="Sets the BTOERR field."]
    #[inline] pub fn set_btoerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline] pub fn dmaerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAERR != 0"]
    #[inline] pub fn test_dmaerr(&self) -> bool {
        self.dmaerr() != 0
    }

    #[doc="Sets the DMAERR field."]
    #[inline] pub fn set_dmaerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This bit is set when a bit stuff error is detected"]
    #[inline] pub fn btserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BTSERR != 0"]
    #[inline] pub fn test_btserr(&self) -> bool {
        self.btserr() != 0
    }

    #[doc="Sets the BTSERR field."]
    #[inline] pub fn set_btserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Errstat {
    #[inline]
    fn from(other: u8) -> Self {
         Errstat(other)
    }
}

impl ::core::fmt::Display for Errstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Errstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.piderr() != 0 { try!(write!(f, " piderr"))}
        if self.crc5eof() != 0 { try!(write!(f, " crc5eof"))}
        if self.crc16() != 0 { try!(write!(f, " crc16"))}
        if self.dfn8() != 0 { try!(write!(f, " dfn8"))}
        if self.btoerr() != 0 { try!(write!(f, " btoerr"))}
        if self.dmaerr() != 0 { try!(write!(f, " dmaerr"))}
        if self.btserr() != 0 { try!(write!(f, " btserr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Error Interrupt Enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Erren(pub u8);
impl Erren {
    #[doc="PIDERR Interrupt Enable"]
    #[inline] pub fn piderren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PIDERREN != 0"]
    #[inline] pub fn test_piderren(&self) -> bool {
        self.piderren() != 0
    }

    #[doc="Sets the PIDERREN field."]
    #[inline] pub fn set_piderren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CRC5/EOF Interrupt Enable"]
    #[inline] pub fn crc5eofen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRC5EOFEN != 0"]
    #[inline] pub fn test_crc5eofen(&self) -> bool {
        self.crc5eofen() != 0
    }

    #[doc="Sets the CRC5EOFEN field."]
    #[inline] pub fn set_crc5eofen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CRC16 Interrupt Enable"]
    #[inline] pub fn crc16en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRC16EN != 0"]
    #[inline] pub fn test_crc16en(&self) -> bool {
        self.crc16en() != 0
    }

    #[doc="Sets the CRC16EN field."]
    #[inline] pub fn set_crc16en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DFN8 Interrupt Enable"]
    #[inline] pub fn dfn8en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DFN8EN != 0"]
    #[inline] pub fn test_dfn8en(&self) -> bool {
        self.dfn8en() != 0
    }

    #[doc="Sets the DFN8EN field."]
    #[inline] pub fn set_dfn8en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BTOERR Interrupt Enable"]
    #[inline] pub fn btoerren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BTOERREN != 0"]
    #[inline] pub fn test_btoerren(&self) -> bool {
        self.btoerren() != 0
    }

    #[doc="Sets the BTOERREN field."]
    #[inline] pub fn set_btoerren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMAERR Interrupt Enable"]
    #[inline] pub fn dmaerren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAERREN != 0"]
    #[inline] pub fn test_dmaerren(&self) -> bool {
        self.dmaerren() != 0
    }

    #[doc="Sets the DMAERREN field."]
    #[inline] pub fn set_dmaerren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="BTSERR Interrupt Enable"]
    #[inline] pub fn btserren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BTSERREN != 0"]
    #[inline] pub fn test_btserren(&self) -> bool {
        self.btserren() != 0
    }

    #[doc="Sets the BTSERREN field."]
    #[inline] pub fn set_btserren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Erren {
    #[inline]
    fn from(other: u8) -> Self {
         Erren(other)
    }
}

impl ::core::fmt::Display for Erren {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Erren {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.piderren() != 0 { try!(write!(f, " piderren"))}
        if self.crc5eofen() != 0 { try!(write!(f, " crc5eofen"))}
        if self.crc16en() != 0 { try!(write!(f, " crc16en"))}
        if self.dfn8en() != 0 { try!(write!(f, " dfn8en"))}
        if self.btoerren() != 0 { try!(write!(f, " btoerren"))}
        if self.dmaerren() != 0 { try!(write!(f, " dmaerren"))}
        if self.btserren() != 0 { try!(write!(f, " btserren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u8);
impl Stat {
    #[doc="This bit is set if the last buffer descriptor updated was in the odd bank of the BDT."]
    #[inline] pub fn odd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ODD != 0"]
    #[inline] pub fn test_odd(&self) -> bool {
        self.odd() != 0
    }

    #[doc="Sets the ODD field."]
    #[inline] pub fn set_odd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit Indicator"]
    #[inline] pub fn tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TX != 0"]
    #[inline] pub fn test_tx(&self) -> bool {
        self.tx() != 0
    }

    #[doc="Sets the TX field."]
    #[inline] pub fn set_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This four-bit field encodes the endpoint address that received or transmitted the previous token"]
    #[inline] pub fn endp(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if ENDP != 0"]
    #[inline] pub fn test_endp(&self) -> bool {
        self.endp() != 0
    }

    #[doc="Sets the ENDP field."]
    #[inline] pub fn set_endp<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Stat {
    #[inline]
    fn from(other: u8) -> Self {
         Stat(other)
    }
}

impl ::core::fmt::Display for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.odd() != 0 { try!(write!(f, " odd"))}
        if self.tx() != 0 { try!(write!(f, " tx"))}
        if self.endp() != 0 { try!(write!(f, " endp=0x{:x}", self.endp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u8);
impl Ctl {
    #[doc="USB Enable"]
    #[inline] pub fn usbensofen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USBENSOFEN != 0"]
    #[inline] pub fn test_usbensofen(&self) -> bool {
        self.usbensofen() != 0
    }

    #[doc="Sets the USBENSOFEN field."]
    #[inline] pub fn set_usbensofen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline] pub fn oddrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ODDRST != 0"]
    #[inline] pub fn test_oddrst(&self) -> bool {
        self.oddrst() != 0
    }

    #[doc="Sets the ODDRST field."]
    #[inline] pub fn set_oddrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline] pub fn resume(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RESUME != 0"]
    #[inline] pub fn test_resume(&self) -> bool {
        self.resume() != 0
    }

    #[doc="Sets the RESUME field."]
    #[inline] pub fn set_resume<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline] pub fn hostmodeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HOSTMODEEN != 0"]
    #[inline] pub fn test_hostmodeen(&self) -> bool {
        self.hostmodeen() != 0
    }

    #[doc="Sets the HOSTMODEEN field."]
    #[inline] pub fn set_hostmodeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline] pub fn txsuspendtokenbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXSUSPENDTOKENBUSY != 0"]
    #[inline] pub fn test_txsuspendtokenbusy(&self) -> bool {
        self.txsuspendtokenbusy() != 0
    }

    #[doc="Sets the TXSUSPENDTOKENBUSY field."]
    #[inline] pub fn set_txsuspendtokenbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Live USB Single Ended Zero signal"]
    #[inline] pub fn se0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SE0 != 0"]
    #[inline] pub fn test_se0(&self) -> bool {
        self.se0() != 0
    }

    #[doc="Sets the SE0 field."]
    #[inline] pub fn set_se0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Live USB differential receiver JSTATE signal"]
    #[inline] pub fn jstate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if JSTATE != 0"]
    #[inline] pub fn test_jstate(&self) -> bool {
        self.jstate() != 0
    }

    #[doc="Sets the JSTATE field."]
    #[inline] pub fn set_jstate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctl {
    #[inline]
    fn from(other: u8) -> Self {
         Ctl(other)
    }
}

impl ::core::fmt::Display for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usbensofen() != 0 { try!(write!(f, " usbensofen"))}
        if self.oddrst() != 0 { try!(write!(f, " oddrst"))}
        if self.resume() != 0 { try!(write!(f, " resume"))}
        if self.hostmodeen() != 0 { try!(write!(f, " hostmodeen"))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.txsuspendtokenbusy() != 0 { try!(write!(f, " txsuspendtokenbusy"))}
        if self.se0() != 0 { try!(write!(f, " se0"))}
        if self.jstate() != 0 { try!(write!(f, " jstate"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u8);
impl Addr {
    #[doc="USB Address"]
    #[inline] pub fn addr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Speed Enable bit"]
    #[inline] pub fn lsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LSEN != 0"]
    #[inline] pub fn test_lsen(&self) -> bool {
        self.lsen() != 0
    }

    #[doc="Sets the LSEN field."]
    #[inline] pub fn set_lsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Addr {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.lsen() != 0 { try!(write!(f, " lsen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BDT Page register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdtpage1(pub u8);
impl Bdtpage1 {
    #[doc="Provides address bits 15 through 9 of the BDT base address."]
    #[inline] pub fn bdtba(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if BDTBA != 0"]
    #[inline] pub fn test_bdtba(&self) -> bool {
        self.bdtba() != 0
    }

    #[doc="Sets the BDTBA field."]
    #[inline] pub fn set_bdtba<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Bdtpage1 {
    #[inline]
    fn from(other: u8) -> Self {
         Bdtpage1(other)
    }
}

impl ::core::fmt::Display for Bdtpage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdtpage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bdtba() != 0 { try!(write!(f, " bdtba=0x{:x}", self.bdtba()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frame Number register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frmnuml(pub u8);
impl Frmnuml {
    #[doc="This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline] pub fn frm(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRM != 0"]
    #[inline] pub fn test_frm(&self) -> bool {
        self.frm() != 0
    }

    #[doc="Sets the FRM field."]
    #[inline] pub fn set_frm<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Frmnuml {
    #[inline]
    fn from(other: u8) -> Self {
         Frmnuml(other)
    }
}

impl ::core::fmt::Display for Frmnuml {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frmnuml {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frm() != 0 { try!(write!(f, " frm=0x{:x}", self.frm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frame Number register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frmnumh(pub u8);
impl Frmnumh {
    #[doc="This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline] pub fn frm(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FRM != 0"]
    #[inline] pub fn test_frm(&self) -> bool {
        self.frm() != 0
    }

    #[doc="Sets the FRM field."]
    #[inline] pub fn set_frm<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Frmnumh {
    #[inline]
    fn from(other: u8) -> Self {
         Frmnumh(other)
    }
}

impl ::core::fmt::Display for Frmnumh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frmnumh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frm() != 0 { try!(write!(f, " frm=0x{:x}", self.frm()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Token register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Token(pub u8);
impl Token {
    #[doc="Holds the Endpoint address for the token command"]
    #[inline] pub fn tokenendpt(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if TOKENENDPT != 0"]
    #[inline] pub fn test_tokenendpt(&self) -> bool {
        self.tokenendpt() != 0
    }

    #[doc="Sets the TOKENENDPT field."]
    #[inline] pub fn set_tokenendpt<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Contains the token type executed by the USB module."]
    #[inline] pub fn tokenpid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if TOKENPID != 0"]
    #[inline] pub fn test_tokenpid(&self) -> bool {
        self.tokenpid() != 0
    }

    #[doc="Sets the TOKENPID field."]
    #[inline] pub fn set_tokenpid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Token {
    #[inline]
    fn from(other: u8) -> Self {
         Token(other)
    }
}

impl ::core::fmt::Display for Token {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Token {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tokenendpt() != 0 { try!(write!(f, " tokenendpt=0x{:x}", self.tokenendpt()))}
        if self.tokenpid() != 0 { try!(write!(f, " tokenpid=0x{:x}", self.tokenpid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SOF Threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Softhld(pub u8);
impl Softhld {
    #[doc="Represents the SOF count threshold in byte times."]
    #[inline] pub fn cnt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Softhld {
    #[inline]
    fn from(other: u8) -> Self {
         Softhld(other)
    }
}

impl ::core::fmt::Display for Softhld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Softhld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BDT Page Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdtpage2(pub u8);
impl Bdtpage2 {
    #[doc="Provides address bits 23 through 16 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
    #[inline] pub fn bdtba(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BDTBA != 0"]
    #[inline] pub fn test_bdtba(&self) -> bool {
        self.bdtba() != 0
    }

    #[doc="Sets the BDTBA field."]
    #[inline] pub fn set_bdtba<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bdtpage2 {
    #[inline]
    fn from(other: u8) -> Self {
         Bdtpage2(other)
    }
}

impl ::core::fmt::Display for Bdtpage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdtpage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bdtba() != 0 { try!(write!(f, " bdtba=0x{:x}", self.bdtba()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BDT Page Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdtpage3(pub u8);
impl Bdtpage3 {
    #[doc="Provides address bits 31 through 24 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
    #[inline] pub fn bdtba(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BDTBA != 0"]
    #[inline] pub fn test_bdtba(&self) -> bool {
        self.bdtba() != 0
    }

    #[doc="Sets the BDTBA field."]
    #[inline] pub fn set_bdtba<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bdtpage3 {
    #[inline]
    fn from(other: u8) -> Self {
         Bdtpage3(other)
    }
}

impl ::core::fmt::Display for Bdtpage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdtpage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bdtba() != 0 { try!(write!(f, " bdtba=0x{:x}", self.bdtba()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Endpoint Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Endpt(pub u8);
impl Endpt {
    #[doc="When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline] pub fn ephshk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EPHSHK != 0"]
    #[inline] pub fn test_ephshk(&self) -> bool {
        self.ephshk() != 0
    }

    #[doc="Sets the EPHSHK field."]
    #[inline] pub fn set_ephshk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="When set this bit indicates that the endpoint is called"]
    #[inline] pub fn epstall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPSTALL != 0"]
    #[inline] pub fn test_epstall(&self) -> bool {
        self.epstall() != 0
    }

    #[doc="Sets the EPSTALL field."]
    #[inline] pub fn set_epstall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit, when set, enables the endpoint for TX transfers."]
    #[inline] pub fn eptxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EPTXEN != 0"]
    #[inline] pub fn test_eptxen(&self) -> bool {
        self.eptxen() != 0
    }

    #[doc="Sets the EPTXEN field."]
    #[inline] pub fn set_eptxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit, when set, enables the endpoint for RX transfers."]
    #[inline] pub fn eprxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EPRXEN != 0"]
    #[inline] pub fn test_eprxen(&self) -> bool {
        self.eprxen() != 0
    }

    #[doc="Sets the EPRXEN field."]
    #[inline] pub fn set_eprxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit, when set, disables control (SETUP) transfers"]
    #[inline] pub fn epctldis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EPCTLDIS != 0"]
    #[inline] pub fn test_epctldis(&self) -> bool {
        self.epctldis() != 0
    }

    #[doc="Sets the EPCTLDIS field."]
    #[inline] pub fn set_epctldis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline] pub fn retrydis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RETRYDIS != 0"]
    #[inline] pub fn test_retrydis(&self) -> bool {
        self.retrydis() != 0
    }

    #[doc="Sets the RETRYDIS field."]
    #[inline] pub fn set_retrydis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline] pub fn hostwohub(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HOSTWOHUB != 0"]
    #[inline] pub fn test_hostwohub(&self) -> bool {
        self.hostwohub() != 0
    }

    #[doc="Sets the HOSTWOHUB field."]
    #[inline] pub fn set_hostwohub<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Endpt {
    #[inline]
    fn from(other: u8) -> Self {
         Endpt(other)
    }
}

impl ::core::fmt::Display for Endpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Endpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ephshk() != 0 { try!(write!(f, " ephshk"))}
        if self.epstall() != 0 { try!(write!(f, " epstall"))}
        if self.eptxen() != 0 { try!(write!(f, " eptxen"))}
        if self.eprxen() != 0 { try!(write!(f, " eprxen"))}
        if self.epctldis() != 0 { try!(write!(f, " epctldis"))}
        if self.retrydis() != 0 { try!(write!(f, " retrydis"))}
        if self.hostwohub() != 0 { try!(write!(f, " hostwohub"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Usbctrl(pub u8);
impl Usbctrl {
    #[doc="Enables the weak pulldowns on the USB transceiver."]
    #[inline] pub fn pde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PDE != 0"]
    #[inline] pub fn test_pde(&self) -> bool {
        self.pde() != 0
    }

    #[doc="Sets the PDE field."]
    #[inline] pub fn set_pde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Places the USB transceiver into the suspend state."]
    #[inline] pub fn susp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Usbctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Usbctrl(other)
    }
}

impl ::core::fmt::Display for Usbctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Usbctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pde() != 0 { try!(write!(f, " pde"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB OTG Observe register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Observe(pub u8);
impl Observe {
    #[doc="Provides observability of the D- Pulldown enable at the USB transceiver."]
    #[inline] pub fn dmpd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMPD != 0"]
    #[inline] pub fn test_dmpd(&self) -> bool {
        self.dmpd() != 0
    }

    #[doc="Sets the DMPD field."]
    #[inline] pub fn set_dmpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Provides observability of the D+ Pulldown enable at the USB transceiver."]
    #[inline] pub fn dppd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DPPD != 0"]
    #[inline] pub fn test_dppd(&self) -> bool {
        self.dppd() != 0
    }

    #[doc="Sets the DPPD field."]
    #[inline] pub fn set_dppd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Provides observability of the D+ Pullup enable at the USB transceiver."]
    #[inline] pub fn dppu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DPPU != 0"]
    #[inline] pub fn test_dppu(&self) -> bool {
        self.dppu() != 0
    }

    #[doc="Sets the DPPU field."]
    #[inline] pub fn set_dppu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Observe {
    #[inline]
    fn from(other: u8) -> Self {
         Observe(other)
    }
}

impl ::core::fmt::Display for Observe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Observe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmpd() != 0 { try!(write!(f, " dmpd"))}
        if self.dppd() != 0 { try!(write!(f, " dppd"))}
        if self.dppu() != 0 { try!(write!(f, " dppu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB OTG Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Control(pub u8);
impl Control {
    #[doc="Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
    #[inline] pub fn dppullupnonotg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DPPULLUPNONOTG != 0"]
    #[inline] pub fn test_dppullupnonotg(&self) -> bool {
        self.dppullupnonotg() != 0
    }

    #[doc="Sets the DPPULLUPNONOTG field."]
    #[inline] pub fn set_dppullupnonotg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Control {
    #[inline]
    fn from(other: u8) -> Self {
         Control(other)
    }
}

impl ::core::fmt::Display for Control {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Control {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dppullupnonotg() != 0 { try!(write!(f, " dppullupnonotg"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Transceiver Control register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Usbtrc0(pub u8);
impl Usbtrc0 {
    #[doc="USB Asynchronous Interrupt"]
    #[inline] pub fn usb_resume_int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USB_RESUME_INT != 0"]
    #[inline] pub fn test_usb_resume_int(&self) -> bool {
        self.usb_resume_int() != 0
    }

    #[doc="Sets the USB_RESUME_INT field."]
    #[inline] pub fn set_usb_resume_int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronous USB Interrupt Detect"]
    #[inline] pub fn sync_det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SYNC_DET != 0"]
    #[inline] pub fn test_sync_det(&self) -> bool {
        self.sync_det() != 0
    }

    #[doc="Sets the SYNC_DET field."]
    #[inline] pub fn set_sync_det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Combined USB Clock Recovery interrupt status"]
    #[inline] pub fn usb_clk_recovery_int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if USB_CLK_RECOVERY_INT != 0"]
    #[inline] pub fn test_usb_clk_recovery_int(&self) -> bool {
        self.usb_clk_recovery_int() != 0
    }

    #[doc="Sets the USB_CLK_RECOVERY_INT field."]
    #[inline] pub fn set_usb_clk_recovery_int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Asynchronous Resume Interrupt Enable"]
    #[inline] pub fn usbresmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USBRESMEN != 0"]
    #[inline] pub fn test_usbresmen(&self) -> bool {
        self.usbresmen() != 0
    }

    #[doc="Sets the USBRESMEN field."]
    #[inline] pub fn set_usbresmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="USB Reset"]
    #[inline] pub fn usbreset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if USBRESET != 0"]
    #[inline] pub fn test_usbreset(&self) -> bool {
        self.usbreset() != 0
    }

    #[doc="Sets the USBRESET field."]
    #[inline] pub fn set_usbreset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Usbtrc0 {
    #[inline]
    fn from(other: u8) -> Self {
         Usbtrc0(other)
    }
}

impl ::core::fmt::Display for Usbtrc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Usbtrc0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usb_resume_int() != 0 { try!(write!(f, " usb_resume_int"))}
        if self.sync_det() != 0 { try!(write!(f, " sync_det"))}
        if self.usb_clk_recovery_int() != 0 { try!(write!(f, " usb_clk_recovery_int"))}
        if self.usbresmen() != 0 { try!(write!(f, " usbresmen"))}
        if self.usbreset() != 0 { try!(write!(f, " usbreset"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frame Adjust Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Usbfrmadjust(pub u8);
impl Usbfrmadjust {
    #[doc="Frame Adjustment"]
    #[inline] pub fn adj(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ADJ != 0"]
    #[inline] pub fn test_adj(&self) -> bool {
        self.adj() != 0
    }

    #[doc="Sets the ADJ field."]
    #[inline] pub fn set_adj<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Usbfrmadjust {
    #[inline]
    fn from(other: u8) -> Self {
         Usbfrmadjust(other)
    }
}

impl ::core::fmt::Display for Usbfrmadjust {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Usbfrmadjust {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adj() != 0 { try!(write!(f, " adj=0x{:x}", self.adj()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Clock recovery control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClkRecoverCtrl(pub u8);
impl ClkRecoverCtrl {
    #[doc="Restart from IFR trim value"]
    #[inline] pub fn restart_ifrtrim_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RESTART_IFRTRIM_EN != 0"]
    #[inline] pub fn test_restart_ifrtrim_en(&self) -> bool {
        self.restart_ifrtrim_en() != 0
    }

    #[doc="Sets the RESTART_IFRTRIM_EN field."]
    #[inline] pub fn set_restart_ifrtrim_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Reset/resume to rough phase enable"]
    #[inline] pub fn reset_resume_rough_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RESET_RESUME_ROUGH_EN != 0"]
    #[inline] pub fn test_reset_resume_rough_en(&self) -> bool {
        self.reset_resume_rough_en() != 0
    }

    #[doc="Sets the RESET_RESUME_ROUGH_EN field."]
    #[inline] pub fn set_reset_resume_rough_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Crystal-less USB enable"]
    #[inline] pub fn clock_recover_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CLOCK_RECOVER_EN != 0"]
    #[inline] pub fn test_clock_recover_en(&self) -> bool {
        self.clock_recover_en() != 0
    }

    #[doc="Sets the CLOCK_RECOVER_EN field."]
    #[inline] pub fn set_clock_recover_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for ClkRecoverCtrl {
    #[inline]
    fn from(other: u8) -> Self {
         ClkRecoverCtrl(other)
    }
}

impl ::core::fmt::Display for ClkRecoverCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClkRecoverCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.restart_ifrtrim_en() != 0 { try!(write!(f, " restart_ifrtrim_en"))}
        if self.reset_resume_rough_en() != 0 { try!(write!(f, " reset_resume_rough_en"))}
        if self.clock_recover_en() != 0 { try!(write!(f, " clock_recover_en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IRC48M oscillator enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClkRecoverIrcEn(pub u8);
impl ClkRecoverIrcEn {
    #[doc="IRC48M regulator enable"]
    #[inline] pub fn reg_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if REG_EN != 0"]
    #[inline] pub fn test_reg_en(&self) -> bool {
        self.reg_en() != 0
    }

    #[doc="Sets the REG_EN field."]
    #[inline] pub fn set_reg_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IRC48M enable"]
    #[inline] pub fn irc_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IRC_EN != 0"]
    #[inline] pub fn test_irc_en(&self) -> bool {
        self.irc_en() != 0
    }

    #[doc="Sets the IRC_EN field."]
    #[inline] pub fn set_irc_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for ClkRecoverIrcEn {
    #[inline]
    fn from(other: u8) -> Self {
         ClkRecoverIrcEn(other)
    }
}

impl ::core::fmt::Display for ClkRecoverIrcEn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClkRecoverIrcEn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.reg_en() != 0 { try!(write!(f, " reg_en"))}
        if self.irc_en() != 0 { try!(write!(f, " irc_en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock recovery separated interrupt status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClkRecoverIntStatus(pub u8);
impl ClkRecoverIntStatus {
    #[doc="Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"]
    #[inline] pub fn ovf_error(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OVF_ERROR != 0"]
    #[inline] pub fn test_ovf_error(&self) -> bool {
        self.ovf_error() != 0
    }

    #[doc="Sets the OVF_ERROR field."]
    #[inline] pub fn set_ovf_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for ClkRecoverIntStatus {
    #[inline]
    fn from(other: u8) -> Self {
         ClkRecoverIntStatus(other)
    }
}

impl ::core::fmt::Display for ClkRecoverIntStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClkRecoverIntStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf_error() != 0 { try!(write!(f, " ovf_error"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


#[doc="Buffer Descriptor"]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferDesc(pub [u8; 8]);

impl BufferDesc {
#[doc="Read the BDESC register."]
    #[inline] pub fn bdesc(&self) -> Bdesc { 
        unsafe {
            read_volatile(self.0.as_ptr().offset(0x0) as *const Bdesc)
        }
    }

#[doc="Write the BDESC register."]
    #[inline] pub fn set_bdesc<F: FnOnce(Bdesc) -> Bdesc>(&mut self, f: F) -> &mut Self {
        unsafe {
            write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut Bdesc, f(Bdesc(0)));
        }
        self
  }

#[doc="Modfy the BDESC register."]
    #[inline] pub fn with_bdesc<F: FnOnce(Bdesc) -> Bdesc>(&mut self, f: F) -> &mut Self {
        unsafe {
            write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut Bdesc, f(self.bdesc()));
        }
      self
    }


#[doc="Read the BADDR register."]
    #[inline] pub fn baddr(&self) -> Baddr { 
        unsafe {
            read_volatile(self.0.as_ptr().offset(0x4) as *const Baddr)
        }
    }

#[doc="Write the BADDR register."]
    #[inline] pub fn set_baddr<F: FnOnce(Baddr) -> Baddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut Baddr, f(Baddr(0)));
        }
        self
  }

#[doc="Modfy the BADDR register."]
    #[inline] pub fn with_baddr<F: FnOnce(Baddr) -> Baddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut Baddr, f(self.baddr()));
        }
      self
    }


}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdesc(pub u32);
impl Bdesc {
    #[doc="Byte Count"]
    #[inline] pub fn bc(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if BC != 0"]
    #[inline] pub fn test_bc(&self) -> bool {
        self.bc() != 0
    }

    #[doc="Sets the BC field."]
    #[inline] pub fn set_bc<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Ownership: 0 = Processor, 1 = USB"]
    #[inline] pub fn own(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OWN != 0"]
    #[inline] pub fn test_own(&self) -> bool {
        self.own() != 0
    }

    #[doc="Sets the OWN field."]
    #[inline] pub fn set_own<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DATA0 / DATA1"]
    #[inline] pub fn data01(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DATA01 != 0"]
    #[inline] pub fn test_data01(&self) -> bool {
        self.data01() != 0
    }

    #[doc="Sets the DATA01 field."]
    #[inline] pub fn set_data01<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="KEEP"]
    #[inline] pub fn keep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if KEEP != 0"]
    #[inline] pub fn test_keep(&self) -> bool {
        self.keep() != 0
    }

    #[doc="Sets the KEEP field."]
    #[inline] pub fn set_keep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NINC"]
    #[inline] pub fn ninc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NINC != 0"]
    #[inline] pub fn test_ninc(&self) -> bool {
        self.ninc() != 0
    }

    #[doc="Sets the NINC field."]
    #[inline] pub fn set_ninc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DTS"]
    #[inline] pub fn dts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DTS != 0"]
    #[inline] pub fn test_dts(&self) -> bool {
        self.dts() != 0
    }

    #[doc="Sets the DTS field."]
    #[inline] pub fn set_dts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BDT_STALL"]
    #[inline] pub fn bdt_stall(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BDT_STALL != 0"]
    #[inline] pub fn test_bdt_stall(&self) -> bool {
        self.bdt_stall() != 0
    }

    #[doc="Sets the BDT_STALL field."]
    #[inline] pub fn set_bdt_stall<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Token PID"]
    #[inline] pub fn tok_pid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Returns true if TOK_PID != 0"]
    #[inline] pub fn test_tok_pid(&self) -> bool {
        self.tok_pid() != 0
    }

    #[doc="Sets the TOK_PID field."]
    #[inline] pub fn set_tok_pid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Bdesc {
    #[inline]
    fn from(other: u32) -> Self {
         Bdesc(other)
    }
}

impl ::core::fmt::Display for Bdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bc() != 0 { try!(write!(f, " bc=0x{:x}", self.bc()))}
        if self.own() != 0 { try!(write!(f, " own"))}
        if self.data01() != 0 { try!(write!(f, " data01"))}
        if self.keep() != 0 { try!(write!(f, " keep"))}
        if self.ninc() != 0 { try!(write!(f, " ninc"))}
        if self.dts() != 0 { try!(write!(f, " dts"))}
        if self.bdt_stall() != 0 { try!(write!(f, " bdt_stall"))}
        if self.tok_pid() != 0 { try!(write!(f, " tok_pid=0x{:x}", self.tok_pid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baddr(pub u32);
impl Baddr {
    #[doc="Buffer Address"]
    #[inline] pub fn addr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Baddr {
    #[inline]
    fn from(other: u32) -> Self {
         Baddr(other)
    }
}

impl ::core::fmt::Display for Baddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

