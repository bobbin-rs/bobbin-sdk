use chip::gpio::*;
use hal::sysctl;
use hal::gpio;

macro_rules! pindef {
    ($id:ident, $pin:expr) => {
        pub fn $id() -> gpio::PinUnknown { 
            sysctl::set_gpio_enabled($pin.port(), true);
            gpio::pin($pin.port(), $pin.index())
        }
    }
}
pindef!(pa0, PA0);
pindef!(pa1, PA1);
pindef!(pa2, PA2);
pindef!(pa3, PA3);
pindef!(pa4, PA4);
pindef!(pa5, PA5);
pindef!(pa6, PA6);
pindef!(pa7, PA7);

pindef!(pb0, PB0);
pindef!(pb1, PB1);
pindef!(pb2, PB2);
pindef!(pb3, PB3);
pindef!(pb4, PB4);
pindef!(pb5, PB5);


pindef!(pc0, PC0);
pindef!(pc1, PC1);
pindef!(pc2, PC2);
pindef!(pc3, PC3);
pindef!(pc4, PC4);
pindef!(pc5, PC5);
pindef!(pc6, PC6);
pindef!(pc7, PC7);

pindef!(pd0, PD0);
pindef!(pd1, PD1);
pindef!(pd2, PD2);
pindef!(pd3, PD3);
pindef!(pd4, PD4);
pindef!(pd5, PD5);
pindef!(pd6, PD6);
pindef!(pd7, PD7);

pindef!(pe0, PE0);
pindef!(pe1, PE1);
pindef!(pe2, PE2);
pindef!(pe3, PE3);
pindef!(pe4, PE4);
pindef!(pe5, PE5);

pindef!(pf0, PF0);
pindef!(pf1, PF1);
pindef!(pf2, PF2);
pindef!(pf3, PF3);
pindef!(pf4, PF4);

pindef!(pg0, PG0);
pindef!(pg1, PG1);

pindef!(ph0, PH0);
pindef!(ph1, PH1);
pindef!(ph2, PH2);
pindef!(ph3, PH3);

pindef!(pj0, PJ0);
pindef!(pj1, PJ1);

pindef!(pk0, PK0);
pindef!(pk1, PK1);
pindef!(pk2, PK2);
pindef!(pk3, PK3);
pindef!(pk4, PK4);
pindef!(pk5, PK5);
pindef!(pk6, PK6);
pindef!(pk7, PK7);

pindef!(pl0, PL0);
pindef!(pl1, PL1);
pindef!(pl2, PL2);
pindef!(pl3, PL3);
pindef!(pl4, PL4);
pindef!(pl5, PL5);
pindef!(pl6, PL6);
pindef!(pl7, PL7);

pindef!(pm0, PM0);
pindef!(pm1, PM1);
pindef!(pm2, PM2);
pindef!(pm3, PM3);
pindef!(pm4, PM4);
pindef!(pm5, PM5);
pindef!(pm6, PM6);
pindef!(pm7, PM7);

pindef!(pn0, PN0);
pindef!(pn1, PN1);
pindef!(pn2, PN2);
pindef!(pn3, PN3);
pindef!(pn4, PN4);
pindef!(pn5, PN5);

pindef!(pp0, PP0);
pindef!(pp1, PP1);
pindef!(pp2, PP2);
pindef!(pp3, PP3);
pindef!(pp4, PP4);
pindef!(pp5, PP5);

pindef!(pq0, PQ0);
pindef!(pq1, PQ1);
pindef!(pq2, PQ2);
pindef!(pq3, PQ3);
pindef!(pq4, PQ4);