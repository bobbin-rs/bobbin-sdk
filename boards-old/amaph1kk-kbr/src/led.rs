pub use common::hal::digital::DigitalOutput;
use hal::gpio::*;

// LED0 = P17

pub const LED0: Gp17 = GP17;
pub const LED1: Gp14 = GP14;
pub const LED2: Gp15 = GP15;
pub const LED3: Gp30 = GP30;
pub const LED4: Gp10 = GP10;

pub fn init() {  
    GPIO.unlock();  
    LED0
        .set_pad_gpio()
        .set_gpio_output_pushpull();
    LED1
        .set_pad_gpio()
        .set_gpio_output_pushpull();        
    LED2
        .set_pad_gpio()
        .set_gpio_output_pushpull();
    LED3
        .set_pad_gpio()
        .set_gpio_output_pushpull();
    LED4
        .set_pad_gpio()
        .set_gpio_output_pushpull();
}
