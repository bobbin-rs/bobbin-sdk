#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::i2c::*;
use board::hal::gpio::*;
use board::common::bits::*;
use board::common::Poll;
use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
// use core::slice;

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
        .set_presc(0x0)
        .set_scldel(0x3)
        .set_sdadel(0x0)
        .set_sclh(0xF)
        .set_scll(0x12)
    );

    let mut tx_buf = [0u8; 64];
    let mut rx_buf = [0u8; 64];

    let d = I2cDriver::new(i2c, &mut tx_buf, &mut rx_buf);

    use board::common::Irq;
    use board::hal::scb::SCB;
    use board::hal::nvic;

    let irq = i2c.irq_i2c_ev();
    println!("irq: {:?}", irq);
    SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(&d)));
    nvic::set_enabled(irq.irq_num() as usize, true);

    let irq_er = i2c.irq_i2c_er();
    SCB.set_irq_handler(irq_er.irq_num() as usize, Some(irq.wrap(&d)));
    nvic::set_enabled(irq_er.irq_num() as usize, true);


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
    done: UnsafeCell<bool>,
    addr: Cell<Option<U7>>,
    tx_buf: * mut [u8],
    tx_start: Cell<bool>,
    tx_pos: Cell<usize>,
    tx_len: Cell<usize>,
    rx_buf: * mut [u8],
    rx_start: Cell<bool>,
    rx_pos: Cell<usize>,
    rx_len: Cell<usize>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for I2cDriver<'a> {}
unsafe impl<'a> Send for I2cDriver<'a> {}

impl<'a> I2cDriver<'a> {
    pub fn new<I: Into<I2cPeriph>>(i2c: I, tx_buf: &'a mut [u8], rx_buf: &'a mut [u8]) -> Self {
        I2cDriver { 
            i2c: i2c.into(),
            done: UnsafeCell::new(false),
            addr: Cell::new(None),
            tx_buf: tx_buf,
            tx_start: Cell::new(false),
            tx_pos: Cell::new(0),
            tx_len: Cell::new(0),
            rx_buf: rx_buf,
            rx_start: Cell::new(false),
            rx_pos: Cell::new(0),
            rx_len: Cell::new(0),
            _phantom: PhantomData,
        }
    }

    pub fn tx(&self) -> &mut [u8] {        
        unsafe {
            core::slice::from_raw_parts_mut(self.tx_buf as *mut u8, self.tx_len.get())
        }
    }


    pub fn rx(&self) -> &mut [u8] {        
        unsafe {
            core::slice::from_raw_parts_mut(self.rx_buf as *mut u8, self.rx_len.get())
        }
    }


    pub fn clear(&self) {
        self.set_transfer_complete(false);
        self.addr.set(None);
        self.tx_start.set(false);
        self.tx_pos.set(0);
        self.tx_len.set(0);
        self.rx_start.set(false);
        self.rx_pos.set(0);
        self.rx_len.set(0);
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
        self.start_transfer(addr, tx_buf, rx_buf.len());        
        loop {
            // self.poll();
            if self.transfer_complete() {
                rx_buf.copy_from_slice(self.as_slice());
                self.clear();
                break
            }
            board::delay(100);
            
        }
    }

    pub fn start_transfer(&self, addr: U7, tx_buf: &[u8], rx_len: usize) {
        self.clear();        
        self.addr.set(Some(addr));
        self.tx_len.set(tx_buf.len());                
        &self.tx()[..tx_buf.len()].copy_from_slice(tx_buf);
        self.rx_len.set(rx_len);

        self.i2c.with_cr1(|r| r.set_pe(1));
        // println!("tx start");
        if tx_buf.len() > 0 {
            self.i2c.with_cr2(|r| r
                    .set_sadd(addr.value() << 1)
                    .set_rd_wrn(0)
                    .set_nbytes(self.tx_len.get())
                    .set_autoend(0)
                    // .set_autoend(self.rx_len.get() == 0)
            );
        } else {
            self.i2c.with_cr2(|r| r
                    .set_sadd(addr.value() << 1)
                    .set_rd_wrn(1)
                    .set_nbytes(self.tx_len.get())
                    .set_autoend(0)
                    // .set_autoend(self.rx_len.get() == 0)
            );
            
        }
        self.i2c.with_cr2(|r| r.set_start(1));                
        self.tx_start.set(true);
        self.i2c.with_cr1(|r| r.set_txie(1).set_rxie(1).set_tcie(1).set_stopie(1));
    }

    pub fn transfer_complete(&self) -> bool {
        unsafe {
            core::ptr::read_volatile(self.done.get())
        }
    }

    pub fn set_transfer_complete(&self, value: bool) {
        unsafe {
            core::ptr::write_volatile(self.done.get(), value);
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.rx()[..self.rx_len.get()]
    }
}

impl<'a> Poll for I2cDriver<'a> {
    fn poll(&self) {       
        let isr = self.i2c.isr();
        if isr.test_txis() {
            // println!("tx data");
            self.i2c.set_txdr(|r| r.set_txdata(self.tx()[self.tx_pos.get()]));
            self.tx_pos.set(self.tx_pos.get() + 1);
        } else if isr.test_rxne() {
            // println!("rx data");
            self.rx()[self.rx_pos.get()] = self.i2c.rxdr().rxdata().value();
            self.rx_pos.set(self.rx_pos.get() + 1);
        } else if isr.test_tc() {
            // println!("tc");
            let addr = self.addr.get().unwrap();
            if self.rx_len.get() - self.rx_pos.get() > 0 {
                self.i2c.with_cr2(|r| r
                        .set_sadd(addr.value() << 1)
                        .set_rd_wrn(1)
                        .set_nbytes(self.rx_len.get())   
                );
                self.i2c.with_cr2(|r| r.set_start(1));               
                self.i2c.with_cr2(|r| r.set_autoend(0));          
            } else {
                self.i2c.with_cr2(|r| r.set_stop(1));
            }
        } else if isr.test_stopf() {
            self.i2c.with_cr1(|r| r.set_txie(0).set_rxie(0).set_tcie(0).set_stopie(1).set_stopie(0).set_pe(0));
            self.set_transfer_complete(true);
        }
    }
}