#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::pcc;
use board::hal::lpi2c::*;
use board::hal::lpspi::*;
use board::hal::port::*;
use board::hal::gpio::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    PORTA.pcc_enable();
    PORTB.pcc_enable();

    let spi = LPSPI0;

    
    let port_sck = PTB2;
    let port_sin = PTB3;
    let port_sout = PTB4;
    let port_cs = PTB5;    
    let port_g0 = PTD14;
    let port_rst = PTD13;

    println!("# SPI Start");

    port_sck.mode_spi_sck(&spi);
    port_sin.mode_spi_sin(&spi);
    port_sout.mode_spi_sout(&spi);
    // port_cs.mode_spi_pcs0(&spi);

    port_cs.set_mux_gpio();
    port_g0.set_mux_gpio();
    port_rst.set_mux_gpio();

    port_cs.gpio_pin().set_output(true).set_dir_output();
    port_g0.gpio_pin().set_dir_input();    
    
    port_rst.gpio_pin().set_output(true).set_dir_output();

    let cs = port_cs.gpio_pin();
    let _g0 = port_g0.gpio_pin();
    let rst = port_rst.gpio_pin();

    println!("Initialize SPI");

    spi.pcc_set_clock_source(pcc::ClockSource::SPLLDIV2).pcc_set_enabled(true);
    spi.set_enabled(false);    
    
    spi.configure(Config::default()
        .set_master(true)
        .set_clock_config(
            8,
            8,
            9,
            4
        )
        // .sckpcs(4)
        // .pcssck(9)
        // .dbt(8)
        // .sckdiv(8)
        // .txwater(3)        
    );
    spi.with_fcr(|r| r.set_txwater(3));

    spi.set_enabled(true);
    let t = spi.target()
        .cpol(false)
        .cpha(false)
        .prescale(3)
        .pcs(0)
        .framesz(7);
    t.configure();

    println!("SPI Running");

    rst.set_output(false);    
    cs.set_output(false);

    let cmd = [0x10];
    let mut buf = [0u8];
    t.write(&[0x10]);
    t.read(&mut buf);
    println!("Version: 0x{:02x}", buf[0]);
    cs.set_output(true);

    let irq = spi.irq_lpspi();

    let mut tx_buf = [SpiAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let pins: [GpioPin; 1] = [cs.into()];
    let s = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&irq);

    println!("Reading Version...");
    let v = s.reg_read(0, 0x10);
    println!("Version: 0x{:02x}", v);
    

    loop {}
}


use board::common::ring::Ring;
use board::common::{Irq, Poll};
use board::common::digital::DigitalOutput;
use board::cortexm::wfi;
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::*;
use board::hal::gpio::GpioPin;

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
    spi: LpspiPeriph,
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
    pub fn new<P: Into<LpspiPeriph>>(spi: P, pins: &'a [GpioPin], tx_buf: &'a mut [SpiAction], rx_buf: &'a mut [u8]) -> Self {
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
        while self.rx.len() < buf.len() {
            wfi()
        }
        self.rx.read(buf);
    }

    pub fn read_byte(&self) -> u8 {
        while self.rx.len() == 0 {
            wfi()
        }
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
                            self.spi.set_enabled(true);
                            self.spi.set_tcr(|r| r.set_framesz(7));
                            self.spi.tx(b);
                            self.spi.with_ier(|r| r.set_tdie(true));
                            break;
                        },
                        SpiAction::Repeat(n) => {
                            self.repeat.set(n);
                        }
                        SpiAction::Stop(pin) => {
                            self.pins[pin as usize].set_output(true);
                            self.spi.set_enabled(false);                            
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
        while self.rx.len() == 0 {
            wfi()
        }
        self.rx.dequeue().unwrap()
    }

    pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Write(value));
        self.enqueue(SpiAction::Stop(pin));
        while self.action().is_some() {
            wfi()
        }
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
        self.spi.with_ier(|r| r.set_tdie(true));
        self.spi.set_enabled(true);
    }
    
    pub fn transfer_disable(&self) {        
        self.spi.set_ier(|r| r);
        self.spi.set_enabled(false);
    }

}

impl<'a> Poll for SpiDriver<'a> {
    fn poll(&self) {       
        let sr = self.spi.sr();
        let action = self.action().unwrap();
        let repeat = self.repeat.get();
        // println!("SR: {:?} Action: {:?}", sr, self.action());        
        match action {
            SpiAction::Write(b) => { 
                if sr.rdf() != 0 {
                    let _: u8  = self.spi.rx(); 
                    if repeat > 0 {
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                }
            },
            SpiAction::Transfer(b) => { 
                if sr.rdf() != 0 {
                    self.rx.enqueue(self.spi.rx()); 
                    if repeat > 0 { 
                        self.repeat.set(repeat - 1);
                        self.spi.tx(b);
                    } else {
                        self.action.set(None);
                    }
                }
            },
            _ => {},
        }            
        self.next();
    }
}
