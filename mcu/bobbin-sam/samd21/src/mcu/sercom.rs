#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::sercom::*;

// //! Serial Communication Interface

periph!( SERCOM0, Sercom0, SERCOM0_PERIPH, SercomPeriph, 0x42000800, 0x10);
periph!( SERCOM1, Sercom1, SERCOM1_PERIPH, SercomPeriph, 0x42000c00, 0x11);
periph!( SERCOM2, Sercom2, SERCOM2_PERIPH, SercomPeriph, 0x42001000, 0x12);
periph!( SERCOM3, Sercom3, SERCOM3_PERIPH, SercomPeriph, 0x42001400, 0x13);
periph!( SERCOM4, Sercom4, SERCOM4_PERIPH, SercomPeriph, 0x42001800, 0x14);
periph!( SERCOM5, Sercom5, SERCOM5_PERIPH, SercomPeriph, 0x42001c00, 0x15);

