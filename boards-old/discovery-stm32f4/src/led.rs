use hal::gpio::*;

// LD3 = PD13 (Orange)
// LD4 = PD12 (Green)
// LD5 = PD14 (Red)
// LD6 = PD15 (Blue)

pub const LED0: Pd13 = PD13;
pub const LED1: Pd12 = PD12;
pub const LED2: Pd14 = PD14;
pub const LED3: Pd15 = PD15;


pub fn init() {    
    LED0.port().rcc_enable();
    LED0.mode_output();

    LED1.port().rcc_enable();
    LED1.mode_output();

    LED2.port().rcc_enable();
    LED2.mode_output();

    LED3.port().rcc_enable();
    LED3.mode_output();
}
