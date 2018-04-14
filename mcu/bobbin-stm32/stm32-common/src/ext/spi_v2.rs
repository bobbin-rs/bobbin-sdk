pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::spi::*;
pub use periph::spi_v2::*;


// use bobbin_common::ring::Ring;
// use bobbin_common::{Irq, Poll};
// use bobbin_common::hal::digital::DigitalOutput;
// use bobbin_cortexm::wfi;
// use bobbin_cortexm::hal::nvic;
// use bobbin_cortexm::hal::scb::*;
// use periph::gpio::GpioPin;

// use core::cell::Cell;
// use core::marker::PhantomData;

use bobbin_common::bits::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataSize {
    Bits4 = 0b0011,
    Bits5 = 0b0100,
    Bits6 = 0b0101,
    Bits7 = 0b0110,
    Bits8 = 0b0111,
    Bits9 = 0b1000,
    Bits10 = 0b1001,
    Bits11 = 0b1010,
    Bits12 = 0b1011,
    Bits13 = 0b1100,
    Bits14 = 0b1101,
    Bits15 = 0b1110,
    Bits16 = 0b1111,
}

#[derive(Debug, Default)]
pub struct Config {    
    pub cr1: Cr1,
    pub cr2: Cr2,
}

impl Config {
    pub fn set_data_size(mut self, value: DataSize) -> Self {
        self.cr2 = self.cr2.set_ds(value as u8);
        self
    }

    pub fn baud_divider(self) -> U3 {
        self.cr1.br()
    }

    // Divide by 2^(n+1)
    pub fn set_baud_divider(mut self, value: U3) -> Self {
        self.cr1 = self.cr1.set_br(value);
        self
    }

    pub fn master(self) -> bool {
        self.cr1.test_mstr()
    }

    pub fn set_master(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_mstr(value);
        self
    }

    pub fn cpol(self) -> bool {
        self.cr1.test_cpol()
    }

    pub fn set_cpol(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_cpol(value);
        self
    }

    pub fn cpha(self) -> bool {
        self.cr1.test_cpha()
    }

    pub fn set_cpha(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_cpha(value);
        self
    }    
}

impl Configure<Config> for SpiPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: self.cr1(),
            cr2: self.cr2(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cr1(|_| cfg.cr1)
            .set_cr2(|_| cfg.cr2);
        self
    }
}

impl Enabled for SpiPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_spe()
    }
    
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_spe(value));
        self
    }
}

impl SpiOutputEnabled for SpiPeriph {
    fn output_enabled(&self) -> bool {
        self.cr2().test_ssoe()
    }
    
    fn set_output_enabled(&self, value: bool) -> &Self {
        self.with_cr2(|r| r.set_ssoe(value));
        self
    }
}

impl SpiBusy for SpiPeriph {
    fn busy(&self) -> bool {
        self.sr().test_bsy()
    }
}

impl SpiCanTx for SpiPeriph {
    fn can_tx(&self) -> bool {
        self.sr().test_txe()
    }
}

impl SpiTx<u8> for SpiPeriph {
    fn tx(&self, value: u8) -> &Self {
        self.set_dr8(|r| r.set_dr8(value));
        self
    }
}

impl SpiTx<u16> for SpiPeriph {
    fn tx(&self, value: u16) -> &Self {
        self.dr().set_dr(value);
        self
    }
}

impl SpiCanRx for SpiPeriph {
    fn can_rx(&self) -> bool {
        self.sr().test_rxne()
    }
}

impl SpiRx<u8> for SpiPeriph {
    fn rx(&self) -> u8 {
        self.dr8().dr8().value() as u8
    }
}

impl SpiRx<u16> for SpiPeriph {
    fn rx(&self) -> u16 {
        self.dr().dr().into()
    }
}

impl SpiRead for SpiPeriph {
    fn read(&self, rx: &mut[u8]) {
        for i in 0..rx.len() {
            while !self.can_tx() {}
            self.tx(0xffu8);
            while !self.can_rx() {}
            rx[i] = self.rx();
        }
    }
}

impl SpiWrite for SpiPeriph {
    fn write(&self, tx: &[u8]) {
        for i in 0..tx.len() {
            while !self.can_tx() {}
            self.tx(tx[i]);
            while !self.can_rx() {}
            let _: u8 = self.rx();
        }
    }
}

impl SpiTransfer for SpiPeriph {}

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

// pub struct SpiDriver<'a> {
//     spi: SpiPeriph,
//     pins: &'a [GpioPin],
//     repeat: Cell<u8>,
//     action: Cell<Option<SpiAction>>,
//     tx: Ring<'a, SpiAction>,
//     rx: Ring<'a, u8>,
//     _phantom: PhantomData<&'a mut [u8]>,
// }

// unsafe impl<'a> Sync for SpiDriver<'a> {}
// unsafe impl<'a> Send for SpiDriver<'a> {}

// impl<'a> SpiDriver<'a> {
//     pub fn new<P: Into<SpiPeriph>>(spi: P, pins: &'a [GpioPin], tx_buf: &'a mut [SpiAction], rx_buf: &'a mut [u8]) -> Self {
//         SpiDriver { 
//             spi: spi.into(),
//             pins: pins,
//             repeat: Cell::new(0),
//             action: Cell::new(None),
//             tx: Ring::new(tx_buf),
//             rx: Ring::new(rx_buf),
//             _phantom: PhantomData,
//         }
//     }

//     // pub fn enable_irq<I: Irq>(&self, irq: &I) {        
//     //     SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
//     //     nvic::set_enabled(irq.irq_num() as usize, true);
//     // }

//     pub fn enqueue(&self, action: SpiAction) {        
//         self.tx.enqueue(action);
//         self.next();
//     }

//     pub fn tx_len(&self) -> usize {
//         self.tx.len()
//     }

//     pub fn rx_len(&self) -> usize {
//         self.rx.len()
//     }

//     pub fn read(&self, buf: &mut [u8]) {
//         while self.rx.len() < buf.len() {
//             wfi()
//         }
//         self.rx.read(buf);
//     }

//     pub fn read_byte(&self) -> u8 {
//         while self.rx.len() == 0 {
//             wfi()
//         }
//         self.rx.dequeue().unwrap()
//     }

//     pub fn action(&self) -> Option<SpiAction> {
//         self.action.get()
//     }

//     pub fn next(&self) {
//         if self.action().is_none() {
//             loop {
//                 if let Some(action) = self.tx.dequeue() {
//                     match action {
//                         SpiAction::Start(pin) => {
//                             self.pins[pin as usize].set_output(false);
//                         },
//                         SpiAction::Write(b) | SpiAction::Transfer(b) => { 
//                             self.action.set(Some(action));
//                             self.spi.tx(b);
//                             self.transfer_enable();
//                             break;
//                         },
//                         SpiAction::Repeat(n) => {
//                             self.repeat.set(n);
//                         }
//                         SpiAction::Stop(pin) => {
//                             self.pins[pin as usize].set_output(true);
//                         },
//                         SpiAction::Idle => {}
//                     }
//                 } else {
//                     self.action.set(None);
//                     self.transfer_disable();
//                     break;
//                 }
//             }
//         }
//     }

//     pub fn reg_read(&self, pin: u8, reg: u8) -> u8 {
//         assert!(self.rx.len() == 0);
        
//         self.enqueue(SpiAction::Start(pin));
//         self.enqueue(SpiAction::Write(reg));
//         self.enqueue(SpiAction::Transfer(0x55));        
//         self.enqueue(SpiAction::Stop(pin));
//         while self.rx.len() == 0 {
//             wfi()
//         }
//         self.rx.dequeue().unwrap()
//     }

//     pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
//         self.enqueue(SpiAction::Start(pin));
//         self.enqueue(SpiAction::Write(reg));
//         self.enqueue(SpiAction::Write(value));
//         self.enqueue(SpiAction::Stop(pin));
//         while self.action().is_some() {
//             wfi()
//         }
//     }

//     pub fn transfer_start(&self, pin: u8) {
//         self.pins[pin as usize].set_output(false);
//     }

//     pub fn send_blocking(&self, tx_buf: &[u8]) {
//         for b in tx_buf.iter() {
//             while !self.spi.can_tx() {}
//             self.spi.tx(*b);
//             while !self.spi.can_rx() {}
//             let _: u8 = self.spi.rx();
//         }
//     }

//     pub fn recv_blocking(&self, rx_buf: &mut [u8]) {
//         for b in rx_buf.iter_mut() {
//             while !self.spi.can_tx() {}
//             self.spi.tx(0xffu8);
//             while !self.spi.can_rx() {}
//             *b = self.spi.rx();            
//         }
//     }

//     pub fn transfer_blocking(&self, pin: u8, tx_buf: &[u8], rx_buf: &mut [u8]) {
//         self.transfer_start(pin);
//         if tx_buf.len() > 0 {
//             self.send_blocking(tx_buf);
//         }
//         if rx_buf.len() > 0 {
//             self.recv_blocking(rx_buf);
//         }
//         self.transfer_end(pin);
//     }

//     pub fn transfer_end(&self, pin: u8) {
//         self.pins[pin as usize].set_output(true);
//     }


//     pub fn transfer(&self, pin: u8, tx_buf: &[u8], rx_buf: &mut [u8]) {
//         self.enqueue(SpiAction::Start(pin));        
//         for b in tx_buf.iter() {
//             self.enqueue(SpiAction::Write(*b));
//         }
//         if rx_buf.len() > 0 {
//             self.enqueue(SpiAction::Repeat((rx_buf.len() - 1) as u8));
//         }
//         self.enqueue(SpiAction::Transfer(0x55));
//         self.enqueue(SpiAction::Stop(pin));
//         self.read(rx_buf);
//     }

//     pub fn transfer_enable(&self) {        
//         self.spi.with_cr2(|r| r.set_rxneie(true));
//         self.spi.set_enabled(true);
//     }
    
//     pub fn transfer_disable(&self) {        
//         self.spi.with_cr2(|r| r.set_rxneie(false));
//     }
// }

// impl<'a> Poll for SpiDriver<'a> {
//     fn poll(&self) {       
//         let sr = self.spi.sr();
//         let action = self.action().unwrap();
//         let repeat = self.repeat.get();
//         // println!("SR: {:?} Action: {:?}", sr, self.action());
//         if sr.rxne() != 0 {
//             match action {
//                 SpiAction::Write(b) => { 
//                     let _: u8  = self.spi.rx(); 
//                     if repeat > 0 {
//                         self.repeat.set(repeat - 1);
//                         self.spi.tx(b);
//                     } else {
//                         self.action.set(None);
//                     }
//                 },
//                 SpiAction::Transfer(b) => { 
//                     self.rx.enqueue(self.spi.rx()); 
//                     if repeat > 0 { 
//                         self.repeat.set(repeat - 1);
//                         self.spi.tx(b);
//                     } else {
//                         self.action.set(None);
//                     }
//                 },
//                 _ => {},
//             }            
//             self.next();
//         }
//     }
// }
