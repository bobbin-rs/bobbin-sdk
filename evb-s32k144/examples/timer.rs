#![no_std]
#![no_main]
#![feature(const_fn)]

#[macro_use]
extern crate evb_s32k144 as board;

#[macro_use]
extern crate simple_semaphore;

use simple_semaphore::{Semaphore, SemaphoreReader, SemaphoreWriter};
use board::chip::irq::Irq;
use board::hal::lpit::Timer;
use board::hal::nvic;

// Assume PIT bus clock is 40Mhz

pub struct Guard {
    index: usize,
    irq: Irq,
}

impl Drop for Guard {
    fn drop(&mut self) {
        unsafe {
            println!("drop: {}, {}", self.index, self.irq.0);
            self.irq.set_handler(None);
            HANDLERS[self.index] = None;
        }
    }
}

pub trait HandleIrq {
    fn handle_irq(&self);
}

static mut HANDLERS: [Option<*const HandleIrq>; 4] = [None; 4];

unsafe extern "C" fn handle_irq(index: usize) {
    if let Some(handler) = HANDLERS[index] {
        (&*handler).handle_irq();
    }
}

unsafe extern "C" fn handle_irq_0() { handle_irq(0) }
unsafe extern "C" fn handle_irq_1() { handle_irq(1) }
unsafe extern "C" fn handle_irq_2() { handle_irq(2) }
unsafe extern "C" fn handle_irq_3() { handle_irq(3) }


pub fn register(irq: Irq, handler: *const HandleIrq) -> Guard {    
    unsafe {
        let channel = irq.0 - 48;
        println!("registering {} {} {:p}", irq.0, channel, handler);
        HANDLERS[channel] = Some(handler);
        irq.set_handler(Some(match channel {
            0 => handle_irq_0,
            1 => handle_irq_1,
            2 => handle_irq_2,
            3 => handle_irq_3,
            _ => panic!("Only channel 0..3 supported")
        }));
        nvic::set_enabled(irq.0, true);
        Guard { index: channel, irq: irq }
    }
}


pub struct Driver<'a> {
    timer: Timer,    
    reader: SemaphoreReader<'a>,
    writer: SemaphoreWriter<'a>,
}

impl<'a> Driver<'a> {
    pub fn new(timer: Timer, reader: SemaphoreReader<'a>, writer: SemaphoreWriter<'a>) -> Self {        
        Driver {
            timer: timer,
            reader: reader,
            writer: writer,
        }
    }

    pub fn irq(&self) -> Irq {
        self.timer.irq()
    }

    pub fn start(&self, period: u32) {
        let timer = &self.timer;
        timer.set_value(40_000 * period);
        timer.clr_tif();
        timer.set_tie(true);
        timer.set_enabled(true);
    }

    pub fn stop(&self) {
        let timer = &self.timer;
        timer.set_enabled(false);
        timer.set_tie(false);
    }

    pub fn ready(&self) -> bool {
        self.reader.read() != 0
    }

    pub fn wait(&self) {
        while self.reader.read() == 0 {}
    }
}


impl<'a> HandleIrq for Driver<'a> {    
    fn handle_irq(&self) {
        if self.timer.tif() {
            self.timer.clr_tif();
            self.writer.write(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    

    let (r0, w0) = static_semaphore!();
    let (r1, w1) = static_semaphore!();
    let client0 = Driver::new(board::timer::lpit0(), r0, w0);
    let client1 = Driver::new(board::timer::lpit1(), r1, w1);
    println!("client0: {:p}", &client0);
    println!("client1: {:p}", &client1);
    let _guard_1 = register(client0.irq(), &client0 as *const HandleIrq);
    let _guard_2 = register(client1.irq(), &client1 as *const HandleIrq);

    client0.start(700);

    client1.start(500);    

    let mut i = 0;
    let mut j = 0;
    loop {
        if client0.ready() {
            println!("Client 0: {}", j);
            if j == 10 {
                println!("Stopping Client 0");
                client0.stop();
            }
            j += 1;
        }
        if client1.ready() {
            println!("            Client 1: {}", i);
            if i == 10 {
                println!("Stopping Client 1");
                client1.stop();
            }
            i += 1;
        }
    }
}