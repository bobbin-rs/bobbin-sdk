
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

    let pins: [GpioPin; 1] = [spi_nss.into()];
    let mut tx_buf = [SpiAction::Idle; 16];
    let mut rx_buf = [0u8; 16];
    let s = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&spi.irq_spi());

    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        let a = s.reg_read(0, tx);
        println!("0x{:02x}: 0x{:02x} = 0x{:02x}", tx, rx, a);
        assert_eq!(rx, a);
    }

    println!("--- transfer ---");

    // let tx_buf: [u8; 7] = [0x01, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];    
    let tx_buf = [0x01];
    let mut rx_buf = [0u8; 6];    

    s.transfer(0, &tx_buf, &mut rx_buf);
    for i in 0..rx_buf.len() {        
        println!("0x{:02x}", rx_buf[i]);
    }
        
    assert_eq!(rx_buf[0], 0x09);
    assert_eq!(rx_buf[1], 0x1a);
    assert_eq!(rx_buf[2], 0x0b);
    assert_eq!(rx_buf[3], 0x00);
    assert_eq!(rx_buf[4], 0x52);
    assert_eq!(rx_buf[5], 0x6c);

    println!("--- commands --- ");

    s.enqueue(SpiAction::Start(0));
    s.enqueue(SpiAction::Write(0x01));    
    s.enqueue(SpiAction::Repeat(5));
    s.enqueue(SpiAction::Transfer(0x55));
    s.enqueue(SpiAction::Stop(0));

    let mut rx_buf = [0u8; 6];    

    s.read(&mut rx_buf);
    for i in 0..rx_buf.len() {        
        println!("0x{:02x}", rx_buf[i]);
    }
        
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

pub struct SpiDriver<'a> {
    spi: SpiPeriph,
    pins: &'a [GpioPin],
    repeat: Cell<u8>,
    action: Cell<Option<SpiAction>>,
    tx: Ring<'a, SpiAction>,
    rx: Ring<'a, u8>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for SpiDriver<'a> {}
unsafe impl<'a> Send for SpiDriver<'a> {}

impl<'a> SpiDriver<'a> {
    pub fn new<P: Into<SpiPeriph>>(spi: P, pins: &'a [GpioPin], tx_buf: &'a mut [SpiAction], rx_buf: &'a mut [u8]) -> Self {
        SpiDriver { 
            spi: spi.into(),
            pins: pins,
            repeat: Cell::new(0),
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

    pub fn enqueue(&self, action: SpiAction) {        
        self.tx.enqueue(action);
        self.next();
    }

    pub fn tx_len(&self) -> usize {
        self.tx.len()
    }

    pub fn rx_len(&self) -> usize {
        self.rx.len()
    }

    pub fn read(&self, buf: &mut [u8]) {
        while self.rx.len() < buf.len() {}
        self.rx.read(buf);
    }

    pub fn read_byte(&self) -> u8 {
        while self.rx.len() == 0 {}
        self.rx.dequeue().unwrap()
    }

    pub fn action(&self) -> Option<SpiAction> {
        self.action.get()
    }

    pub fn next(&self) {
        if self.action().is_none() {
            loop {
                if let Some(action) = self.tx.dequeue() {
                    match action {
                        SpiAction::Start(pin) => {
                            self.pins[pin as usize].set_output(false);
                        },
                        SpiAction::Write(b) | SpiAction::Transfer(b) => { 
                            self.action.set(Some(action));
                            self.spi.tx(b);
                            self.transfer_enable();
                            break;
                        },
                        SpiAction::Repeat(n) => {
                            self.repeat.set(n);
                        }
                        SpiAction::Stop(pin) => {
                            self.pins[pin as usize].set_output(true);
                        },
                        SpiAction::Idle => {}
                    }
                } else {
                    self.action.set(None);
                    self.transfer_disable();
                    break;
                }
            }
        }
    }

    pub fn reg_read(&self, pin: u8, reg: u8) -> u8 {
        assert!(self.rx.len() == 0);
        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Transfer(0x55));        
        self.enqueue(SpiAction::Stop(pin));
        while self.rx.len() == 0 {}
        self.rx.dequeue().unwrap()
    }

    pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Write(value));
        self.enqueue(SpiAction::Stop(pin));
        while self.action().is_some() {}
    }

    pub fn transfer(&self, pin: u8, tx_buf: &[u8], rx_buf: &mut [u8]) {
        self.enqueue(SpiAction::Start(pin));        
        for b in tx_buf.iter() {
            self.enqueue(SpiAction::Write(*b));
        }
        if rx_buf.len() > 0 {
            self.enqueue(SpiAction::Repeat((rx_buf.len() - 1) as u8));
        }
        self.enqueue(SpiAction::Transfer(0x55));
        self.enqueue(SpiAction::Stop(pin));
        self.read(rx_buf);
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
        let action = self.action().unwrap();
        let repeat = self.repeat.get();
        // println!("SR: {:?} Action: {:?}", sr, self.action());
        if sr.rxne() != 0 {
            match action {
                SpiAction::Write(b) => { 
                    let _: u8  = self.spi.rx(); 
                    if repeat > 0 {
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                },
                SpiAction::Transfer(b) => { 
                    self.rx.enqueue(self.spi.rx()); 
                    if repeat > 0 { 
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                },
                _ => {},
            }            
            self.next();
        }
    }
}
