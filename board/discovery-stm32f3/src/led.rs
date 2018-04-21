use ::prelude::*;

pub use bobbin_hal::led::*;
pub use mcu::pin::*;
pub use mcu::gpio::*;

// pub const LED0: Pe9 = PE9;
// pub const LED1: Pe8 = PE8;
// pub const LED2: Pe10 = PE10;
// pub const LED3: Pe15 = PE15;
// pub const LED4: Pe11 = PE11;
// pub const LED5: Pe14 = PE14;
// pub const LED6: Pe12 = PE12;
// pub const LED7: Pe13 = PE13;

// pub const LD3: Pe9 = PE9;
// pub const LD4: Pe8 = PE8;
// pub const LD5: Pe10 = PE10;
// pub const LD6: Pe15 = PE15;
// pub const LD7: Pe11 = PE11;
// pub const LD8: Pe14 = PE14;
// pub const LD9: Pe12 = PE12;
// pub const LD10: Pe13 = PE13;

// pub const LED_N: Pe9 = PE9;
// pub const LED_NW: Pe8 = PE8;
// pub const LED_NE: Pe10 = PE10;
// pub const LED_W: Pe15 = PE15;
// pub const LED_E: Pe11 = PE11;
// pub const LED_SW: Pe14 = PE14;
// pub const LED_SE: Pe12 = PE12;
// pub const LED_S: Pe13 = PE13;


pub const LED0: LedHigh<GpioPin> = LedHigh::new(PE9_PIN);
pub const LED1: LedHigh<GpioPin> = LedHigh::new(PE8_PIN);
pub const LED2: LedHigh<GpioPin> = LedHigh::new(PE10_PIN);
pub const LED3: LedHigh<GpioPin> = LedHigh::new(PE15_PIN);
pub const LED4: LedHigh<GpioPin> = LedHigh::new(PE11_PIN);
pub const LED5: LedHigh<GpioPin> = LedHigh::new(PE14_PIN);
pub const LED6: LedHigh<GpioPin> = LedHigh::new(PE12_PIN);
pub const LED7: LedHigh<GpioPin> = LedHigh::new(PE13_PIN);


pub fn init() {
    PE9.port().gate_enable();
    PE9.mode_output();

    PE8.port().gate_enable();
    PE8.mode_output();

    PE10.port().gate_enable();
    PE10.mode_output();

    PE15.port().gate_enable();
    PE15.mode_output();

    PE11.port().gate_enable();
    PE11.mode_output();

    PE14.port().gate_enable();
    PE14.mode_output();

    PE12.port().gate_enable();
    PE12.mode_output();

    PE13.port().gate_enable();
    PE13.mode_output();
}

impl GetLed for ::Board {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            1 => &LED1,
            2 => &LED2,
            3 => &LED3,
            4 => &LED4,
            5 => &LED5,
            6 => &LED6,
            7 => &LED7,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 8 }
}

impl ::Board {
    pub fn led0(&self) -> LedHigh<GpioPin> { LedHigh::new(PE9_PIN) }
    pub fn led1(&self) -> LedHigh<GpioPin> { LedHigh::new(PE8_PIN) }
    pub fn led2(&self) -> LedHigh<GpioPin> { LedHigh::new(PE10_PIN) }
    pub fn led3(&self) -> LedHigh<GpioPin> { LedHigh::new(PE15_PIN) }
    pub fn led4(&self) -> LedHigh<GpioPin> { LedHigh::new(PE11_PIN) }
    pub fn led5(&self) -> LedHigh<GpioPin> { LedHigh::new(PE14_PIN) }
    pub fn led6(&self) -> LedHigh<GpioPin> { LedHigh::new(PE12_PIN) }
    pub fn led7(&self) -> LedHigh<GpioPin> { LedHigh::new(PE13_PIN) }
}