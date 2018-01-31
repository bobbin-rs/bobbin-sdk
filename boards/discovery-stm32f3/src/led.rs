use hal::gpio::*;

// LD3 = PD13 (Orange)
// LD4 = PD12 (Green)
// LD5 = PD14 (Red)
// LD6 = PD15 (Blue)

pub const LED0: Pe9 = PE9;
pub const LED1: Pe8 = PE8;
pub const LED2: Pe10 = PE10;
pub const LED3: Pe15 = PE15;
pub const LED4: Pe11 = PE11;
pub const LED5: Pe14 = PE14;
pub const LED6: Pe12 = PE12;
pub const LED7: Pe13 = PE13;

pub const LD3: Pe9 = PE9;
pub const LD4: Pe8 = PE8;
pub const LD5: Pe10 = PE10;
pub const LD6: Pe15 = PE15;
pub const LD7: Pe11 = PE11;
pub const LD8: Pe14 = PE14;
pub const LD9: Pe12 = PE12;
pub const LD10: Pe13 = PE13;

pub const LED_N: Pe9 = PE9;
pub const LED_NW: Pe8 = PE8;
pub const LED_NE: Pe10 = PE10;
pub const LED_W: Pe15 = PE15;
pub const LED_E: Pe11 = PE11;
pub const LED_SW: Pe14 = PE14;
pub const LED_SE: Pe12 = PE12;
pub const LED_S: Pe13 = PE13;

pub fn init() {    
    LED0.port().rcc_enable();
    LED0.mode_output();

    LED1.port().rcc_enable();
    LED1.mode_output();

    LED2.port().rcc_enable();
    LED2.mode_output();

    LED3.port().rcc_enable();
    LED3.mode_output();

    LED3.port().rcc_enable();
    LED3.mode_output();

    LED4.port().rcc_enable();
    LED4.mode_output();

    LED5.port().rcc_enable();
    LED5.mode_output();

    LED6.port().rcc_enable();
    LED6.mode_output();

    LED7.port().rcc_enable();
    LED7.mode_output();
}
