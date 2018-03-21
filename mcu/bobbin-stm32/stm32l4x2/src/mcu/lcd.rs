#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::lcd::*;

periph!( LCD, Lcd, LCD_PERIPH, LcdPeriph, 0x40002400, 0x00, 0x05);

