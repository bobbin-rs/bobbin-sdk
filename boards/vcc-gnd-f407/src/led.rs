use hal::gpio::*;

pub const LED0: Pb9 = PB9;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output();
    LED0.set_output(true);
}