pub use bobbin_common::sys::console::Putc;
pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::serial::*;
// pub use bobbin_common::{Irq, Poll};
pub use chip::usart::*;

use bobbin_common::sys::ring::*;
// use bobbin_cortexm::hal::nvic;
// use bobbin_cortexm::hal::scb::*;

use core::fmt::{self, Write};

#[derive(Debug)]
pub struct Config {
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    brr: Brr,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            cr1: Cr1(0),
            cr2: Cr2(0),
            cr3: Cr3(0),
            brr: Brr(0),
        }
    }
}

impl Config {
    // 29.5.4 - USART baud rate generation
    // In case of oversampling by 16, the equation is: Baud = Fck / USARTDIV
    // In case of oversampling by 8, the equation is: Baud = 2 * Fck / USARTDIV

    // NOTE: USART1 is on APB2, all others on APB1
    // Fck will be 36Mhz for APB1, 72Mhz for APB2

    // Assuming FCK is 36Mhz and 8x oversampling, 115,200 = 2 * 36Mhz / USARTDIV
    // USARTDIV = 2 * 36Mhz / 115,200 = 625
    // let brr = (apb_hz / baud_hz) as u16;    
    pub fn set_baud(self, baud: u32, clock: u32) -> Self {
        self.set_brr(clock / baud)
    }

    pub fn set_baud_clock(self, baud: u32, clock: u32) -> Self {
        self.set_brr(clock / baud)
    }

    pub fn set_brr(mut self, brr: u32) -> Self {
        self.brr = Brr(0)
            .set_div_fraction((brr & 0b1111) as u32)
            .set_div_mantissa(brr as u32 >> 4);
        self            
    }
}

impl Configure<Config> for UsartPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: self.cr1(),
            cr2: self.cr2(),
            cr3: self.cr3(),
            brr: self.brr(),
        }
    }


    fn configure(&self, cfg: Config) -> &Self {        
        self.set_cr1(|_| cfg.cr1);
        self.set_cr2(|_| cfg.cr2);
        self.set_cr3(|_| cfg.cr3);
        self.set_brr(|_| cfg.brr);
        self
    }
}

impl Enabled for UsartPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_ue()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self
            .with_cr1(|r| r
                .set_ue(value)
                .set_re(value)
                .set_te(value)
            )        
    }
}

impl Write for UsartPeriph {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}


impl SerialTx<u8> for UsartPeriph {    
    fn can_tx(&self) -> bool {
        self.isr().txe() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_tdr(|r| r.set_tdr(c))
    }
}

impl SerialRx<u8> for UsartPeriph {
    fn can_rx(&self) -> bool {
        self.isr().rxne() != 0
    }

    fn rx(&self) -> u8 {
        self.rdr().rdr().value() as u8
    }
}

impl Putc for UsartPeriph {
    fn console_putc(&self, c: u8) {
        self.putc(c);
    }
}

pub struct UsartDriver<'a> {    
    usart: UsartPeriph,
    tx: Ring<'a, u8>,
    rx: Ring<'a, u8>,    
    // irq_num: Option<u8>,
}

impl<'a> UsartDriver<'a> {
    pub fn new<U: Into<UsartPeriph>>(usart: U, tx_buf: &mut [u8], rx_buf: &mut [u8]) -> Self {
        let tx = Ring::new(tx_buf);
        let rx = Ring::new(rx_buf);
   
        UsartDriver {
            usart: usart.into(),
            tx: tx,
            rx: rx,
            // irq_num: None,
        }
    }

    // pub fn enable_irq<I: Irq>(&mut self, irq: &I) {
    //     self.irq_num = Some(irq.irq_num());
    //     SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
    //     nvic::set_enabled(irq.irq_num() as usize, true);
    // }

    pub fn enable_rx(&self) {
        self.usart.with_cr1(|r| r.set_rxneie(1));
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let n = self.tx.write(buf);
        if n > 0 {
            self.usart.with_cr1(|r| r.set_txeie(1));
        }
        n
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        self.rx.read(buf)
    }
}

// impl<'a> Drop for UsartDriver<'a> {
//     fn drop(&mut self) {
//         if let Some(irq_num) = self.irq_num {
//             nvic::set_enabled(irq_num as usize, false);
//             SCB.set_irq_handler(irq_num as usize, None);
//         }
//     }
// }

// impl<'a> Poll for UsartDriver<'a> {
//     fn poll(&self) {
//         if self.usart.can_tx() {
//             if let Some(b) = self.tx.dequeue() {
//                 self.usart.tx(b);
//                 if self.tx.is_empty() {
//                     self.usart.with_cr1(|r| r.set_txeie(0));
//                 }
//             }
//         }
//         if self.usart.can_rx() {
//             let b = self.usart.rx();
//             if !self.rx.is_full() {
//                 self.rx.enqueue(b);
//             }
//         }
//     }

// }

impl<'a> Write for UsartDriver<'a> {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        let buf = buf.as_bytes();
        let mut n = 0;
        loop {
            if n == buf.len() { return Ok(())}
            n += self.write(&buf[n..])
        }
    }
}