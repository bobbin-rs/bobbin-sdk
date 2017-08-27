#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::exti::*;

periph!( EXTI, Exti, _EXTI, ExtiPeriph, 0x40010400);



channel!(EXTI_LINE0, ExtiLine0, EXTI, Exti, _EXTI_LINE0, ExtiCh, _EXTI, 0);
channel!(EXTI_LINE1, ExtiLine1, EXTI, Exti, _EXTI_LINE1, ExtiCh, _EXTI, 1);
channel!(EXTI_LINE2, ExtiLine2, EXTI, Exti, _EXTI_LINE2, ExtiCh, _EXTI, 2);
channel!(EXTI_LINE3, ExtiLine3, EXTI, Exti, _EXTI_LINE3, ExtiCh, _EXTI, 3);
channel!(EXTI_LINE4, ExtiLine4, EXTI, Exti, _EXTI_LINE4, ExtiCh, _EXTI, 4);
channel!(EXTI_LINE5, ExtiLine5, EXTI, Exti, _EXTI_LINE5, ExtiCh, _EXTI, 5);
channel!(EXTI_LINE6, ExtiLine6, EXTI, Exti, _EXTI_LINE6, ExtiCh, _EXTI, 6);
channel!(EXTI_LINE7, ExtiLine7, EXTI, Exti, _EXTI_LINE7, ExtiCh, _EXTI, 7);
channel!(EXTI_LINE8, ExtiLine8, EXTI, Exti, _EXTI_LINE8, ExtiCh, _EXTI, 8);
channel!(EXTI_LINE9, ExtiLine9, EXTI, Exti, _EXTI_LINE9, ExtiCh, _EXTI, 9);
channel!(EXTI_LINE10, ExtiLine10, EXTI, Exti, _EXTI_LINE10, ExtiCh, _EXTI, 10);
channel!(EXTI_LINE11, ExtiLine11, EXTI, Exti, _EXTI_LINE11, ExtiCh, _EXTI, 11);
channel!(EXTI_LINE12, ExtiLine12, EXTI, Exti, _EXTI_LINE12, ExtiCh, _EXTI, 12);
channel!(EXTI_LINE13, ExtiLine13, EXTI, Exti, _EXTI_LINE13, ExtiCh, _EXTI, 13);
channel!(EXTI_LINE14, ExtiLine14, EXTI, Exti, _EXTI_LINE14, ExtiCh, _EXTI, 14);
channel!(EXTI_LINE15, ExtiLine15, EXTI, Exti, _EXTI_LINE15, ExtiCh, _EXTI, 15);
channel!(EXTI_LINE16, ExtiLine16, EXTI, Exti, _EXTI_LINE16, ExtiCh, _EXTI, 16);
channel!(EXTI_LINE17, ExtiLine17, EXTI, Exti, _EXTI_LINE17, ExtiCh, _EXTI, 17);
channel!(EXTI_LINE18, ExtiLine18, EXTI, Exti, _EXTI_LINE18, ExtiCh, _EXTI, 18);
channel!(EXTI_LINE19, ExtiLine19, EXTI, Exti, _EXTI_LINE19, ExtiCh, _EXTI, 19);
channel!(EXTI_LINE20, ExtiLine20, EXTI, Exti, _EXTI_LINE20, ExtiCh, _EXTI, 20);
channel!(EXTI_LINE21, ExtiLine21, EXTI, Exti, _EXTI_LINE21, ExtiCh, _EXTI, 21);
channel!(EXTI_LINE22, ExtiLine22, EXTI, Exti, _EXTI_LINE22, ExtiCh, _EXTI, 22);
channel!(EXTI_LINE23, ExtiLine23, EXTI, Exti, _EXTI_LINE23, ExtiCh, _EXTI, 23);
channel!(EXTI_LINE24, ExtiLine24, EXTI, Exti, _EXTI_LINE24, ExtiCh, _EXTI, 24);
channel!(EXTI_LINE25, ExtiLine25, EXTI, Exti, _EXTI_LINE25, ExtiCh, _EXTI, 25);
channel!(EXTI_LINE26, ExtiLine26, EXTI, Exti, _EXTI_LINE26, ExtiCh, _EXTI, 26);
channel!(EXTI_LINE28, ExtiLine28, EXTI, Exti, _EXTI_LINE28, ExtiCh, _EXTI, 28);
channel!(EXTI_LINE29, ExtiLine29, EXTI, Exti, _EXTI_LINE29, ExtiCh, _EXTI, 29);
channel!(EXTI_LINE30, ExtiLine30, EXTI, Exti, _EXTI_LINE30, ExtiCh, _EXTI, 30);
channel!(EXTI_LINE31, ExtiLine31, EXTI, Exti, _EXTI_LINE31, ExtiCh, _EXTI, 31);
channel!(EXTI_LINE32, ExtiLine32, EXTI, Exti, _EXTI_LINE32, ExtiCh, _EXTI, 32);
channel!(EXTI_LINE33, ExtiLine33, EXTI, Exti, _EXTI_LINE33, ExtiCh, _EXTI, 33);
channel!(EXTI_LINE34, ExtiLine34, EXTI, Exti, _EXTI_LINE34, ExtiCh, _EXTI, 34);
channel!(EXTI_LINE35, ExtiLine35, EXTI, Exti, _EXTI_LINE35, ExtiCh, _EXTI, 35);
