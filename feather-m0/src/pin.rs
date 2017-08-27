use chip::port::*;
use hal::port;

macro_rules! pindef {
    ($id:ident, $pin:expr) => {
        pub fn $id() -> port::PinUnknown { 
            port::pin($pin.port, $pin.index)
        }
    }
}
pindef!(rx, PA11); // RX
pindef!(tx, PA12); // TX

pindef!(d0, PA11); // RX
pindef!(d1, PA12); // TX
pindef!(d2, PA14);
pindef!(d3, PA09);
pindef!(d4, PA08);
pindef!(d5, PA15);
pindef!(d6, PA20);
pindef!(d7, PA21);
pindef!(d8, PA06);
pindef!(d9, PA07);
pindef!(d10, PA18);
pindef!(d11, PA16);
pindef!(d12, PA19);
pindef!(d13, PA17);

pindef!(a0, PA02);
pindef!(a1, PB08);
pindef!(a2, PB09);
pindef!(a3, PA04);
pindef!(a4, PA05);
pindef!(a5, PB02);
pindef!(sda, PA22);
pindef!(scl, PA23);

pindef!(miso, PA12);
pindef!(mosi, PB10);
pindef!(sck, PB11);


pindef!(pa0, PA00);
pindef!(pa1, PA01);
pindef!(pa2, PA02);
pindef!(pa3, PA03);
pindef!(pa4, PA04);
pindef!(pa5, PA05);
pindef!(pa6, PA06);
pindef!(pa7, PA07);
pindef!(pa8, PA08);
pindef!(pa9, PA09);
pindef!(pa10, PA10);
pindef!(pa11, PA11);
pindef!(pa12, PA12);
pindef!(pa13, PA13);
pindef!(pa14, PA14);
pindef!(pa15, PA15);
pindef!(pa16, PA16);
pindef!(pa17, PA17);
pindef!(pa18, PA18);
pindef!(pa19, PA19);
pindef!(pa20, PA20);
pindef!(pa21, PA21);
pindef!(pa22, PA22);
pindef!(pa23, PA23);
pindef!(pa24, PA24);
pindef!(pa25, PA25);
pindef!(pa27, PA27);
pindef!(pa28, PA28);
pindef!(pa30, PA30);
pindef!(pa31, PA31);

pindef!(pb0, PB00);
pindef!(pb1, PB01);
pindef!(pb2, PB02);
pindef!(pb3, PB03);
pindef!(pb4, PB04);
pindef!(pb5, PB05);
pindef!(pb6, PB06);
pindef!(pb7, PB07);
pindef!(pb8, PB08);
pindef!(pb9, PB09);
pindef!(pb10, PB10);
pindef!(pb11, PB11);
pindef!(pb12, PB12);
pindef!(pb13, PB13);
pindef!(pb14, PB14);
pindef!(pb15, PB15);
pindef!(pb16, PB16);
pindef!(pb17, PB17);
pindef!(pb22, PB22);
pindef!(pb23, PB23);
pindef!(pb30, PB30);
pindef!(pb31, PB31);
