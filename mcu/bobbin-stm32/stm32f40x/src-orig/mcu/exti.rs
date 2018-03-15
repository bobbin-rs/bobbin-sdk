#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::exti::*;

periph!( EXTI, Exti, EXTI_PERIPH, ExtiPeriph, 0x40013c00, 0x0d);

channel!(EXTI_LINE0, ExtiLine0, EXTI, Exti, EXTI_LINE0_CH, ExtiCh, EXTI_PERIPH, 0);
channel!(EXTI_LINE1, ExtiLine1, EXTI, Exti, EXTI_LINE1_CH, ExtiCh, EXTI_PERIPH, 1);
channel!(EXTI_LINE2, ExtiLine2, EXTI, Exti, EXTI_LINE2_CH, ExtiCh, EXTI_PERIPH, 2);
channel!(EXTI_LINE3, ExtiLine3, EXTI, Exti, EXTI_LINE3_CH, ExtiCh, EXTI_PERIPH, 3);
channel!(EXTI_LINE4, ExtiLine4, EXTI, Exti, EXTI_LINE4_CH, ExtiCh, EXTI_PERIPH, 4);
channel!(EXTI_LINE5, ExtiLine5, EXTI, Exti, EXTI_LINE5_CH, ExtiCh, EXTI_PERIPH, 5);
channel!(EXTI_LINE6, ExtiLine6, EXTI, Exti, EXTI_LINE6_CH, ExtiCh, EXTI_PERIPH, 6);
channel!(EXTI_LINE7, ExtiLine7, EXTI, Exti, EXTI_LINE7_CH, ExtiCh, EXTI_PERIPH, 7);
channel!(EXTI_LINE8, ExtiLine8, EXTI, Exti, EXTI_LINE8_CH, ExtiCh, EXTI_PERIPH, 8);
channel!(EXTI_LINE9, ExtiLine9, EXTI, Exti, EXTI_LINE9_CH, ExtiCh, EXTI_PERIPH, 9);
channel!(EXTI_LINE10, ExtiLine10, EXTI, Exti, EXTI_LINE10_CH, ExtiCh, EXTI_PERIPH, 10);
channel!(EXTI_LINE11, ExtiLine11, EXTI, Exti, EXTI_LINE11_CH, ExtiCh, EXTI_PERIPH, 11);
channel!(EXTI_LINE12, ExtiLine12, EXTI, Exti, EXTI_LINE12_CH, ExtiCh, EXTI_PERIPH, 12);
channel!(EXTI_LINE13, ExtiLine13, EXTI, Exti, EXTI_LINE13_CH, ExtiCh, EXTI_PERIPH, 13);
channel!(EXTI_LINE14, ExtiLine14, EXTI, Exti, EXTI_LINE14_CH, ExtiCh, EXTI_PERIPH, 14);
channel!(EXTI_LINE15, ExtiLine15, EXTI, Exti, EXTI_LINE15_CH, ExtiCh, EXTI_PERIPH, 15);
channel!(EXTI_LINE16, ExtiLine16, EXTI, Exti, EXTI_LINE16_CH, ExtiCh, EXTI_PERIPH, 16);
channel!(EXTI_LINE17, ExtiLine17, EXTI, Exti, EXTI_LINE17_CH, ExtiCh, EXTI_PERIPH, 17);
channel!(EXTI_LINE18, ExtiLine18, EXTI, Exti, EXTI_LINE18_CH, ExtiCh, EXTI_PERIPH, 18);
channel!(EXTI_LINE19, ExtiLine19, EXTI, Exti, EXTI_LINE19_CH, ExtiCh, EXTI_PERIPH, 19);
channel!(EXTI_LINE20, ExtiLine20, EXTI, Exti, EXTI_LINE20_CH, ExtiCh, EXTI_PERIPH, 20);
channel!(EXTI_LINE21, ExtiLine21, EXTI, Exti, EXTI_LINE21_CH, ExtiCh, EXTI_PERIPH, 21);
channel!(EXTI_LINE22, ExtiLine22, EXTI, Exti, EXTI_LINE22_CH, ExtiCh, EXTI_PERIPH, 22);
