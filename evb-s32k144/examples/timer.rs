#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

#[macro_use]
extern crate simple_semaphore;

use simple_semaphore::{Semaphore, SemaphoreReader, SemaphoreWriter};
use board::hal::lpit::Timer;

// Assume PIT bus clock is 40Mhz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let client0 = setup(board::timer::lpit0(), static_semaphore!());
    let client1 = setup(board::timer::lpit1(), static_semaphore!());    

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
            println!("Client 1: {}", i);
            if i == 10 {
                println!("Stopping Client 1");
                client1.stop();
            }
            i += 1;
        }
    }
}

pub fn setup(t: Timer, sem: (SemaphoreReader<'static>, SemaphoreWriter<'static>)) -> Client<'static> {    
    let (r, w) = sem;
    let index = unsafe {
        if DRIVER0.is_none() {
            DRIVER0 = Some(Driver::new(t.clone(), w));
            0
        } else if DRIVER1.is_none() {
            DRIVER1 = Some(Driver::new(t.clone(), w));
            1
        } else {
            panic!("No driver slots available");
        }
    };
    Client::new(t, r, index)
}

pub struct Client<'a> {
    timer: Timer,
    reader: SemaphoreReader<'a>,
    index: usize
}

impl<'a> Client<'a> {
    pub fn new(timer: Timer, reader: SemaphoreReader<'a>, index: usize) -> Self {
        Client {
            timer: timer,
            reader: reader,
            index: index,
        }
    }

    pub fn start(&self, period: u32) {
        let timer = &self.timer;
        let handler = match self.index { 
            0 => handler_0,
            1 => handler_1,
            _ => panic!("out of handlers"),
        };
        timer.set_handler(Some(handler));
        timer.set_value(40_000 * period);
        timer.clr_tif();
        timer.set_tie(true);
        timer.set_enabled(true);
    }

    pub fn stop(&self) {
        let timer = &self.timer;
        timer.set_enabled(false);
        timer.set_handler(None);
    }

    pub fn ready(&self) -> bool {
        self.reader.read() != 0
    }

    pub fn wait(&self) {
        while self.reader.read() == 0 {}
    }
}

static mut DRIVER0: Option<Driver> = None;
static mut DRIVER1: Option<Driver> = None;

pub unsafe extern "C" fn handler_0() {
    DRIVER0.as_ref().unwrap().handle_irq()
}        

pub unsafe extern "C" fn handler_1() {
    DRIVER1.as_ref().unwrap().handle_irq()
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
    
    pub fn handle_irq(&self) {
        if self.timer.tif() {
            self.timer.clr_tif();
            self.writer.write(1);
        }
    }
}

