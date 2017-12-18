use hal::gpio::*;

pub const LED0: Pc0 = PC0;
pub const LED1: Pc2 = PC2;
pub const LED2: Pc3 = PC3;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output().set_output(false);

    LED1.port().rcc_enable();
    LED1.mode_output().set_output(false);

    LED2.port().rcc_enable();
    LED2.mode_output().set_output(false);
}
