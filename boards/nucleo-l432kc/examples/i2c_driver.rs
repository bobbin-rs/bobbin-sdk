#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;

// A5 = PB6 = I2C1_SCL
// A4 = PB7 = I2C1_SDA

// Address = 0x35

#[no_mangle]
pub extern "C" fn main() -> ! {
    // use board::hal::i2c::*;
    board::init();
    println!("Running I2C");
    
    let addr_gyro: U7 = U7::from(0x6B);
    let i2c = I2C1;
    let i2c_port = GPIOB;
    let i2c_scl = PB6; // A5
    let i2c_sda = PB7; // A4

    GPIOA.rcc_enable();
    PA6.mode_input().open_drain();
    PA5.mode_input().open_drain();

    i2c.rcc_enable();
    i2c_port.rcc_enable();

    i2c_scl.mode_i2c_scl(&i2c).open_drain();
    i2c_sda.mode_i2c_sda(&i2c).open_drain();

    println!("# Configuring I2C");

    // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
    i2c.set_enabled(false);
    // i2c.set_timingr(|_| Timingr(0x00300619));
    i2c.set_timingr(|r| r
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    let mut tx_buf = [I2cAction::Idle; 64];
    // let mut tx_buf = [0u8; 64];
    let mut rx_buf = [0u8; 64];

    let d = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);    
    d.enable_irq(&i2c.irq_i2c_ev());

    println!("I2C Configuration Complete");

    println!("Configuring Gyro");

    println!("ID:   {:02x}", d.read_reg(addr_gyro, 0x0f));
    println!("TEMP: {:02x}", d.read_reg(addr_gyro, 0x26));
    /* Reset then switch to normal mode and enable all three channels */
    d.write_reg(addr_gyro, 0x20, 0x00);
    d.write_reg(addr_gyro, 0x20, 0x0f);
    // for i in 0x20..0x25 {
    //     println!("{:02x}: {:02x}", i, i2c.read_reg(addr_gyro, i));
    // }

    loop {
        // println!("STATUS: {:02x}", i2c.read_reg(addr_gyro, 0x27));
        { 
            let mut buf = [0u8; 6];
            d.transfer(addr_gyro, &[0x28 | 0x80], &mut buf);
            let (xl, xh, yl, yh, zl, zh) = (
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5],
            );

            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            print!("{:6} {:6} {:6}", x, y, z);
        }
        println!("");

        board::delay(500);
    }
}

use board::common::{Irq, Poll};
use board::common::ring::Ring;
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::*;

use core::marker::PhantomData;
use core::cell::Cell;

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
            board::cortexm::wfi();
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

// impl<'a> Poll for I2cDriver<'a> {
//     fn poll(&self) {       
//         let isr = self.i2c.isr();
//         if isr.test_txis() {
//             // println!("tx data");
//             self.i2c.set_txdr(|r| r.set_txdata(self.tx()[self.tx_pos.get()]));
//             self.tx_pos.set(self.tx_pos.get() + 1);
//         } else if isr.test_rxne() {
//             // println!("rx data");
//             self.rx()[self.rx_pos.get()] = self.i2c.rxdr().rxdata().value();
//             self.rx.enqueue(self.i2c.rxdr().rxdata().value());
//             self.rx_pos.set(self.rx_pos.get() + 1);
//         } else if isr.test_tc() {
//             // println!("tc");
//             let addr = self.addr.get().unwrap();
//             if self.rx_len.get() - self.rx_pos.get() > 0 {
//                 self.i2c.with_cr2(|r| r
//                         .set_sadd(addr.value() << 1)
//                         .set_rd_wrn(1)
//                         .set_nbytes(self.rx_len.get())   
//                 );
//                 self.i2c.with_cr2(|r| r.set_start(1));               
//                 self.i2c.with_cr2(|r| r.set_autoend(0));          
//             } else {
//                 self.i2c.with_cr2(|r| r.set_stop(1));
//             }
//         } else if isr.test_stopf() {
//             self.i2c.with_cr1(|r| r.set_txie(0).set_rxie(0).set_tcie(0).set_stopie(1).set_stopie(0).set_pe(0));
//             self.set_transfer_complete(true);
//         }
//     }
// }