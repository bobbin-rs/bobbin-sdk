#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lcd::*;

periph!( LCD, Lcd, LCD_PERIPH, LcdPeriph, 0x40002400, 0x05);

