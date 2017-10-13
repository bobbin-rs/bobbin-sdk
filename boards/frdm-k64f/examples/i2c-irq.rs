#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::i2c::*;
use board::hal::port::*;
use board::hal::gpio::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running I2c Driver");

    // FXOS8700CQ Accelerometer + Magnetometer

    // I2C = I2C0
    // SCL = PTE24
    // SDA = PTE25
    // I2C_ADDR: 0x1D

    pub const I2C_ADDR: u8 = 0x1d;

    PORTE.sim_enable();
    I2C0.sim_enable();

    let i2c = I2C0;
    // Reset hung I2C Bus
    { 
        PTE24.set_mux_gpio();
        PTE25.set_mux_gpio();

        for _ in 0..10 {
            PTE24.gpio_pin().set_dir_output().toggle_output();
            PTE25.gpio_pin().set_dir_output().toggle_output();
            board::delay(10);
        }
    }

    let _i2c_scl = PTE24.mode_i2c_scl(&i2c).set_pull_none().set_ode(true);
    let _i2c_sda = PTE25.mode_i2c_sda(&i2c).set_pull_none().set_ode(true);
    
    let mult = 0;
    let icr = 0x1c;
    let addr = U7::from(I2C_ADDR);

    i2c.init(mult, icr);    
    println!("Configuring I2C Device");

    let mut tx_buf = [I2cAction::Idle; 64];
    let mut rx_buf = [0u8; 64];

    let i2c = I2cDriver::new(I2C0, &mut tx_buf, &mut rx_buf);
    i2c.enable_irq(&I2C0.irq_i2c());

    // Check WHO_AM_I @ 0x0d = 0xC7
    assert_eq!(i2c.reg_read(addr, 0x0d), 0xc7);

    println!("WHO_AMI_:       0x{:02x} = 0xc7", i2c.reg_read(addr, 0x0d));

    // Standby Mode
    i2c.reg_write(addr, 0x2A, 0x00);

    println!("CTRL_REG1:      0x{:02x} = 0x00", i2c.reg_read(addr, 0x2a));

    // write 0001 1111 = 0x1F to magnetometer control register 1
    i2c.reg_write(addr, 0x5B, 0x1f);
    println!("MAG_CTRL_REG1:  0x{:02x} = 0x1f", i2c.reg_read(addr, 0x5b));

    // write 0010 0000 = 0x20 to magnetometer control register 2
    i2c.reg_write(addr, 0x5C, 0x20);
    println!("MAG_CTRL_REG2:  0x{:02x} = 0x20", i2c.reg_read(addr, 0x5c));

    // write 0000 0001 = 0x01 to XYZ_DATA_CFG register
    i2c.reg_write(addr, 0x0E, 0x01);
    println!("XYZ_DATA_CFG:   0x{:02x} = 0x01", i2c.reg_read(addr, 0x0e));

    // write 0000 1101 = 0x0D to accelerometer control register 1
    i2c.reg_write(addr, 0x2A, 0x0D);
    println!("CTRL_REG1:      0x{:02x} = 0x0d", i2c.reg_read(addr, 0x2a));

    let mut cfg = [0u8; 5];
    i2c.transfer(addr, &[0x2a], &mut cfg);
    for b in cfg.iter() {
        print!("{:02x}", b);
    }
    println!("");

    println!("I2C Configured");    
    let mut buf = [0u8; 17];
    loop {
        i2c.transfer(addr, &[0x00], &mut buf);
        for b in buf.iter() {
            print!("{:02x} ", b);
        }
        let data = MagAccelData::from(buf);
        println!("| {:?}", data);

        board::delay(500);
    }
}

#[derive(Debug)]
pub struct MagAccelData {
    ax: i16,
    ay: i16,
    az: i16,
    mx: i16,
    my: i16,
    mz: i16,
}

impl From<[u8; 17]> for MagAccelData {
    fn from(other: [u8; 17]) -> Self {
        MagAccelData {
            ax: (((other[1] as u16) << 8 | (other[2] as u16)) as i16) >> 2,
            ay: (((other[3] as u16) << 8 | (other[4] as u16)) as i16) >> 2,
            az: (((other[5] as u16) << 8 | (other[6] as u16)) as i16) >> 2,
            mx: ((other[7] as u16) << 8 | (other[8] as u16)) as i16,
            my: ((other[9] as u16) << 8 | (other[10] as u16)) as i16,
            mz: ((other[11] as u16) << 8 | (other[12] as u16)) as i16,
        }
    }
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
        nvic::set_enabled(irq.irq_num() as usize, false);
        SCB.set_irq_handler(irq.irq_num() as usize, None);
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn action(&self) -> Option<I2cAction> {
        self.action.get()
    }

    pub fn reg_read(&self, addr: U7, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(addr, &[reg], &mut buf);
        buf[0]
    }

    pub fn reg_write(&self, addr: U7, reg: u8, value: u8) {
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
        self.i2c.with_c1(|r| r.set_iicen(1));
        loop {
            // If currently processing an action, return without any changes
            if self.action().is_some() { return }                    
            // Get the next action off of the queue
            if let Some(action) = self.tx.dequeue() {
                // println!("next: {:?}", action);                
                // board::delay(1);
                match action {
                    I2cAction::Idle => {},
                    I2cAction::Start(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_flt(|r| r.set_startf(0).set_ssie(1));                        
                        self.i2c.with_c1(|r| r.set_mst(1).set_tx(1).set_iicie(1));
                    },
                    I2cAction::Restart(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_c1(|r| r.set_iicie(1));                     
                    },
                    I2cAction::WriteBytes(_) => {},
                    I2cAction::WriteByte(_) => {   
                        self.action.set(Some(action));
                        self.i2c.with_c1(|r| r.set_iicie(1));                     
                    },
                    I2cAction::ReadBytes(_) => {
                        self.action.set(Some(action));
                        self.i2c.with_c1(|r| r.set_iicie(1));                     
                    },
                    I2cAction::ReadByte(_) => {
                        panic!("Unexpected ReadByte in Tx Queue")
                    },
                    I2cAction::Stop => {
                        self.action.set(Some(action));
                        self.i2c.with_c1(|r| r.set_iicie(1));                     
                    },
                }                
            } else {                
                // self.i2c.with_flt(|r| r.set_ssie(0));
                // self.i2c.with_c1(|r| r.set_iicen(0).set_iicie(0));                
                return
            }
        }
    }
}

impl<'a> Poll for I2cDriver<'a> {
    fn poll(&self) {       
        let action = self.action();

        let c1 = self.i2c.c1();
        let s = self.i2c.s();
        let flt = self.i2c.flt();

        self.i2c.set_s(|r| r.set_iicif(1));
        
        // println!("S: {:?} FLT: {:?} C1: {:?} Action: {:?}", s, flt, c1, action);
        self.i2c.set_s(|r| r.set_iicif(true));
        if action.is_none() { 
            println!("ACTION=NONE");
            loop {}
        }
        let action = action.unwrap();
        match action {
            I2cAction::Start(addr) => {
                if flt.test_startf() {
                    self.i2c.with_flt(|r| r.set_startf(1).set_ssie(0));
                    self.i2c.with_d(|r| r.set_data(addr));
                    self.i2c.with_c1(|r| r.set_iicie(0));
                    self.action.set(None);
                } 
            },
            I2cAction::Restart(addr) => {
                if flt.test_startf() {
                    self.i2c.with_flt(|r| r.set_startf(1).set_ssie(0));
                    self.i2c.with_d(|r| r.set_data(addr));
                    self.i2c.with_c1(|r| r.set_iicie(0));                    
                    self.action.set(None);
                } else {
                    self.i2c.with_flt(|r| r.set_startf(0).set_ssie(1));                        
                    self.i2c.with_c1(|r| r.set_rsta(1).set_tx(1));
                }
            },            
            I2cAction::WriteByte(data) => {
                if s.test_tcf() {
                    self.i2c.with_d(|r| r.set_data(data));
                    self.i2c.with_c1(|r| r.set_iicie(0));                    
                    self.action.set(None);
                }
            },
            I2cAction::ReadBytes(n) => {
                if s.test_tcf() {
                    self.i2c.with_c1(|r| r.set_tx(0).set_txak(n == 1));
                    let _ = self.i2c.data();
                    self.action.set(Some(I2cAction::ReadByte(n-1)));
                }
            }
            I2cAction::ReadByte(n) => {
                if s.test_tcf() {
                    match n {
                        1 => {
                            // self.i2c.set_tx(true);
                            self.rx.enqueue(self.i2c.d().data().value());                                                        
                        },
                        2 => {
                            self.i2c.set_txak(true);
                            self.rx.enqueue(self.i2c.d().data().value());                            
                        },
                        _ => {
                            self.rx.enqueue(self.i2c.d().data().value());                                                    }
                    }                    
                    
                    if n > 0 {
                        self.action.set(Some(I2cAction::ReadByte(n - 1)));
                    } else {
                        self.action.set(None);
                    }
                }
            },
            I2cAction::Stop => {
                if flt.test_stopf() {
                    self.i2c.with_flt(|r| r.set_stopf(1).set_ssie(0));
                    self.i2c.with_c1(|r| r.set_mst(0).set_iicie(0));
                    self.action.set(None)
                } else if s.test_tcf() {
                    self.i2c.with_flt(|r| r.set_ssie(1));
                    self.i2c.with_c1(|r| r.set_mst(0));
                }        
            },            

            _ => unimplemented!()
        }
        self.next();
        // board::delay(1);
    }
}
