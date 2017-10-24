use hal::gpio::*;

pub const BUZ0: Pc1 = PC1;

pub fn init() {
    BUZ0.port().rcc_enable();
    BUZ0.mode_output().set_output(true);
}
