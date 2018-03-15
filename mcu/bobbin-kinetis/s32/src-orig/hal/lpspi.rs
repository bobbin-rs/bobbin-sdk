use bobbin_common::bits::*;
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

#[derive(Debug)]
pub struct Config {
    cfgr0: Cfgr0,
    cfgr1: Cfgr1,
    ccr: Ccr,
    fcr: Fcr,
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            cfgr0: Cfgr0(0),
            cfgr1: Cfgr1(0),
            ccr: Ccr(0),
            fcr: Fcr(0),
        }
    }
}

impl Config {
    // CFGR0

    pub fn rdmo(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_rdmo(value);
        self
    }

    pub fn cirfifo(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_cirfifo(value);
        self
    }

    pub fn hrsel(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hrsel(value);
        self
    }

    pub fn hrpol(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hrpol(value);
        self
    }

    pub fn hren(mut self, value: bool) -> Self {
        self.cfgr0 = self.cfgr0.set_hren(value);
        self
    }

    // CFGR1

    pub fn pcscfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_pcscfg(value);
        self
    }

    pub fn outcfg(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_outcfg(value);
        self
    }

    pub fn pincfg(mut self, value: U2) -> Self {
        self.cfgr1 = self.cfgr1.set_pincfg(value);
        self
    }

    pub fn matcfg(mut self, value: U3) -> Self {
        self.cfgr1 = self.cfgr1.set_matcfg(value);
        self
    }

    pub fn pcspol(mut self, value: U4) -> Self {
        self.cfgr1 = self.cfgr1.set_pcspol(value);
        self
    }

    pub fn nostall(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_nostall(value);
        self
    }

    pub fn autopcs(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_autopcs(value);
        self
    }    

    pub fn sample(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_sample(value);
        self
    }

    pub fn master(mut self, value: bool) -> Self {
        self.cfgr1 = self.cfgr1.set_master(value);
        self
    }       

    // CCR

    pub fn sckpcs(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckpcs(value as u32);
        self
    }       

    pub fn pcssck(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_pcssck(value as u32);
        self
    }    

    pub fn dbt(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_dbt(value as u32);
        self
    }    

    pub fn sckdiv(mut self, value: u8) -> Self {
        self.ccr = self.ccr.set_sckdiv(value as u32);
        self
    }       

    // FCR

    pub fn rxwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_rxwater(value as u32);
        self
    }

    pub fn txwater(mut self, value: u8) -> Self {
        self.fcr = self.fcr.set_txwater(value as u32);
        self
    }
}

pub struct Target<T> {
    pub periph: Periph<T>,
    pub tcr: Tcr
}

impl<T> Target<T> {
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

impl LpspiPeriph {
    fn configure(&self, config: Config) -> &Self {
        self
            .set_cfgr0(|_| config.cfgr0)
            .set_cfgr1(|_| config.cfgr1)
            .set_ccr(|_| config.ccr)
            .set_fcr(|_| config.fcr)
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_men(value))
    }    
    fn target(self) -> Target<T> {
        Target { periph: self, tcr: Tcr(0) }
    }
}