pub use chip::tc::*;
pub use pm::PmEnabled;

pub enum Prescsync {
    GCLK = 0x0,
    PRESC = 0x1,
    RESYNC = 0x2,
}

pub enum Prescaler {
    Div1 = 0x0,
    Div2 = 0x1,
    Div4 = 0x2,
    Div8 = 0x3,
    Div16 = 0x4,
    Div64 = 0x5,
    Div256 = 0x6,
    Div1024 = 0x7,
}

pub enum Wavegen {
    NFRQ = 0x0,
    MFRQ = 0x1,
    NPWM = 0x2,
    MPWM = 0x3,
}

pub enum Direction {
    Up = 0x0,
    Down = 0x1,
}

pub struct Config {
    pub wavegen: Wavegen,
    pub prescaler: Prescaler,
    pub runstdby: bool,
    pub prescsync: Prescsync,
}

pub trait TcExt {
    fn configure_16bit(&self, cfg: Config) -> &Self;
    fn intflag(&self) -> count16::Intflag;
    fn set_enabled(&self, value: bool) -> &Self;
    fn set_cc(&self, index: usize, value: u16) -> &Self;
    fn clr_syncrdy(&self) -> &Self;
    fn set_mc0_enabled(&self, value: bool) -> &Self;
}

impl TcExt for TcImpl {
    fn configure_16bit(&self, cfg: Config) -> &Self {
        let tc16 = self.count16();
        tc16.set_ctrla(count16::Ctrla(0).set_enable(0));
        tc16.set_ctrla(count16::Ctrla(0).set_swrst(1));
        while tc16.ctrla().swrst() == 1 {}

        tc16.set_ctrla(count16::Ctrla(0)
            .set_mode(0x0)
            .set_wavegen(cfg.wavegen as u16)
            .set_prescaler(cfg.prescaler as u16)
            .set_runstdby(if cfg.runstdby { 1 } else { 0 })
            .set_prescsync(cfg.prescsync as u16)            
        );
        while tc16.status().syncbusy() != 0 {}
        tc16.set_intflag(count16::Intflag(0xff));
        self
    }


    fn intflag(&self) -> count16::Intflag {
        let tc16 = self.count16();
        tc16.intflag()
    }    

    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        let tc16 = self.count16();
        tc16.with_ctrla(|r| r.set_enable(value));
        self
    }    

    fn set_cc(&self, index: usize, value: u16) -> &Self {
        let tc16 = self.count16();
        tc16.set_cc(index, count16::Cc(0).set_cc(value));
        self
    }    


    fn clr_syncrdy(&self) -> &Self {
        let tc16 = self.count16();
        tc16.set_intflag(count16::Intflag(0).set_syncrdy(1));
        self
    }    
        
    fn set_mc0_enabled(&self, value: bool) -> &Self {
        let tc16 = self.count16();
        match value {
            true => tc16.set_intenset(count16::Intenset(0).set_mc0(1)),
            false => tc16.set_intenclr(count16::Intenclr(0).set_mc0(1)),
        };
        self
    }

}