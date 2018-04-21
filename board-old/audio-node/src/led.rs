use hal::gpio::*;
use common::Pin;

pub const LED0: Pb10 = PB10;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output().set_output(false);
}