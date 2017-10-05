pub use bobbin_common::i2c::*;
pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use ::chip::i2c_v2::*;

use bobbin_common::bits::*;

use bobbin_common::{Irq, Poll};
use bobbin_cortexm::hal::nvic;
use bobbin_cortexm::hal::scb::*;

use core::ptr;
use core::slice;
use core::cell::{Cell, UnsafeCell};
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


pub struct I2cDriver<'a> {
    i2c: I2cPeriph,
    done: UnsafeCell<bool>,
    addr: Cell<Option<U7>>,
    tx_buf: * mut [u8],
    tx_pos: Cell<usize>,
    tx_len: Cell<usize>,
    rx_buf: * mut [u8],
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
            tx_pos: Cell::new(0),
            tx_len: Cell::new(0),
            rx_buf: rx_buf,
            rx_pos: Cell::new(0),
            rx_len: Cell::new(0),
            _phantom: PhantomData,
        }
    }
    pub fn enable_irq<I: Irq>(&self, irq: &I) {
        // self.irq_num = Some(irq.irq_num());
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn tx(&self) -> &mut [u8] {        
        unsafe {
            slice::from_raw_parts_mut(self.tx_buf as *mut u8, self.tx_len.get())
        }
    }


    pub fn rx(&self) -> &mut [u8] {        
        unsafe {
            slice::from_raw_parts_mut(self.rx_buf as *mut u8, self.rx_len.get())
        }
    }


    pub fn clear(&self) {
        self.set_transfer_complete(false);
        self.addr.set(None);
        self.tx_pos.set(0);
        self.tx_len.set(0);
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
            if self.transfer_complete() {
                rx_buf.copy_from_slice(self.as_slice());
                self.clear();
                break
            }            
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
        self.i2c.with_cr1(|r| r.set_txie(1).set_rxie(1).set_tcie(1).set_stopie(1));
    }

    pub fn transfer_complete(&self) -> bool {
        unsafe {
            ptr::read_volatile(self.done.get())
        }
    }

    pub fn set_transfer_complete(&self, value: bool) {
        unsafe {
            ptr::write_volatile(self.done.get(), value);
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