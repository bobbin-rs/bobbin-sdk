pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::i2c::*;

// use bobbin_common::{Irq, Poll};
// use bobbin_common::ring::Ring;
// use bobbin_cortexm::wfi;
// use bobbin_cortexm::hal::nvic;
// use bobbin_cortexm::hal::scb::*;

// use bobbin_common::bits::*;

// use core::cell::Cell;
// use core::marker::PhantomData;

pub use ::periph::i2c::*;

impl I2cPeriph {
    pub fn init(&self, mult: u8, icr: u8) -> &Self {
        self.set_f(|_| F(0).set_mult(mult).set_icr(icr));
        self.with_c1(|r| r.set_iicen(1));
        self
    }

    pub fn data(&self) -> u8 {
        self.d().data().value()
    }

    pub fn set_data(&self, d: u8) {
        self.set_d(|_| D(0).set_data(d));
    }

    pub fn set_tx(&self, value: bool) {
        self.with_c1(|r| r.set_tx(value));
    }

    pub fn set_txak(&self, value: bool) {
        self.with_c1(|r| r.set_txak(value));
    }    

    fn with_tx<F: FnOnce(&Self) -> &Self>(&self, f: F) -> &Self {
        // Wait while Busy
        while self.s().busy() != 0 {}
        // Send Start
        self.with_c1(|r| r.set_mst(1).set_tx(1));
        f(&self);
        // Send Stop
        self.with_c1(|r| r.set_mst(0).set_tx(0));        
        // Wait while Busy
        while self.s().busy() != 0 {}
        self
    }

    fn restart(&self) -> &Self {
        self.with_c1(|r| r.set_tx(1).set_rsta(1));
        self                     
    }

    fn wait_transfer(&self) -> &Self {
        while self.s().iicif() == 0 {}
        self.with_s(|r| r.set_iicif(1));
        self
    }

    fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
        self.set_data(addr << 1);
        self.wait_transfer();
        let mut n = bytes.len();
        let mut i = 0;
        while n > 0 {        
            self.set_data(bytes[i]);
            self.wait_transfer();
            i += 1;
            n -= 1;
        }
        self
    }

    pub fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
        self.set_data(addr << 1 | 1);
        self.wait_transfer();
        self.set_tx(false);
        if bytes.len() == 1 {
            self.set_txak(true);
        } else {
            self.set_txak(false);
        }
        
        let _ = self.data();        
        let mut n = bytes.len();
        let mut i = 0;
        loop {
            self.wait_transfer();
            match n {
                1 => {
                    self.set_tx(true);
                    bytes[i] = self.data();
                    return self
                },
                2 => {
                    self.set_txak(true);
                    bytes[i] = self.data();
                    i += 1;
                },
                _ => {
                    bytes[i] = self.data();
                    i += 1;
                }
            }
            n -= 1;
        }
    }

    pub fn reg_write(&self, addr: u8, reg: u8, value: u8) -> &Self {
        let cmd = [reg, value];
        self.with_tx(|i| i.write(addr, &cmd))
    }

    pub fn reg_read(&self, addr: u8, reg: u8) -> u8 {
        let cmd = [reg];
        let mut buf = [0u8];
        self.with_tx(|i| i.write(addr, &cmd).restart().read(addr, &mut buf));
        buf[0]
    }
}

impl I2cTransfer<u8> for I2cPeriph {
    fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
        if tx_data.len() == 0 {            
            self.with_tx(|i| i.read(addr, rx_data))
        } else if rx_data.len() == 0 {
            self.with_tx(|i| i.write(addr, tx_data))
        } else {
            self.with_tx(|i| i.write(addr, tx_data).restart().read(addr, rx_data))
        }
    }

    fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
        self.with_tx(|i| i.write(addr, bytes))
    }

    fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
        self.with_tx(|i| i.read(addr, bytes))
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

// pub struct I2cDriver<'a> {
//     i2c: I2cPeriph,
//     action: Cell<Option<I2cAction>>,
//     tx: Ring<'a, I2cAction>,
//     rx: Ring<'a, u8>,
//     _phantom: PhantomData<&'a mut [u8]>,
// }

// unsafe impl<'a> Sync for I2cDriver<'a> {}
// unsafe impl<'a> Send for I2cDriver<'a> {}

// impl<'a> I2cDriver<'a> {
//     pub fn new<I: Into<I2cPeriph>>(i2c: I, tx_buf: &'a mut [I2cAction], rx_buf: &'a mut [u8]) -> Self {
//         I2cDriver { 
//             i2c: i2c.into(),
//             action: Cell::new(None),
//             tx: Ring::new(tx_buf),
//             rx: Ring::new(rx_buf),
//             _phantom: PhantomData,
//         }
//     }
//     pub fn enable_irq<I: Irq>(&self, irq: &I) {
//         nvic::set_enabled(irq.irq_num() as usize, false);
//         SCB.set_irq_handler(irq.irq_num() as usize, None);
//         SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
//         nvic::set_enabled(irq.irq_num() as usize, true);
//     }

//     pub fn action(&self) -> Option<I2cAction> {
//         self.action.get()
//     }

//     pub fn reg_read(&self, addr: U7, reg: u8) -> u8 {
//         let mut buf = [0u8];
//         self.transfer(addr, &[reg], &mut buf);
//         buf[0]
//     }

//     pub fn reg_write(&self, addr: U7, reg: u8, value: u8) {
//         let mut buf = [];
//         self.transfer(addr, &[reg, value], &mut buf);
//     }

//     pub fn write(&self, addr: U7, data: &[u8]) {
//         let mut buf = [];
//         self.transfer(addr, data, &mut buf);        
//     }

//     pub fn read(&self, addr: U7, buf: &mut [u8]) {
//         self.transfer(addr, &[], buf);
//     }


//     pub fn transfer(&self, addr: U7, tx_buf: &[u8], rx_buf: &mut [u8]) {
//         // println!("transfer: addr={:?} tx_buf={:?} rx_buf={:?}", addr, tx_buf, rx_buf);
//         if tx_buf.len() > 0 {
//             self.tx.enqueue(I2cAction::Start(addr.value() << 1));
//             self.tx.enqueue(I2cAction::WriteBytes(tx_buf.len() as u8));
//             for b in tx_buf.iter() {
//                 self.tx.enqueue(I2cAction::WriteByte(*b));
//             }
//             if rx_buf.len() == 0 {
//                 self.tx.enqueue(I2cAction::Stop);
//             }
//         }
//         if rx_buf.len() > 0 {
//             if tx_buf.len() == 0 {
//                 self.tx.enqueue(I2cAction::Start(addr.value() << 1 | 1));
//             } else {
//                 self.tx.enqueue(I2cAction::Restart(addr.value() << 1 | 1));
//             }
//             self.tx.enqueue(I2cAction::ReadBytes(rx_buf.len() as u8));
//             self.tx.enqueue(I2cAction::Stop);
//         }
//         self.next();
//         loop {
//             wfi();
//             if self.rx.len() >= rx_buf.len() {
//                 if rx_buf.len() > 0 {
//                     self.rx.read(rx_buf);
//                 }
//                 return
//             }
//         }
//     }
    

//     pub fn next(&self) {            
//         self.i2c.with_c1(|r| r.set_iicen(1));
//         loop {
//             // If currently processing an action, return without any changes
//             if self.action().is_some() { return }                    
//             // Get the next action off of the queue
//             if let Some(action) = self.tx.dequeue() {
//                 // println!("next: {:?}", action);                
//                 // board::delay(1);
//                 match action {
//                     I2cAction::Idle => {},
//                     I2cAction::Start(_) => {
//                         self.action.set(Some(action));
//                         self.i2c.with_flt(|r| r.set_startf(0).set_ssie(1));                        
//                         self.i2c.with_c1(|r| r.set_mst(1).set_tx(1).set_iicie(1));
//                     },
//                     I2cAction::Restart(_) => {
//                         self.action.set(Some(action));
//                         self.i2c.with_c1(|r| r.set_iicie(1));                     
//                     },
//                     I2cAction::WriteBytes(_) => {},
//                     I2cAction::WriteByte(_) => {   
//                         self.action.set(Some(action));
//                         self.i2c.with_c1(|r| r.set_iicie(1));                     
//                     },
//                     I2cAction::ReadBytes(_) => {
//                         self.action.set(Some(action));
//                         self.i2c.with_c1(|r| r.set_iicie(1));                     
//                     },
//                     I2cAction::ReadByte(_) => {
//                         panic!("Unexpected ReadByte in Tx Queue")
//                     },
//                     I2cAction::Stop => {
//                         self.action.set(Some(action));
//                         self.i2c.with_c1(|r| r.set_iicie(1));                     
//                     },
//                 }                
//             } else {                
//                 // self.i2c.with_flt(|r| r.set_ssie(0));
//                 // self.i2c.with_c1(|r| r.set_iicie(0));                
//                 return
//             }
//         }
//     }
// }

// impl<'a> Poll for I2cDriver<'a> {
//     fn poll(&self) {       
//         let action = self.action();

//         let s = self.i2c.s();

//         self.i2c.set_s(|r| r.set_iicif(1));
        
//         // println!("S: {:?} FLT: {:?} C1: {:?} Action: {:?}", s, flt, c1, action);
//         self.i2c.set_s(|r| r.set_iicif(true));
//         if action.is_none() { 
//             panic!("ACTION = NONE");
//         }
//         let action = action.unwrap();
//         match action {
//             I2cAction::Start(addr) => {
//                 let flt = self.i2c.flt();
//                 if flt.test_startf() {
//                     self.i2c.with_flt(|r| r.set_startf(1).set_ssie(0));
//                     self.i2c.with_d(|r| r.set_data(addr));
//                     self.i2c.with_c1(|r| r.set_iicie(0));
//                     self.action.set(None);
//                 } 
//             },
//             I2cAction::Restart(addr) => {
//                 let flt = self.i2c.flt();                
//                 if flt.test_startf() {
//                     self.i2c.with_flt(|r| r.set_startf(1).set_ssie(0));
//                     self.i2c.with_d(|r| r.set_data(addr));
//                     self.i2c.with_c1(|r| r.set_iicie(0));                    
//                     self.action.set(None);
//                 } else {
//                     self.i2c.with_flt(|r| r.set_startf(0).set_ssie(1));                        
//                     self.i2c.with_c1(|r| r.set_rsta(1).set_tx(1));
//                 }
//             },            
//             I2cAction::WriteByte(data) => {
//                 if s.test_tcf() {
//                     self.i2c.with_d(|r| r.set_data(data));
//                     self.i2c.with_c1(|r| r.set_iicie(0));                    
//                     self.action.set(None);
//                 }
//             },
//             I2cAction::ReadBytes(n) => {
//                 if s.test_tcf() {
//                     self.i2c.with_c1(|r| r.set_tx(0).set_txak(n == 1));
//                     let _ = self.i2c.data();
//                     self.action.set(Some(I2cAction::ReadByte(n-1)));
//                 }
//             }
//             I2cAction::ReadByte(n) => {
//                 if s.test_tcf() {
//                     match n {
//                         1 => {
//                             // self.i2c.set_tx(true);
//                             self.rx.enqueue(self.i2c.d().data().value());                                                        
//                         },
//                         2 => {
//                             self.i2c.set_txak(true);
//                             self.rx.enqueue(self.i2c.d().data().value());                            
//                         },
//                         _ => {
//                             self.rx.enqueue(self.i2c.d().data().value());                                                    }
//                     }                    
                    
//                     if n > 0 {
//                         self.action.set(Some(I2cAction::ReadByte(n - 1)));
//                     } else {
//                         self.action.set(None);
//                     }
//                 }
//             },
//             I2cAction::Stop => {
//                 let flt = self.i2c.flt();
//                 if flt.test_stopf() {
//                     self.i2c.with_flt(|r| r.set_stopf(1).set_ssie(0));
//                     self.i2c.with_c1(|r| r.set_mst(0).set_iicie(0));
//                     self.action.set(None)
//                 } else if s.test_tcf() {
//                     self.i2c.with_flt(|r| r.set_ssie(1));
//                     self.i2c.with_c1(|r| r.set_mst(0));
//                 }        
//             },            

//             _ => unimplemented!()
//         }
//         self.next();
//         // board::delay(1);
//     }
// }
