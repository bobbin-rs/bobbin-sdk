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

    let t1 = board::timer::lpit1();
    let (r, w) = static_semaphore!();
    let client = Client { reader: r };
    
    unsafe { 
        DRIVER = Some(Driver::new(t1, 500, w));
    }

    loop {
        client.wait();
        println!("Tick");
    }
}
pub struct Client<'a> {
    reader: SemaphoreReader<'a>
}

impl<'a> Client<'a> {
    pub fn wait(&self) {
        while self.reader.read() == 0 {}
    }
}

static mut DRIVER: Option<Driver> = None;

pub unsafe extern "C" fn handler() {
    DRIVER.as_ref().unwrap().handle_irq()
}        


pub struct Driver<'a> {
    timer: Timer,
    writer: SemaphoreWriter<'a>
}

impl<'a> Driver<'a> {
    pub fn new(timer: Timer, period: u32, writer: SemaphoreWriter<'a>) -> Self {

        timer.set_handler(Some(handler));
        timer.set_value(40_000 * period);
        timer.clr_tif();
        timer.set_tie(true);
        timer.set_enabled(true);
        
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

