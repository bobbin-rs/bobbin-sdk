use prelude::*;

use mcu::gpio::*;
use mcu::pin::*;

pub const LED0: LedHigh<GpioCh> = LedHigh::new(PB22_CH);
pub const LED1: LedHigh<GpioCh> = LedHigh::new(PB21_CH);
pub const LED2: LedHigh<GpioCh> = LedHigh::new(PE26_CH);

pub fn init() {    
    PTB22.port().gate_enable();
    PTB22.connect_to(PB22);    
    PB22.set_dir_output().set_output(true);

    PTB21.port().gate_enable();
    PTB21.connect_to(PB21);
    PB21.set_dir_output().set_output(true);

    PTE26.port().gate_enable();
    PTE26.connect_to(PE26);
    PE26.set_dir_output().set_output(true);

}

impl GetLed for ::FrdmK64f {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            1 => &LED1,
            2 => &LED2,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 3 }
}

impl ::FrdmK64f {
    pub fn led0(&self) -> LedHigh<GpioCh> { LedHigh::new(PB22_CH) }
    pub fn led1(&self) -> LedHigh<GpioCh> { LedHigh::new(PB21_CH) }
    pub fn led2(&self) -> LedHigh<GpioCh> { LedHigh::new(PE26_CH) }

}