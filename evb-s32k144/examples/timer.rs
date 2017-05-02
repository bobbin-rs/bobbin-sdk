#![no_std]
#![no_main]
#![feature(const_fn)]

#[macro_use]
extern crate evb_s32k144 as board;

#[macro_use]
extern crate simple_semaphore;

use simple_semaphore::{Semaphore, SemaphoreReader, SemaphoreWriter};
use board::hal::lpit::Timer;
use core::cell::UnsafeCell;

// Assume PIT bus clock is 40Mhz

pub trait HandleIrq {
    fn handle_irq(&self);
}

static mut HANDLERS: [Option<&'static HandleIrq>; 8] = [None; 8];

unsafe extern "C" fn handle_irq(index: usize) {
    if let Some(handler) = HANDLERS[index] {
        handler.handle_irq();
    }
}

unsafe extern "C" fn handle_irq_0() { handle_irq(0) }
unsafe extern "C" fn handle_irq_1() { handle_irq(1) }
unsafe extern "C" fn handle_irq_2() { handle_irq(2) }
unsafe extern "C" fn handle_irq_3() { handle_irq(3) }


pub fn register(h: &'static HandleIrq) -> unsafe extern "C" fn() {
    unsafe {
        for i in 0..HANDLERS.len() {
            if HANDLERS[i].is_none() {
                HANDLERS[i] = Some(h);
                return match i {
                    0 => handle_irq_0,
                    1 => handle_irq_1,
                    2 => handle_irq_2,
                    3 => handle_irq_3,
                    _ => panic!("Out of IRQ handler slots")
                }
            }
        }
        panic!("Out of IRQ handler slots")
    }
}


macro_rules! timer_client {
    ($timer:expr) => {
        {            
            let (r, w) = static_semaphore!();
            static mut DRIVER: UnsafeCell<Option<Driver>> = UnsafeCell::new(None);            
            unsafe {
                DRIVER = UnsafeCell::new(Some(Driver::new($timer.clone(), w)));
                $timer.set_handler(Some(register((&*DRIVER.get()).as_ref().unwrap() as &HandleIrq)));
            }            
        
            Client::new($timer, r)
        }
    }
}

pub struct Client<'a> {
    timer: Timer,
    reader: SemaphoreReader<'a>,
}

impl<'a> Client<'a> {
    pub fn new(timer: Timer, reader: SemaphoreReader<'a>) -> Self {
        Client {
            timer: timer,
            reader: reader,
        }
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

pub struct Driver<'a> {
    timer: Timer,
    writer: SemaphoreWriter<'a>
}

impl<'a> Driver<'a> {
    pub fn new(timer: Timer, writer: SemaphoreWriter<'a>) -> Self {       
        Driver { 
            timer: timer,
            writer: writer,
        }
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
    
    let client0 = timer_client!(board::timer::lpit0());
    let client1 = timer_client!(board::timer::lpit1());

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