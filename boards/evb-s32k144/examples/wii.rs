#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::pcc;
use board::hal::lpi2c::*;
use board::hal::port::*;


// pub const i2c_addr: u8 = 0x52;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let i2c_addr = U7::from(0x52);    

    PORTA.pcc_enable();
    let i2c = LPI2C0;

    let port_scl = PTA3;
    let port_sda = PTA2;

    println!("# I2C WII Start");
    port_scl.mode_i2c_scl(&i2c).set_pull_none();
    port_sda.mode_i2c_sda(&i2c).set_pull_none();

    i2c
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .pcc_enable();

    {
        // let input_clock = i2c.clock(&CLK).unwrap();

        i2c.with_mcfgr0(|r| r.set_cirfifo(0));
        i2c.with_mcfgr1(|r| r.set_prescale(0x1));
        i2c.with_mcfgr2(|r| r.set_filtscl(1).set_filtsda(1).set_busidle(0xBA));        
        i2c.with_mccr0(|r| r.set_clklo(0x3e).set_clkhi(0x35).set_sethold(0x1d).set_datavd(0x0f));
        i2c.with_mfcr(|r| r.set_txwater(0x3).set_rxwater(0x3));
    }

    // Enable Master
    i2c.with_mcr(|r| r.set_men(true));

    println!("I2C Configured");

    let mut tx_buf = [I2cAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let s = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&i2c.irq_lpi2c_master());

    println!("Initialize");
    
    s.write_reg(i2c_addr, 0xf0, 0x55);
    board::delay(1);    
    s.write_reg(i2c_addr, 0xfb, 0x00);
    board::delay(1);
    
    println!("Identify");
    s.write(i2c_addr, &[0xfa]);
    board::delay(1);
    let mut buf = [0u8; 6];
    s.read(i2c_addr, &mut buf);
    for b in buf.iter() {
        print!("{:02x} ", b);
    }
    println!("");



    loop {}
}

use board::common::{Irq, Poll};
use board::common::ring::Ring;
use board::cortexm::wfi;
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::*;

use board::common::bits::*;

use core::cell::Cell;
use core::marker::PhantomData;


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
        // println!("commands queued");
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
                        // println!("mier: {:?}", self.i2c.mier());
                    },
                    I2cAction::Restart(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::WriteBytes(_) => {
                        // self.cmd_start(self.addr.get() << 1);
                    },
                    I2cAction::WriteByte(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::ReadBytes(_) => {
                        // self.i2c.with_cr2(|r| r.set_nbytes(n).set_rd_wrn(1));
                        // self.i2c.with_cr2(|r| r.set_start(1));
                        // self.i2c.with_cr1(|r| r.set_rxie(1));                        
                        // self.cmd_start(self.addr.get() << 1 | 1);
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));
                    },
                    I2cAction::ReadByte(_) => {
                        panic!("Unexpected ReadByte in Tx Queue")
                    },
                    I2cAction::Stop => {
                        // self.i2c.with_cr1(|r| r.set_tcie(1));     
                        self.action.set(Some(action));
                        self.i2c.with_mier(|r| r.set_tdie(1));      
                    },
                }                
            } else {                
                // self.i2c.with_cr1(|r| r.set_txie(0).set_rxie(0).set_tcie(0).set_stopie(0).set_pe(0));
                // println!("next_done");
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
            println!("MSR: {:?} MIER: {:?}", msr, self.i2c.mier());
        }

        if msr.test_fef() {
            println!("FEF");
            loop {}
        }

        let action = self.action().unwrap();
    

        match action {
            I2cAction::Start(addr) => {
                if msr.test_tdf() {
                    // println!("[ {:02x}", addr);
                    self.i2c.cmd_start(addr);
                    self.action.set(None);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                }
            },            
            I2cAction::Restart(addr) => {
                if msr.test_tdf() {
                    // println!("| {:02x}", addr);
                    self.i2c.cmd_start(addr);
                    self.action.set(None);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                }
            },            
            I2cAction::WriteBytes(_) => {
                unimplemented!()
            },            
            I2cAction::WriteByte(n) => {
                if msr.test_tdf() {
                    // println!("> {:02x}", n);
                    self.i2c.cmd_transmit(n);
                    self.action.set(None);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                } 
            },
            I2cAction::ReadBytes(n) => {
                if msr.test_tdf() {
                    self.i2c.cmd_receive(n - 1);
                    self.action.set(None);
                    self.i2c.with_mier(|r| r.set_tdie(0));
                }                
            }
            I2cAction::ReadByte(_) => {
                unimplemented!()
            },
            I2cAction::Stop => {
                if msr.test_sdf() {
                    // println!("stop done");
                    self.i2c.with_msr(|r| r.set_sdf(1));
                    self.action.set(None);
                    self.i2c.with_mier(|r| r.set_sdie(0));                    
                } else if msr.test_tdf() && self.i2c.mier().test_tdie() {           
                    // println!(".");
                    self.i2c.cmd_stop();                    
                    self.i2c.with_mier(|r| r.set_tdie(0).set_sdie(1));
                }
            },
            _ => panic!("Poll in unexpected state: {:?}", action),
        }

        self.next();
        // board::delay(100);
    }
}


// impl I2cTransfer<u8> for Lpi2cPeriph {
//     fn write(&self, addr: u8, tx_data: &[u8]) -> &Self {        
//         self.wait_tdf().cmd_start(addr << 1);
//         for b in tx_data.iter() {
//             self.wait_tdf().cmd_transmit(*b);
//         }
//         self.wait_tdf().cmd_stop();
//         self
//     }

//     fn read(&self, addr: u8, rx_data: &mut [u8]) -> &Self {
//         self.wait_tdf().cmd_start(addr << 1 | 1);
//         self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
//         self.wait_tdf().cmd_stop();
//         for i in 0..rx_data.len() {
//             while self.rx_empty() {}
//             rx_data[i] = self.receive();
//         }
//         self
//     }

//     fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
//         self.wait_tdf().cmd_start(addr << 1);
//         for b in tx_data.iter() {
//             self.wait_tdf().cmd_transmit(*b);
//         }
//         self.wait_tdf().cmd_start(addr << 1 | 1);
//         self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
//         self.wait_tdf().cmd_stop();
//         for i in 0..rx_data.len() {
//             while self.rx_empty() {}
//             rx_data[i] = self.receive();
//         }        
//         self
//     }
    
// }