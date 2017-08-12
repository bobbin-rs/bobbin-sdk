use chip::port::*;

macro_rules! pindef {
    ($id:ident, $t:ty, $pin:expr) => {
        pub const $id: $t = $pin;
    }
}



pindef!(D0, Pa11, PA11);
pindef!(D1, Pa12, PA12);
pindef!(D2, Pa14, PA14);
pindef!(D3, Pa09, PA09);
pindef!(D4, Pa08, PA08);
pindef!(D5, Pa15, PA15);
pindef!(D6, Pa20, PA20);
pindef!(D7, Pa21, PA21);
pindef!(D8, Pa06, PA06);
pindef!(D9, Pa07, PA07);
pindef!(D10, Pa18, PA18);
pindef!(D11, Pa16, PA16);
pindef!(D12, Pa19, PA19);
pindef!(D13, Pa17, PA17);

pindef!(A0, Pa02, PA02);
pindef!(A1, Pb08, PB08);
pindef!(A2, Pb09, PB09);
pindef!(A3, Pa04, PA04);
pindef!(A4, Pa05, PA05);
pindef!(A5, Pb02, PB02);

pindef!(SDA, Pa22, PA22);
pindef!(SCL, Pa23, PA23);

pindef!(MISO, Pa12, PA12);
pindef!(MOSI, Pb10, PB10);
pindef!(SCK, Pb11, PB11);
