use hal::gpio::*;

pub const LED0: Pb6 = PB6;
pub const LED1: Pb5 = PB5;
pub const LED2: Pb4 = PB4;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output().set_output(true);

    LED1.port().rcc_enable();
    LED1.mode_output().set_output(true);

    LED2.port().rcc_enable();
    LED2.mode_output().set_output(true);
}
