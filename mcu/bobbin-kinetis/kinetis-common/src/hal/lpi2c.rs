pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::i2c::*;


use bobbin_common::{Irq, Poll};
use bobbin_common::ring::Ring;
use bobbin_cortexm::wfi;
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::hal::scb::*;

use bobbin_common::bits::*;

use core::cell::Cell;
use core::marker::PhantomData;


pub use ::chip::lpi2c::*;

impl Lpi2cPeriph {
    pub fn reset(&self) -> &Self {
        self.with_mcr(|r| r.set_rst(1));
        self.with_mcr(|r| r.set_rst(0));
        self
    }

    pub fn reset_receive_fifo(&self) -> &Self {
        self.with_mcr(|r| r.set_rrf(true))
    }

    pub fn reset_transmit_fifo(&self) -> &Self {
        self.with_mcr(|r| r.set_rtf(true))
    }

    pub fn bus_busy(&self) -> bool {
        self.msr().test_bbf()
    }

    pub fn master_busy(&self) -> bool {
        self.msr().test_mbf()
    }

    pub fn rdf(&self) -> bool {
        self.msr().test_rdf()
    }

    pub fn tdf(&self) -> bool {
        self.msr().test_tdf()
    }

    pub fn cmd_transmit(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b000).set_data(value))
    }

    pub fn cmd_receive(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b001).set_data(value))
    }

    pub fn cmd_stop(&self) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b010))
    }

    pub fn cmd_discard(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b011).set_data(value))
    }

    pub fn cmd_start(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b100).set_data(value))
    }

    pub fn cmd_start_nack(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b101).set_data(value))
    }

    pub fn cmd_start_hs(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b110).set_data(value))
    }
    
    pub fn cmd_start_hs_nack(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b111).set_data(value))
    }    
    
    pub fn rx_empty(&self) -> bool {
        self.mfsr().rxcount() == 0
    }

    pub fn receive(&self) -> u8 {
        self.mrdr().data().value()
    }

    pub fn wait_rdf(&self) -> &Self {
        while !self.msr().test_rdf() {}
        self
    }

    pub fn wait_tdf(&self) -> &Self {
        while !self.msr().test_tdf() {}
        self
    }
}

impl Enabled for Lpi2cPeriph {
    fn enabled(&self) -> bool {
        self.mcr().test_men()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.with_mcr(|r| r.set_men(value))
    }
}

impl I2cTransfer<u8> for Lpi2cPeriph {
    fn write(&self, addr: u8, tx_data: &[u8]) -> &Self {        
        self.wait_tdf().cmd_start(addr << 1);
        for b in tx_data.iter() {
            self.wait_tdf().cmd_transmit(*b);
        }
        self.wait_tdf().cmd_stop();
        self
    }

    fn read(&self, addr: u8, rx_data: &mut [u8]) -> &Self {
        self.wait_tdf().cmd_start(addr << 1 | 1);
        self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
        self.wait_tdf().cmd_stop();
        for i in 0..rx_data.len() {
            while self.rx_empty() {}
            rx_data[i] = self.receive();
        }
        self
    }

    fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
        self.wait_tdf().cmd_start(addr << 1);
        for b in tx_data.iter() {
            self.wait_tdf().cmd_transmit(*b);
        }
        self.wait_tdf().cmd_start(addr << 1 | 1);
        self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
        self.wait_tdf().cmd_stop();
        for i in 0..rx_data.len() {
            while self.rx_empty() {}
            rx_data[i] = self.receive();
        }        
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
    i2c: Lpi2cPeriph,
    action: Cell<Option<I2cAction>>,
    tx: Ring<'a, I2cAction>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for I2cDriver<'a> {}
unsafe impl<'a> Send for I2cDriver<'a> {}

impl<'a> I2cDriver<'a> {
    pub fn new<I: Into<Lpi2cPeriph>>(i2c: I, tx_buf: &'a mut [I2cAction], rx_buf: &'a mut [u8]) -> Self {
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

    pub fn write(&self, addr: U7, data: &[u8]) {
        let mut buf = [];
        self.transfer(addr, data, &mut buf);        
    }

    pub fn read(&self, addr: U7, buf: &mut [u8]) {
        self.transfer(addr, &[], buf);
    }


    pub fn transfer(&self, addr: U7, tx_buf: &[u8], rx_buf: &mut [u8]) {
        // println!("transfer: addr={:?} tx_buf={:?} rx_buf={:?}", addr, tx_buf, rx_buf);
        self.i2c.with_mfcr(|r| r.set_rxwater(0));
        
        
        self.i2c.with_mier(|r| r.set_feie(1).set_rdie(1));
        if tx_buf.len() > 0 {
            self.tx.enqueue(I2cAction::Start(addr.value() << 1));
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
                self.tx.enqueue(I2cAction::Start(addr.value() << 1 | 1));
            } else {
                self.tx.enqueue(I2cAction::Restart(addr.value() << 1 | 1));
            }
            self.tx.enqueue(I2cAction::ReadBytes(rx_buf.len() as u8));
            self.tx.enqueue(I2cAction::Stop);
        }
        self.next();
        loop {
            wfi();
            if self.rx.len() >= rx_buf.len() {
                if rx_buf.len() > 0 {
                    self.rx.read(rx_buf);
                }
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
                    I2cAction::Start(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::Restart(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::WriteBytes(_) => {},
                    I2cAction::WriteByte(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::ReadBytes(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::ReadByte(_) => {
                        panic!("Unexpected ReadByte in Tx Queue")
                    },
                    I2cAction::Stop => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));      
                    },
                }                
            } else {                
                return
            }
        }
    }
}

impl<'a> Poll for I2cDriver<'a> {
    fn poll(&self) {       
        let msr = self.i2c.msr();

        if msr.test_rdf() {
            loop {
                let mrdr = self.i2c.mrdr();
                if mrdr.test_rxempty() {
                    return
                } else {
                    let v = mrdr.data().value();
                    // println!("< {:02x}", v);
                    self.rx.enqueue(v);
                }
            }
        }

        if self.action().is_none() {
            panic!("MSR: {:?} MIER: {:?}", msr, self.i2c.mier());
        }

        if msr.test_fef() {
            panic!("FIFO Error");
        }

        let action = self.action().unwrap();
    

        match action {
            I2cAction::Start(addr) => {
                if msr.test_tdf() {
                    self.i2c.cmd_start(addr);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                    self.action.set(None);
                }
            },            
            I2cAction::Restart(addr) => {
                if msr.test_tdf() {
                    self.i2c.cmd_start(addr);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                    self.action.set(None);
                }
            },            
            I2cAction::WriteByte(n) => {
                if msr.test_tdf() {
                    self.i2c.cmd_transmit(n);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                    self.action.set(None);
                } 
            },
            I2cAction::ReadBytes(n) => {
                if msr.test_tdf() {
                    self.i2c.cmd_receive(n - 1);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                    self.action.set(None);
                }                
            }
            I2cAction::Stop => {
                if msr.test_sdf() {
                    self.i2c.with_msr(|r| r.set_sdf(1));
                    self.i2c.with_mier(|r| r.set_sdie(0));                    
                    self.action.set(None);
                } else if msr.test_tdf() && self.i2c.mier().test_tdie() {           
                    self.i2c.cmd_stop();                    
                    self.i2c.with_mier(|r| r.set_tdie(0).set_sdie(1));
                }
            },
            _ => panic!("Poll in unexpected state: {:?}", action),
        }

        self.next();
    }
}
