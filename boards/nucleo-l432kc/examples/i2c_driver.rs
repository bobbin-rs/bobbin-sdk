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
        .set_presc(0x2)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    let mut tx_buf = [0u8; 64];
    let mut rx_buf = [0u8; 64];

    let mut d = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);

    println!("ID: {:02x}", d.read_reg(addr_gyro, 0x0f));

    // d.transfer(addr_gyro, &[0x0f], 0x1);

    // loop {
    //     d.poll();
    //     if d.transfer_complete() {
    //         println!("Data: {:02x}", d.as_slice()[0]);
    //         d.clear();
    //         break;
    //     }
    // }



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
            // println!("{:?}", buf);
            let (xl, xh, yl, yh, zl, zh) = (
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5],
            );
            // let (xl, xh, yl, yh, zl, zh) = (
            //     d.read_reg(addr_gyro, 0x28),
            //     d.read_reg(addr_gyro, 0x29),
            //     d.read_reg(addr_gyro, 0x2a),
            //     d.read_reg(addr_gyro, 0x2b),
            //     d.read_reg(addr_gyro, 0x2c),
            //     d.read_reg(addr_gyro, 0x2d),
            // );
            // println!("{:?}", (xl, xh, yl, yh, zl, zh));
            let x = (((xh as u16) << 8) | (xl as u16)) as i16;
            let y = (((yh as u16) << 8) | (yl as u16)) as i16;
            let z = (((zh as u16) << 8) | (zl as u16)) as i16;
            print!("{:6} {:6} {:6}", x, y, z);
        }
        println!("");

        // let mut buf = [0u8; 6];
        // i2c.transfer(addr_gyro, &[0x20], &mut buf);
        // println!("# {:?}", buf);
        board::delay(500);
    }
}

pub struct I2cDriver<'a> {
    i2c: I2cPeriph,
    addr: Option<U7>,
    tx_buf: &'a mut [u8],
    tx_start: bool,
    tx_pos: usize,
    tx_len: usize,
    rx_buf: &'a mut [u8],
    rx_start: bool,
    rx_pos: usize,
    rx_len: usize,
}

impl<'a> I2cDriver<'a> {
    pub fn new<I: Into<I2cPeriph>>(i2c: I, tx_buf: &'a mut [u8], rx_buf: &'a mut [u8]) -> Self {
        I2cDriver { 
            i2c: i2c.into(),
            addr: None,
            tx_buf: tx_buf,
            tx_start: false,
            tx_pos: 0,
            tx_len: 0,
            rx_buf: rx_buf,
            rx_start: false,
            rx_pos: 0,
            rx_len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.addr = None;
        for b in self.tx_buf.iter_mut() {
            *b = 0;
        }
        self.tx_start = false;
        self.tx_pos = 0;
        self.tx_len = 0;

        for b in self.rx_buf.iter_mut() {
            *b = 0;
        }
        self.rx_start = false;
        self.tx_pos = 0;
        self.tx_len = 0;
    }

    pub fn read_reg(&mut self, addr: U7, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(addr, &[reg], &mut buf);
        buf[0]
    }

    pub fn write_reg(&mut self, addr: U7, reg: u8, value: u8) {
        let mut buf = [];
        self.transfer(addr, &[reg, value], &mut buf);
    }


    pub fn transfer(&mut self, addr: U7, tx_buf: &[u8], rx_buf: &mut [u8]) {
        self.start_transfer(addr, tx_buf, rx_buf.len());
        loop {
            self.poll();
            if self.transfer_complete() {
                rx_buf.copy_from_slice(self.as_slice());
                self.clear();
                break
            }
        }
    }

    pub fn start_transfer(&mut self, addr: U7, tx_buf: &[u8], rx_len: usize) {
        self.clear();
        self.addr = Some(addr);
        &self.tx_buf[..tx_buf.len()].copy_from_slice(tx_buf);
        self.tx_pos = 0;
        self.tx_len = tx_buf.len();        
        self.rx_pos = 0;
        self.rx_len = rx_len;
        self.i2c.with_cr1(|r| r.set_txie(1).set_rxie(1).set_pe(1));
    }

    pub fn transfer_complete(&self) -> bool {
        // self.tx_len == self.tx_pos && self.rx_len == self.rx_pos
        self.addr.is_none()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.rx_buf[..self.rx_len]
    }

    pub fn poll(&mut self) {
        if self.addr.is_none() {
            return;
        }
        let addr = self.addr.unwrap();
        if (self.tx_len - self.tx_pos) > 0 {
            if !self.tx_start {
                self.i2c.with_cr2(|r| r
                        .set_sadd(addr.value() << 1)
                        .set_rd_wrn(0)
                        .set_nbytes(self.tx_len)
                        .set_autoend(self.rx_len == 0)
                );
                self.i2c.with_cr2(|r| r.set_start(1));                
                self.tx_start = true;
            } else {
                if self.i2c.isr().txis() != 0 {
                    self.i2c.set_txdr(|r| r.set_txdata(self.tx_buf[self.tx_pos]));
                    self.tx_pos += 1;
                }
            }
        } else if (self.rx_len - self.rx_pos) > 0 {            
            if !self.rx_start {
                if self.i2c.isr().tc() != 0 {                
                    self.i2c.with_cr2(|r| r
                            .set_sadd(addr.value() << 1)
                            .set_rd_wrn(1)
                            .set_nbytes(self.rx_len)   
                    );
                    self.i2c.with_cr2(|r| r.set_start(1));               
                    self.i2c.with_cr2(|r| r.set_autoend(1));          
                    self.rx_start = true;
                }                
            } else {
                if self.i2c.isr().rxne() != 0 {
                    let d = self.i2c.rxdr().rxdata().value();
                    self.rx_buf[self.rx_pos] = d;
                    self.rx_pos += 1;
                }
            }
        } else {
            if self.addr.is_some() {                            
                if self.i2c.isr().stopf() != 0 {
                    self.i2c.with_cr1(|r| r.set_txie(0).set_rxie(0).set_pe(0));
                    self.addr = None;
                }
            }
        }
    }
}