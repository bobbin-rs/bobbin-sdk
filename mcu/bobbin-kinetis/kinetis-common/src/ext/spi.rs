pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::spi::*;

// use bobbin_common::ring::Ring;
// use bobbin_common::{Irq, Poll};
// use bobbin_common::digital::DigitalOutput;
// use bobbin_cortexm::wfi;
// use bobbin_cortexm::hal::nvic;
// use bobbin_cortexm::hal::scb::SCB;
// use ::hal::gpio::GpioPin;

// use core::cell::Cell;
// use core::marker::PhantomData;

pub use ::periph::spi::*;

impl SpiPeriph {
    pub fn init(&self, br: u8, pbr: u8) {
        // Halt and clear FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        self.with_mcr(|r| r.set_pcsis(0b1));
        self.with_mcr(|r| r.set_mdis(0).set_mstr(1));

        // Set configuration 0
        self.set_ctar(0, |_| Ctar(0)
            // .set_pcssck(0b01)
            // .set_pasc(0b01)
            .set_cssck(0b0100)
            .set_asc(0b0100)            
            .set_br(br)
            .set_pbr(pbr)
            .set_fmsz(7)
        );              
    }

    pub fn transfer(&self, bytes: &mut [u8]) {
        // Flush FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        // Clear status bits
        self.set_sr(|_| Sr(0).set_tcf(1).set_eoqf(1).set_tfuf(1).set_tfff(1).set_rfof(1).set_rfdf(1));
        self.with_mcr(|r| r.set_halt(0)); 
        let len = bytes.len();
        for i in 0..len {
            self.set_pushr(|_| Pushr(0).set_cont(1).set_ctas(0).set_txdata(bytes[i]).set_pcs(1));
            while self.sr().rfdf() == 0 {}
            bytes[i] = self.popr().rxdata().value() as u8;
            self.set_sr(|_| Sr(0).set_rfdf(1));
        }                        
        self.with_mcr(|r| r.set_halt(1)); 
        
    }

    pub fn reg_read(&self, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.write(&[reg]);   
        self.read(&mut buf);
        buf[0]
    }

}

impl SpiWrite for SpiPeriph {
    fn write(&self, bytes: &[u8]) {
        // Flush FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        
        // Clear status bits
        self.set_sr(|_| Sr(0)
            .set_tcf(1)
            .set_eoqf(1)
            .set_tfuf(1)
            .set_tfff(1)
            .set_rfof(1)
            .set_rfdf(1));
        self.with_mcr(|r| r.set_halt(0)); 

        let len = bytes.len();
        for i in 0..len {
            let last = if i == len - 1 { 1 } else { 0 };

            // NOTE
            // TFFF flag clears automatically when DMA is used to fill TX FIFO.
            // To clear TFFF when not using DMA, follow these steps for every PUSH performed using CPU to fill TX FIFO:
            // 1. Wait until TFFF = 1.
            // 2. Write data to PUSHR using CPU.
            // 3. Clear TFFF by writing a 1 to its location. If TX FIFO is not
            // full, this flag will not clear.

            while self.sr().tfff() == 0 {}                
            self.set_pushr(|_| Pushr(0).set_cont(1).set_ctas(0).set_eoq(last).set_txdata(bytes[i]).set_pcs(1));
            self.set_sr(|_| Sr(0).set_tfff(1));
        }            
        while self.sr().eoqf() == 0 {}
        self.set_sr(|_| Sr(0).set_eoqf(1));
        self.with_mcr(|r| r.set_halt(1));
    }
}
impl SpiRead for SpiPeriph {
    fn read(&self, bytes: &mut [u8]) {
        // Flush FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        // Clear status bits
        self.set_sr(|_| Sr(0).set_tcf(1).set_eoqf(1).set_tfuf(1).set_tfff(1).set_rfof(1).set_rfdf(1));
        self.with_mcr(|r| r.set_halt(0)); 
        let len = bytes.len();
        for i in 0..len {
            self.set_pushr(|_| Pushr(0).set_cont(1).set_ctas(0).set_txdata(0xff).set_pcs(1));
            while self.sr().rfdf() == 0 {}
            bytes[i] = self.popr().rxdata().value() as u8;
            self.set_sr(|_| Sr(0).set_rfdf(1));
        }                        
        self.with_mcr(|r| r.set_halt(1)); 
    }
}

// impl SpiTransfer for SpiPeriph {
//     fn transfer(&self, tx_buf: &[u8], rx_buf: &mut [u8]) {
//         self.write(tx_buf);
//         self.read(rx_buf);
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum SpiAction {
//     Idle,
//     Start(u8),
//     Repeat(u8),
//     Write(u8),
//     Transfer(u8),
//     Stop(u8),
// }

// impl Default for SpiAction {
//     fn default() -> Self {
//         SpiAction::Idle
//     }
// }

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

//     pub fn enable_irq<I: Irq>(&self, irq: &I) {        
//         nvic::set_enabled(irq.irq_num() as usize, false);
//         SCB.set_irq_handler(irq.irq_num() as usize, None);
//         SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
//         nvic::set_enabled(irq.irq_num() as usize, true);
//     }

//     pub fn enqueue(&self, action: SpiAction) {        
//         self.tx.enqueue(action);
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



//     pub fn reg_read(&self, pin: u8, reg: u8) -> u8 {
//         // println!("reg_read: {} {:02x}", pin, reg);
//         self.enqueue(SpiAction::Start(pin));
//         self.enqueue(SpiAction::Write(reg));
//         self.enqueue(SpiAction::Transfer(0x55));        
//         self.enqueue(SpiAction::Stop(pin));
//         self.next();

//         while self.rx.len() == 0 {
//             // wfi()
//         }
//         self.rx.dequeue().unwrap()
//     }

//     pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
//         self.enqueue(SpiAction::Start(pin));
//         self.enqueue(SpiAction::Write(reg));
//         self.enqueue(SpiAction::Write(value));
//         self.enqueue(SpiAction::Stop(pin));
//         self.next();
//         while self.action().is_some() {
//             wfi()
//         }
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
//         self.next();
//         self.read(rx_buf);
//     }

//     pub fn transfer_enable(&self) {        
//         // self.spi.with_ier(|r| r.set_tdie(true));
//         // self.spi.set_enabled(true);
//     }
    
//     pub fn transfer_disable(&self) {        
//         // self.spi.set_ier(|r| r);
//         // self.spi.set_enabled(false);
//     }
    
//     pub fn tx(&self, value: u8) {
//         self.spi.set_pushr(|_| Pushr(0).set_txdata(value));

//     }

//     pub fn rx(&self) -> u8 {
//         self.spi.popr().rxdata().value() as u8
//     }

//     pub fn next(&self) {
//         if self.action().is_none() {
//             loop {
//                 if let Some(action) = self.tx.dequeue() {
//                     // println!("next: {:?}", action);
//                     match action {
//                         SpiAction::Start(pin) => {   
//                             self.spi.with_mcr(|r| r.set_mdis(0).set_halt(1));                                                  
//                             self.pins[pin as usize].set_output(false);
//                             self.spi.with_mcr(|r| r.set_halt(0));
//                         },
//                         SpiAction::Write(b) | SpiAction::Transfer(b) => { 
//                             self.action.set(Some(action));     
//                             self.tx(b);
//                             self.spi.with_rser(|r| r.set_tcf_re(1));
         
//                             // self.spi.with_rser(|r| r.set_tfff_re(1).set_rfdf_re(1));
//                             break;

//                         },
//                         SpiAction::Repeat(n) => {
//                             self.repeat.set(n);
//                         }
//                         SpiAction::Stop(pin) => {
//                             // self.spi.with_mcr(|r| r.set_mdis(0).set_halt(1));
//                             self.pins[pin as usize].set_output(true);

//                             // self.spi.set_enabled(false);                            
//                         },
//                         SpiAction::Idle => {}
//                     }
//                 } else {
//                     // println!("Loop Done");
//                     self.action.set(None);
//                     // self.transfer_disable();
//                     break;
//                 }
//             }
//         }
//     }
// }

// impl<'a> Poll for SpiDriver<'a> {
//     fn poll(&self) {       
//         let sr = self.spi.sr();
//         let action = self.action();
//         let repeat = self.repeat.get();
//         // println!("SR: {:?} Action: {:?}", sr, self.action());        
//         // board::delay(1);
//         let action = action.unwrap();
//         match action {
//             SpiAction::Write(b) => { 
//                 if sr.test_tcf() {
//                     self.spi.with_sr(|r| r.set_tcf(1));
//                     let _: u8  = self.rx(); 
//                     if repeat > 0 {
//                         self.tx(b);
//                         self.repeat.set(repeat - 1);
//                     } else {
//                         self.spi.with_rser(|r| r.set_tcf_re(0));
//                         self.action.set(None);
//                     }
//                 }
//             },
//             SpiAction::Transfer(b) => { 
//                 if sr.test_tcf() {
//                     self.spi.with_sr(|r| r.set_rfdf(1));
//                     self.rx.enqueue(self.rx()); 
//                     if repeat > 0 { 
//                         self.tx(b);
//                         self.repeat.set(repeat - 1);
//                     } else {
//                         self.spi.with_rser(|r| r.set_tcf_re(0));
//                         self.action.set(None);
//                     }
//                 }
//             },

//             _ => {},
//         }            
//         self.next();
//         // println!(".");
//         // board::delay(100);
//     }
// }
