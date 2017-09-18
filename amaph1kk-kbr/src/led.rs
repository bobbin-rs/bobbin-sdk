pub use common::digital::DigitalOutput;
use hal::gpio::*;

// LED0 = P17

pub const LED0: Gp17 = GP17;

pub fn init() {  
    GPIO.unlock();  
    LED0
        .set_pad_gpio()
        .set_gpio_output_pushpull();


}
