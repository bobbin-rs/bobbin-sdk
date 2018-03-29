#[allow(unused_imports)]

use core::fmt::Write;
use common::delay::Delay;

pub struct Tick<OUT: Write, DEL: Delay> {
    out: OUT,
    del: DEL,
    delay_ms: u32,
}

impl<OUT, DEL> Tick<OUT, DEL> 
where
    OUT: Write,
    DEL: Delay,
{
    pub fn new(out: OUT, del: DEL, delay_ms: u32) -> Self {
        Self { out, del, delay_ms }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let _ = self.out.write_str("Tick...\r\n");
            self.del.delay_ms(self.delay_ms);
        }
    }
}