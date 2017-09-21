use hal::gpio::*;

pub const LED0: Pf0 = PF0;
pub const LED1: Pf4 = PF4;
pub const LED2: Pn0 = PN0;
pub const LED3: Pn1 = PN1;

pub fn init() {
    // Hangs if LED2 and LED have power enabled in sequence

    GPIOF.sysctl_enable();
    GPION.sysctl_enable();

    LED0.mode_output();
    LED1.mode_output();
    LED2.mode_output();   
    LED3.mode_output();
}

pub fn led0_on() {
    LED0.set_output(true);
}

pub fn led0_off() {
    LED0.set_output(false);
}

pub fn led0_toggle() {
    LED0.toggle_output();
}

// use hal::gpio;
// use pin;

// // Led0 => (GpioF, 0),
// // Led1 => (GpioF, 4),
// // Led2 => (GpioN, 0),
// // Led3 => (GpioN, 1),

// pub fn init() {
// }

// pub fn led0() -> gpio::PinOutput {
//     pin::pf0().into_output()
// }
// pub fn led1() -> gpio::PinOutput {
//     pin::pf4().into_output()
// }
// pub fn led2() -> gpio::PinOutput {
//     pin::pn0().into_output()
// }
// pub fn led3() -> gpio::PinOutput {
//     pin::pn1().into_output()
// }