// use chip::gpio::*;
// use gpio;
// use rcc;

// macro_rules! pindef {
//     ($id:ident, $pin:expr) => {
//         pub fn $id() -> gpio::PinUnknown { 
//             rcc::set_gpio_enabled($pin.port(), true);
//             gpio::pin(($pin.port(), $pin.index()))
//         }
//     }
// }

// pindef!(pa0, PA0);
// pindef!(pa1, PA1);
// pindef!(pa2, PA2);
// pindef!(pa3, PA3);
// pindef!(pa4, PA4);
// pindef!(pa5, PA5);
// pindef!(pa6, PA6);
// pindef!(pa7, PA7);
// pindef!(pa8, PA8);
// pindef!(pa9, PA9);
// pindef!(pa10, PA10);
// pindef!(pa11, PA11);
// pindef!(pa12, PA12);
// pindef!(pa13, PA13);
// pindef!(pa14, PA14);
// pindef!(pa15, PA15);

// pindef!(pb0, PB0);
// pindef!(pb1, PB1);
// pindef!(pb2, PB2);
// pindef!(pb3, PB3);
// pindef!(pb4, PB4);
// pindef!(pb5, PB5);
// pindef!(pb6, PB6);
// pindef!(pb7, PB7);
// pindef!(pb8, PB8);
// pindef!(pb9, PB9);
// pindef!(pb10, PB10);
// pindef!(pb11, PB11);
// pindef!(pb12, PB12);
// pindef!(pb13, PB13);
// pindef!(pb14, PB14);
// pindef!(pb15, PB15);

// pindef!(pc0, PC0);
// pindef!(pc1, PC1);
// pindef!(pc2, PC2);
// pindef!(pc3, PC3);
// pindef!(pc4, PC4);
// pindef!(pc5, PC5);
// pindef!(pc6, PC6);
// pindef!(pc7, PC7);
// pindef!(pc8, PC8);
// pindef!(pc9, PC9);
// pindef!(pc10, PC10);
// pindef!(pc11, PC11);
// pindef!(pc12, PC12);
// pindef!(pc13, PC13);
// pindef!(pc14, PC14);
// pindef!(pc15, PC15);

// pindef!(pd0, PD0);
// pindef!(pd1, PD1);
// pindef!(pd2, PD2);
// pindef!(pd3, PD3);
// pindef!(pd4, PD4);
// pindef!(pd5, PD5);
// pindef!(pd6, PD6);
// pindef!(pd7, PD7);
// pindef!(pd8, PD8);
// pindef!(pd9, PD9);
// pindef!(pd10, PD10);
// pindef!(pd11, PD11);
// pindef!(pd12, PD12);
// pindef!(pd13, PD13);
// pindef!(pd14, PD14);
// pindef!(pd15, PD15);

// pindef!(pe0, PE0);
// pindef!(pe1, PE1);
// pindef!(pe2, PE2);
// pindef!(pe3, PE3);
// pindef!(pe4, PE4);
// pindef!(pe5, PE5);
// pindef!(pe6, PE6);
// pindef!(pe7, PE7);
// pindef!(pe8, PE8);
// pindef!(pe9, PE9);
// pindef!(pe10, PE10);
// pindef!(pe11, PE11);
// pindef!(pe12, PE12);
// pindef!(pe13, PE13);
// pindef!(pe14, PE14);
// pindef!(pe15, PE15);

// pindef!(pf0, PF0);
// pindef!(pf1, PF1);
// pindef!(pf2, PF2);
// pindef!(pf3, PF3);
// pindef!(pf4, PF4);
// pindef!(pf5, PF5);
// pindef!(pf6, PF6);
// pindef!(pf7, PF7);
// pindef!(pf8, PF8);
// pindef!(pf9, PF9);
// pindef!(pf10, PF10);
// pindef!(pf11, PF11);
// pindef!(pf12, PF12);
// pindef!(pf13, PF13);
// pindef!(pf14, PF14);
// pindef!(pf15, PF15);

// pindef!(pg0, PG0);
// pindef!(pg1, PG1);
// pindef!(pg2, PG2);
// pindef!(pg3, PG3);
// pindef!(pg4, PG4);
// pindef!(pg5, PG5);
// pindef!(pg6, PG6);
// pindef!(pg7, PG7);
// pindef!(pg8, PG8);
// pindef!(pg9, PG9);
// pindef!(pg10, PG10);
// pindef!(pg11, PG11);
// pindef!(pg12, PG12);
// pindef!(pg13, PG13);
// pindef!(pg14, PG14);
// pindef!(pg15, PG15);

// pindef!(ph0, PH0);
// pindef!(ph1, PH1);
// pindef!(ph2, PH2);