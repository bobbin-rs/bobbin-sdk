pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::spi::*;

pub use ::chip::lpspi::*;

pub enum Prescale {
    Div1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

#[derive(Debug, Default)]
pub struct Config {
    cfgr0: Cfgr0,
    cfgr1: Cfgr1,
    ccr: Ccr,
}

impl Config {
    pub fn set_master(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_master(value);
        self
    }
    pub fn set_clock_config(mut self, sckdiv: u8, dbt: u8, pcssck: u8, sckpcs: u8) -> Self {
        self.ccr = self.ccr
            .set_sckdiv(sckdiv)
            .set_dbt(dbt)
            .set_pcssck(pcssck)
            .set_sckpcs(sckpcs);
        self
    }
}


impl Configure<Config> for LpspiPeriph {
    fn config(&self) -> Config {
        Config {
            cfgr0: self.cfgr0(),
            cfgr1: self.cfgr1(),
            ccr: self.ccr(),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cfgr0(|_| cfg.cfgr0)
            .set_cfgr1(|_| cfg.cfgr1)
            .set_ccr(|_| cfg.ccr)
    }
}

impl Enabled for LpspiPeriph {
    fn enabled(&self) -> bool {        
        self.cr().test_men()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_men(value))
    }
}

impl LpspiPeriph {
    pub fn target(&self) -> Target {
        Target {
            periph: *self,
            tcr: Tcr(0),
        }
    }
}

pub struct Target {
    pub periph: LpspiPeriph,
    pub tcr: Tcr,
}

impl Target {
    pub fn cpol(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cpol(value);
        self
    }

    pub fn cpha(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cpha(value);
        self
    }

    pub fn prescale(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_prescale(value);
        self
    }

    pub fn pcs(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_pcs(value);
        self
    }

    pub fn lsbf(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_lsbf(value);
        self
    }

    pub fn bysw(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_bysw(value);
        self
    }

    pub fn cont(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_cont(value);
        self
    }

    pub fn contc(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_contc(value);
        self
    }

    pub fn rxmsk(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_rxmsk(value);
        self
    }

    pub fn txmsk(mut self, value: bool) -> Self {
        self.tcr = self.tcr.set_txmsk(value);
        self
    }

    pub fn width(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_width(value);
        self
    }

    pub fn framesz(mut self, value: u8) -> Self {
        self.tcr = self.tcr.set_framesz(value);
        self
    }

    pub fn configure(&self) -> &Self {
        self.periph.set_tcr(|_| self.tcr);
        self
    }        

    pub fn send(&self, data: u16) -> &Self {
        // self.configure();

        while self.periph.sr().tdf() == 0 {}
        self.periph.set_tdr(|r| r.set_data(data));            
        self
    }

    pub fn recv(&self) -> u16 {
        // self.configure();
        while self.periph.sr().rdf() == 0 {}
        self.periph.rdr().data().into()
    }    
}