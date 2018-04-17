#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate embedded_hal as hal;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();    
    let con = ConsoleWriter::new();
    let del = DelayTimer::new();
    let mut app = examples::tick::Tick::new(con, del, 500);
    app.run()
}

pub struct DelayTimer;

impl DelayTimer {
    pub fn new() -> Self { Self {} }
}

impl hal::blocking::delay::DelayMs<u16> for DelayTimer {
    fn delay_ms(&mut self, ms: u16) {
        board::delay(ms.into())
    }
}

pub struct ConsoleWriter {}
impl ConsoleWriter {
    pub fn new() -> Self { Self {} }
}

impl hal::blocking::serial::Write<u8> for ConsoleWriter {
    type Error = ();
    fn bwrite_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        use board::bobbin_hal::serial::SerialTx;
        board::console::USART.write(buf);
        Ok(())
    }
    fn bflush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}