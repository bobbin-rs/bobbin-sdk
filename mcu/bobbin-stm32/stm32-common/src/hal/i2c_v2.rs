pub use bobbin_common::i2c::*;
pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use ::chip::i2c_v2::*;

use bobbin_common::bits::*;

use bobbin_common::{Irq, Poll};
use bobbin_common::ring::Ring;
use bobbin_cortexm::wfi;
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::hal::scb::*;

use core::cell::Cell;
use core::marker::PhantomData;


#[derive(Debug, Default)]
pub struct Config {
    pub cr1: Cr1,
    pub cr2: Cr2,
    pub timingr: Timingr
}

impl Config {
    pub fn set_timing(mut self,
        presc: U4,
        scldel: U4,
        sdadel: U4,
        sclh: U8,
        scll: U8,
    ) -> Self {
        self.timingr = Timingr(0)
            .set_presc(presc)
            .set_scldel(scldel)
            .set_sdadel(sdadel)
            .set_sclh(sclh)
            .set_scll(scll);
        self
    }
}

impl Configure<Config> for I2cPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: Cr1(0),
            cr2: Cr2(0),
            timingr: Timingr(0),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cr1(|_| cfg.cr1)
            .set_cr2(|_| cfg.cr2)
            .set_timingr(|_| cfg.timingr)
    }
}

impl Enabled for I2cPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_pe()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_pe(value))        
    }
}

impl<A: Into<U7>> I2cTransfer<A> for I2cPeriph {
    fn transfer(&self, addr: A, out_buf: &[u8], in_buf: &mut[u8]) -> &Self {
        let addr = addr.into();
        self.set_enabled(true);
        if out_buf.len() > 0 {
            self.with_cr2(|r| r
                .set_sadd(addr.value() << 1)
                .set_rd_wrn(0)
                .set_nbytes(out_buf.len())
                .set_autoend(in_buf.len() == 0)
            );
            self.with_cr2(|r| r.set_start(1));
            for c in out_buf.iter() {
                while self.isr().txis() == 0 {}
                self.set_txdr(|r| r.set_txdata(*c));
            }
            if in_buf.len() > 0 {
                while self.isr().tc() == 0 {}
            }
        }
        if in_buf.len() > 0 {
            self.with_cr2(|r| r
                .set_sadd(addr.value() << 1)
                .set_rd_wrn(1)
                .set_nbytes(in_buf.len())        
            );
            self.with_cr2(|r| r.set_start(1));
            self.with_cr2(|r| r.set_autoend(1));

            for i in 0..in_buf.len() {
                while self.isr().rxne() == 0 {}
                in_buf[i] = self.rxdr().rxdata().value();
            }
        }
        while self.isr().busy() != 0 {}        
        self.set_enabled(false);
        self
    }    
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum I2cAction {
    Idle,
    Start(u8),
    Restart(u8),
    WriteBytes(u8),
    WriteByte(u8),
    ReadBytes(u8),
    ReadByte(u8), // Number of bytes remaining to read - 1
    Stop,
}

pub struct I2cDriver<'a> {
    i2c: I2cPeriph,
    action: Cell<Option<I2cAction>>,
    tx: Ring<'a, I2cAction>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for I2cDriver<'a> {}
unsafe impl<'a> Send for I2cDriver<'a> {}

impl<'a> I2cDriver<'a> {
    pub fn new<I: Into<I2cPeriph>>(i2c: I, tx_buf: &'a mut [I2cAction], rx_buf: &'a mut [u8]) -> Self {
        I2cDriver { 
            i2c: i2c.into(),
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

    pub fn action(&self) -> Option<I2cAction> {
        self.action.get()
    }

    pub fn read_reg(&self, addr: U7, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(addr, &[reg], &mut buf);
        buf[0]
    }

    pub fn write_reg(&self, addr: U7, reg: u8, value: u8) {
        let mut buf = [];
        self.transfer(addr, &[reg, value], &mut buf);
    }


    pub fn transfer(&self, addr: U7, tx_buf: &[u8], rx_buf: &mut [u8]) {
        // println!("transfer: addr={:?} tx_buf={:?} rx_buf={:?}", addr, tx_buf, rx_buf);
        if tx_buf.len() > 0 {
            self.tx.enqueue(I2cAction::Start(addr.value()));
            self.tx.enqueue(I2cAction::WriteBytes(tx_buf.len() as u8));
            for b in tx_buf.iter() {
                self.tx.enqueue(I2cAction::WriteByte(*b));
            }
            if rx_buf.len() == 0 {
                self.tx.enqueue(I2cAction::Stop);
            }
        }
        if rx_buf.len() > 0 {
            if tx_buf.len() == 0 {
                self.tx.enqueue(I2cAction::Start(addr.value()));
            } else {
                self.tx.enqueue(I2cAction::Restart(addr.value()));
            }
            self.tx.enqueue(I2cAction::ReadBytes(rx_buf.len() as u8));
            self.tx.enqueue(I2cAction::Stop);
        }
        self.next();
        loop {
            wfi();
            if self.rx.len() >= rx_buf.len() {
                self.rx.read(rx_buf);
                return
            }
        }
    }
    

    pub fn next(&self) {
        loop {
            // If currently processing an action, return without any changes
            if self.action().is_some() { return }        

            // Get the next action off of the queue
            if let Some(action) = self.tx.dequeue() {
                // println!("next: {:?}", action);                
                match action {
                    I2cAction::Idle => {},
                    I2cAction::Start(addr) => {
                        self.i2c.with_cr1(|r| r.set_pe(1));
                        self.i2c.with_cr2(|r| r.set_sadd(addr << 1));
                    },
                    I2cAction::Restart(addr) => {
                        self.i2c.with_cr2(|r| r.set_sadd(addr << 1));
                    },
                    I2cAction::WriteBytes(n) => {
                        self.i2c.with_cr2(|r| r.set_nbytes(n).set_rd_wrn(0));
                        self.i2c.with_cr2(|r| r.set_start(1));                        
                    },
                    I2cAction::WriteByte(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_cr1(|r| r.set_txie(1));
                    },
                    I2cAction::ReadBytes(n) => {
                        self.i2c.with_cr2(|r| r.set_nbytes(n).set_rd_wrn(1));
                        self.i2c.with_cr2(|r| r.set_start(1));
                        self.i2c.with_cr1(|r| r.set_rxie(1));
                        self.action.set(Some(I2cAction::ReadByte(n - 1)));
                    },
                    I2cAction::ReadByte(_) => {
                        panic!("Unexpected ReadByte in Tx Queue")
                    },
                    I2cAction::Stop => {
                        self.i2c.with_cr1(|r| r.set_tcie(1));                        
                        self.action.set(Some(action));
                    },
                }                
            } else {                
                self.i2c.with_cr1(|r| r.set_txie(0).set_rxie(0).set_tcie(0).set_stopie(0).set_pe(0));
                return
            }
        }
    }
}

impl<'a> Poll for I2cDriver<'a> {
    fn poll(&self) {       
        let isr = self.i2c.isr();
        let action = self.action().unwrap();
        // println!("ISR: {:?} Action: {:?}", isr, action);
        match action {
            I2cAction::WriteByte(n) => {
                if isr.test_txis() {
                    self.i2c.set_txdr(|r| r.set_txdata(n));
                    self.action.set(None)
                } else {
                    panic!("Unexpected Interrupt: {:?}", isr);
                }
            },
            I2cAction::ReadByte(n) => {
                if isr.test_rxne() {
                    self.rx.enqueue(self.i2c.rxdr().rxdata().value());
                    if n > 0 {
                        self.action.set(Some(I2cAction::ReadByte(n - 1)))
                    } else {
                        self.action.set(None)
                    }                    
                } else {                    
                    panic!("Unexpected Interrupt: {:?}", isr);
                }
            },
            I2cAction::Stop => {
                if isr.test_tc() {
                    self.i2c.with_cr2(|r| r.set_stop(1));                    
                    self.i2c.with_cr1(|r| r.set_stopie(1).set_tcie(0));
                } else if isr.test_stopf() {
                    self.i2c.with_cr1(|r| r.set_stopie(0).set_pe(0));
                    self.action.set(None)
                } else {
                    panic!("Unexpected Interrupt: {:?}", isr);
                }
            },
            _ => panic!("Poll in unexpected state: {:?}", action),
        }
        self.next();
    }
}
