use hal::gpio::*;

pub const BTN0: Pe15 = PE15;

pub fn init() {
    BTN0.port().rcc_enable();
    BTN0.mode_input();
}
