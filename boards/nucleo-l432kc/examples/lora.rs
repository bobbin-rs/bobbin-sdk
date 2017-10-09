
#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::spi::*;
// use board::hal::RegisterPoll;
// use board::hal::nvic::{NvicEnabled, RegisterPoll};
// use board::clock::*;
// use core::ptr;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    
    test_spi_lora();    
    loop {}
}

/// RFM9x LoRa Radio on pins D10-D13
fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    let spi = SPI1;

    let spi_miso = PB4; // D12
    let spi_mosi = PB5; // D11
    let spi_sck = PB3; // D13
    let spi_nss = PA11; // D10

    spi.rcc_enable();
    GPIOA.rcc_enable();
    GPIOB.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
    // spi_nss.mode_spi_nss(&spi).speed_high().push_pull();
    spi_nss.mode_output().set_output(true);

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b100.into())
    );

    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);

    let mut tx_buf = [SpiAction::Idle; 16];
    let mut rx_buf = [0u8; 16];
    let s = SpiDriver::new(spi, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&spi.irq_spi());

    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        let a = s.reg_read(&spi_nss, tx);
        println!("0x{:02x}: 0x{:02x} = 0x{:02x}", tx, rx, a);
        assert_eq!(rx, a);
    }

    // let tx_buf: [u8; 7] = [0x01, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];    
    
    spi_nss.set_output(false);
    s.enqueue_action(SpiAction::Write(0x01));
    s.enqueue_action(SpiAction::Transfer(0x55));
    s.enqueue_action(SpiAction::Transfer(0x55));
    s.enqueue_action(SpiAction::Transfer(0x55));
    s.enqueue_action(SpiAction::Transfer(0x55));
    s.enqueue_action(SpiAction::Transfer(0x55));
    s.enqueue_action(SpiAction::Transfer(0x55));    

    let mut rx_buf = [0u8; 6];    

    for i in 0..rx_buf.len() {
        rx_buf[i] = s.read_byte();
        println!("0x{:02x}", rx_buf[i]);
    }
    spi_nss.set_output(true);
    
    assert_eq!(rx_buf[0], 0x09);
    assert_eq!(rx_buf[1], 0x1a);
    assert_eq!(rx_buf[2], 0x0b);
    assert_eq!(rx_buf[3], 0x00);
    assert_eq!(rx_buf[4], 0x52);
    assert_eq!(rx_buf[5], 0x6c);

    println!("[pass] SPI OK");
    spi.rcc_disable();
    spi_sck.mode_analog();
    spi_mosi.mode_analog();
    spi_miso.mode_analog();
    spi_nss.mode_analog();

}

use board::common::ring::Ring;
use board::common::{Irq, Poll};
use board::common::digital::DigitalOutput;
use board::hal::gpio::GpioPin;
use board::hal::scb::SCB;
use board::hal::nvic;

use core::cell::Cell;
use core::marker::PhantomData;
// use core::slice;
// use core::ptr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpiAction {
    Idle,
    Write(u8),
    Transfer(u8),
}

impl Default for SpiAction {
    fn default() -> Self {
        SpiAction::Idle
    }
}

pub struct SpiDriver<'a> {
    spi: SpiPeriph,
    action: Cell<Option<SpiAction>>,
    tx: Ring<'a, SpiAction>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for SpiDriver<'a> {}
unsafe impl<'a> Send for SpiDriver<'a> {}

impl<'a> SpiDriver<'a> {
    pub fn new<P: Into<SpiPeriph>>(spi: P, tx_buf: &'a mut [SpiAction], rx_buf: &'a mut [u8]) -> Self {
        SpiDriver { 
            spi: spi.into(),
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

    pub fn enqueue_action(&self, action: SpiAction) {        
        if self.action().is_none() {
            self.set_action(action);
        } else {
            self.tx.enqueue(action);
        }        
    }

    pub fn rx_len(&self) -> usize {
        self.rx.len()
    }

    pub fn read_byte(&self) -> u8 {
        while self.rx.len() == 0 {}
        self.rx.dequeue().unwrap()
    }

    pub fn action(&self) -> Option<SpiAction> {
        self.action.get()
    }

    pub fn set_action(&self, action: SpiAction) {
        match action {
            SpiAction::Write(b) | SpiAction::Transfer(b) => { 
                self.action.set(Some(action));
                self.spi.tx(b);
                self.transfer_enable();                    
            },
            SpiAction::Idle => {}
        }
    }

    pub fn next_action(&self) {
        if let Some(action) = self.tx.dequeue() {
            self.set_action(action)
        } else {
            self.action.set(None);
            self.transfer_disable();
        }
    }

    pub fn reg_read(&self, nss: &GpioPin, reg: u8) -> u8 {
        let mut buf = [0u8; 2];
        self.transfer(nss, &[SpiAction::Transfer(reg), SpiAction::Transfer(0x55)], &mut buf);
        buf[1]
    }

    pub fn reg_write(&self, nss: &GpioPin, reg: u8, value: u8) {
        let mut buf = [];
        self.transfer(nss, &[SpiAction::Transfer(reg), SpiAction::Transfer(value)], &mut buf);
    }

    pub fn transfer(&self, nss: &GpioPin, tx_buf: &[SpiAction], rx_buf: &mut [u8]) {
        nss.set_output(false);
        for b in tx_buf.iter() {
            self.enqueue_action(*b);
        }
        loop {
            if self.rx.len() >= rx_buf.len() {
                for i in 0..rx_buf.len() {
                    rx_buf[i] = self.rx.dequeue().unwrap();
                }
                nss.set_output(true);
                break
            }    
        }
    }

    pub fn transfer_enable(&self) {        
        self.spi.with_cr2(|r| r.set_rxneie(true));
        self.spi.set_enabled(true);
    }
    
    pub fn transfer_disable(&self) {        
        self.spi.with_cr2(|r| r.set_rxneie(false));
    }
}

impl<'a> Poll for SpiDriver<'a> {
    fn poll(&self) {       
        let sr = self.spi.sr();
        // println!("SR: {:?} Action: {:?}", sr, self.action());
        if sr.rxne() != 0 {
            match self.action() {
                Some(SpiAction::Write(_)) => { let _: u8  = self.spi.rx(); },
                Some(SpiAction::Transfer(_)) => { self.rx.enqueue(self.spi.rx()); },
                _ => {},
            }
            self.next_action();
        }
    }
}
