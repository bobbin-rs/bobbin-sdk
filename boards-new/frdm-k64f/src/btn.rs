use mcu::pin::*;
use mcu::gpio::*;
pub use common::btn::*;

pub use common::digital::DigitalInput;

pub const BTN0: Pc6 = PC6;
pub const BTN0_PT: Ptc6 = PTC6;

pub const BTN1: Pa4 = PA4;
pub const BTN1_PT: Pta4 = PTA4;

pub fn init() {
    BTN0_PT.port().gate_enable();
    BTN0_PT.connect_to(BTN0);

    BTN1_PT.port().gate_enable();
    BTN1_PT.connect_to(BTN1);


    BTN0.set_dir_input();
    BTN1.set_dir_input();
}


impl ::FrdmK64f {
    pub fn btn0(&self) -> BtnLow<GpioCh> { BtnLow::new(PC6_CH) }
    pub fn btn1(&self) -> BtnLow<GpioCh> { BtnLow::new(PA4_CH) }
}