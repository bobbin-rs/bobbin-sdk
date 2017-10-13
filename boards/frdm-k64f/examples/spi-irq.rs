#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::spi::*;
use board::hal::port::*;
use board::hal::gpio::*;

use board::common::bits::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running SPI Driver");

    let port = PORTD;
    let port_sck = PTD1; // D13
    let port_sout = PTD2; // D12
    let port_sin = PTD3; // D11
    let port_pcs0 = PTD0; // D10

    let spi = SPI0;

    port.sim_enable();
    port_sck.mode_spi_sck(&spi);
    port_sout.mode_spi_sout(&spi);
    port_sin.mode_spi_sin(&spi);
    // port_pcs0.mode_spi_pcs0(&spi);

    port_pcs0.set_mux_gpio();

    let cs = port_pcs0.gpio_pin();
    cs.set_dir_output().set_output(true);

    spi.sim_enable();
    spi.init(0b1000, 0b00);
    

    let mut tx_buf = [SpiAction::Idle; 64];
    let mut rx_buf = [0u8; 64];
    let pins: [GpioPin; 1] = [cs.into()];
    let irq = spi.irq_spi();
    let spi = SpiDriver::new(spi, &pins, &mut tx_buf, &mut rx_buf);
    spi.enable_irq(&irq);
    println!("SPI Initialized");
    for r in 0..16 {
        println!("{:02x}: {:02x}", r, spi.reg_read(0, r));
    }

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x0a, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x0b, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");

    let mut buf = [0u8; 6];
    spi.transfer(0, &[0x10, 0x55, 0x55, 0x55, 0x55, 0x55], &mut buf);
    for i in 1..buf.len() {
        print!("{:02x} ", buf[i]);
    }
    println!("");



    loop {}

}


use board::common::ring::Ring;
use board::common::{Irq, Poll};
use board::common::digital::DigitalOutput;
use board::cortexm::wfi;
use board::cortexm::hal::nvic;
use board::cortexm::hal::scb::SCB;
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
        nvic::set_enabled(irq.irq_num() as usize, false);
        SCB.set_irq_handler(irq.irq_num() as usize, None);
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn enqueue(&self, action: SpiAction) {        
        self.tx.enqueue(action);
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



    pub fn reg_read(&self, pin: u8, reg: u8) -> u8 {
        // println!("reg_read: {} {:02x}", pin, reg);
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Transfer(0x55));        
        self.enqueue(SpiAction::Stop(pin));
        self.next();

        while self.rx.len() == 0 {
            // wfi()
        }
        self.rx.dequeue().unwrap()
    }

    pub fn reg_write(&self, pin: u8, reg: u8, value: u8) {        
        self.enqueue(SpiAction::Start(pin));
        self.enqueue(SpiAction::Write(reg));
        self.enqueue(SpiAction::Write(value));
        self.enqueue(SpiAction::Stop(pin));
        self.next();
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
        self.next();
        self.read(rx_buf);
    }

    pub fn transfer_enable(&self) {        
        // self.spi.with_ier(|r| r.set_tdie(true));
        // self.spi.set_enabled(true);
    }
    
    pub fn transfer_disable(&self) {        
        // self.spi.set_ier(|r| r);
        // self.spi.set_enabled(false);
    }
    
    pub fn tx(&self, value: u8) {
        self.spi.set_pushr(|_| Pushr(0).set_txdata(value));

    }

    pub fn rx(&self) -> u8 {
        self.spi.popr().rxdata().value() as u8
    }

    pub fn next(&self) {
        if self.action().is_none() {
            loop {
                if let Some(action) = self.tx.dequeue() {
                    // println!("next: {:?}", action);
                    match action {
                        SpiAction::Start(pin) => {   
                            self.spi.with_mcr(|r| r.set_mdis(0).set_halt(1));                                                  
                            self.pins[pin as usize].set_output(false);
                            self.spi.with_mcr(|r| r.set_halt(0));
                        },
                        SpiAction::Write(b) | SpiAction::Transfer(b) => { 
                            self.action.set(Some(action));     
                            self.tx(b);
                            self.spi.with_rser(|r| r.set_tcf_re(1));
         
                            // self.spi.with_rser(|r| r.set_tfff_re(1).set_rfdf_re(1));
                            break;

                        },
                        SpiAction::Repeat(n) => {
                            self.repeat.set(n);
                        }
                        SpiAction::Stop(pin) => {
                            // self.spi.with_mcr(|r| r.set_mdis(0).set_halt(1));
                            self.pins[pin as usize].set_output(true);

                            // self.spi.set_enabled(false);                            
                        },
                        SpiAction::Idle => {}
                    }
                } else {
                    // println!("Loop Done");
                    self.action.set(None);
                    // self.transfer_disable();
                    break;
                }
            }
        }
    }
}

impl<'a> Poll for SpiDriver<'a> {
    fn poll(&self) {       
        let sr = self.spi.sr();
        let action = self.action();
        let repeat = self.repeat.get();
        // println!("SR: {:?} Action: {:?}", sr, self.action());        
        // board::delay(1);
        let action = action.unwrap();
        match action {
            SpiAction::Write(b) => { 
                if sr.test_tcf() {
                    self.spi.with_sr(|r| r.set_tcf(1));
                    let _: u8  = self.rx(); 
                    if repeat > 0 {
                        self.tx(b);
                        self.repeat.set(repeat - 1);
                    } else {
                        self.spi.with_rser(|r| r.set_tcf_re(0));
                        self.action.set(None);
                    }
                }
            },
            SpiAction::Transfer(b) => { 
                if sr.test_tcf() {
                    self.spi.with_sr(|r| r.set_rfdf(1));
                    self.rx.enqueue(self.rx()); 
                    if repeat > 0 { 
                        self.tx(b);
                        self.repeat.set(repeat - 1);
                    } else {
                        self.spi.with_rser(|r| r.set_tcf_re(0));
                        self.action.set(None);
                    }
                }
            },

            _ => {},
        }            
        self.next();
        // println!(".");
        // board::delay(100);
    }
}
