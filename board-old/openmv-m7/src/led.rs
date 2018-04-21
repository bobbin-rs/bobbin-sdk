use hal::gpio::*;

pub const LED0: Pc0 = PC0;
pub const LED1: Pc1 = PC1;
pub const LED2: Pc2 = PC2;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output().set_output(true);

    LED1.port().rcc_enable();
    LED1.mode_output().set_output(true);

    LED2.port().rcc_enable();
    LED2.mode_output().set_output(true);
}
