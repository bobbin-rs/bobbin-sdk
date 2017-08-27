use hal::gpio::*;

// LED = PA8

pub const LED0: Pa8 = PA8;

pub fn init() {    
    LED0.port.rcc_enable();
    LED0.mode_output();
}
