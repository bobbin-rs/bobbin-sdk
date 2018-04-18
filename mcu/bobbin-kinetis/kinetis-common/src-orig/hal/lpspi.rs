pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_hal::spi::*;


use bobbin_sys::ring::Ring;
use bobbin_common::{Irq, Poll};
use bobbin_hal::digital::DigitalOutput;
use bobbin_cortexm::wfi;
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::hal::scb::SCB;
use ::hal::gpio::GpioPin;

use core::cell::Cell;
use core::marker::PhantomData;



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

    pub fn tx(&self, data: u8) {
        self.set_tdr(|r| r.set_data(data));
    }    

    pub fn rx(&self) -> u8 {
        self.rdr().data().value() as u8
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

    pub fn transfer(&self, tx: &[u8], rx: &mut [u8]) {
        while self.periph.sr().tdf() == 0 {}
        self.periph.set_tcr(|_| self.tcr.set_framesz(7));

        let mut tx_n = tx.len();
        let mut tx_c = tx.len();
        let mut tx_i = 0;

        let mut rx_n = rx.len();
        let mut rx_c = rx.len();
        let mut rx_i = 0;

        
        loop {
            let sr = self.periph.sr();
            
            if sr.test_tdf() {
                if tx_n > 0 {
                    self.periph.set_tdr(|r| r.set_data(tx[tx_i]));
                    tx_i = tx_i + 1;
                    tx_n = tx_n - 1;
                } else if rx_n > 0 {
                    self.periph.set_tdr(|r| r.set_data(rx_n));
                    rx_n = rx_n - 1;
                }            
            }

            if sr.test_rdf() {
                if tx_c > 0 {
                    let _ = self.periph.rdr().data();
                    tx_c = tx_c - 1;
                } else if rx_c > 0 {
                    rx[rx_i] = self.periph.rdr().data().into();
                    rx_i = rx_i + 1;
                    rx_c = rx_c - 1;                
                }
            }
            if tx_c == 0 && rx_c == 0 {
                break;
            }
        }
    }
    
}

impl Write for Target {
    fn write(&self, tx: &[u8]) {
        self.transfer(tx, &mut []);
    }
}

impl Read for Target {
    fn read(&self, rx: &mut [u8]) {
        self.transfer(&[], rx);
    }
}




#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpiAction {
    Idle,
    Start(u8),
    Repeat(u8),
    Write(u8),
    Transfer(u8),
    Stop(u8),
}

impl Default for SpiAction {
    fn default() -> Self {
        SpiAction::Idle
    }
}

pub struct SpiDriver<'a> {
    spi: LpspiPeriph,
    pins: &'a [GpioPin],
    repeat: Cell<u8>,
    action: Cell<Option<SpiAction>>,
    tx: Ring<'a, SpiAction>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for SpiDriver<'a> {}
unsafe impl<'a> Send for SpiDriver<'a> {}

impl<'a> SpiDriver<'a> {
    pub fn new<P: Into<LpspiPeriph>>(spi: P, pins: &'a [GpioPin], tx_buf: &'a mut [SpiAction], rx_buf: &'a mut [u8]) -> Self {
        SpiDriver { 
            spi: spi.into(),
            pins: pins,
            repeat: Cell::new(0),
            action: Cell::new(None),
            tx: Ring::new(tx_buf),
            rx: Ring::new(rx_buf),
            _phantom: PhantomData,
        }
    }

    pub fn enable_irq<I: Irq>(&self, irq: &I) {        
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn enqueue(&self, action: SpiAction) {        
        self.tx.enqueue(action);
        self.next();
    }

    pub fn tx_len(&self) -> usize {
        self.tx.len()
    }

    pub fn rx_len(&self) -> usize {
        self.rx.len()
    }

    pub fn read(&self, buf: &mut [u8]) {
        while self.rx.len() < buf.len() {
            wfi()
        }
        self.rx.read(buf);
    }

    pub fn read_byte(&self) -> u8 {
        while self.rx.len() == 0 {
            wfi()
        }
        self.rx.dequeue().unwrap()
    }

    pub fn action(&self) -> Option<SpiAction> {
        self.action.get()
    }

    pub fn next(&self) {
        if self.action().is_none() {
            loop {
                if let Some(action) = self.tx.dequeue() {
                    match action {
                        SpiAction::Start(pin) => {
                            self.pins[pin as usize].set_output(false);
                        },
                        SpiAction::Write(b) | SpiAction::Transfer(b) => { 
                            self.action.set(Some(action));
                            self.spi.set_enabled(true);
                            self.spi.set_tcr(|r| r.set_framesz(7));
                            self.spi.tx(b);
                            self.spi.with_ier(|r| r.set_tdie(true));
                            break;
                        },
                        SpiAction::Repeat(n) => {
                            self.repeat.set(n);
                        }
                        SpiAction::Stop(pin) => {
                            self.pins[pin as usize].set_output(true);
                            self.spi.set_enabled(false);                            
                        },
                        SpiAction::Idle => {}
                    }
                } else {
                    self.action.set(None);
                    self.transfer_disable();
                    break;
                }
            }
        }
    }

    pub fn reg_read(&self, pin: u8, reg: u8) -> u8 {
        assert!(self.rx.len() == 0);
        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Transfer(0x55));        
        self.enqueue(SpiAction::Stop(pin));
        while self.rx.len() == 0 {
            wfi()
        }
        self.rx.dequeue().unwrap()
    }

    pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Write(value));
        self.enqueue(SpiAction::Stop(pin));
        while self.action().is_some() {
            wfi()
        }
    }

    pub fn transfer(&self, pin: u8, tx_buf: &[u8], rx_buf: &mut [u8]) {
        self.enqueue(SpiAction::Start(pin));        
        for b in tx_buf.iter() {
            self.enqueue(SpiAction::Write(*b));
        }
        if rx_buf.len() > 0 {
            self.enqueue(SpiAction::Repeat((rx_buf.len() - 1) as u8));
        }
        self.enqueue(SpiAction::Transfer(0x55));
        self.enqueue(SpiAction::Stop(pin));
        self.read(rx_buf);
    }

    pub fn transfer_enable(&self) {        
        self.spi.with_ier(|r| r.set_tdie(true));
        self.spi.set_enabled(true);
    }
    
    pub fn transfer_disable(&self) {        
        self.spi.set_ier(|r| r);
        self.spi.set_enabled(false);
    }

}

impl<'a> Poll for SpiDriver<'a> {
    fn poll(&self) {       
        let sr = self.spi.sr();
        let action = self.action().unwrap();
        let repeat = self.repeat.get();
        // println!("SR: {:?} Action: {:?}", sr, self.action());        
        match action {
            SpiAction::Write(b) => { 
                if sr.rdf() != 0 {
                    let _: u8  = self.spi.rx(); 
                    if repeat > 0 {
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                }
            },
            SpiAction::Transfer(b) => { 
                if sr.rdf() != 0 {
                    self.rx.enqueue(self.spi.rx()); 
                    if repeat > 0 { 
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                }
            },
            _ => {},
        }            
        self.next();
    }
}
