use prelude::*;
use mcu::pin::*;

pub fn init() {
    PA11.port().gate_enable();
    PA11.set_mode_input().set_pull_enabled(true).set_output(true);
}

impl ::ArduinoZero {
    pub fn btn0(&self) -> BtnLow<PortPin> {
        BtnLow::new(PA11_PIN)
    }
}