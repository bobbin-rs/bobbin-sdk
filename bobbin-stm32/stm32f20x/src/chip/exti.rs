#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::exti::*;

periph!(_EXTI, ExtiPeriph, EXTI, Exti, 0x40013c00);



channel!(EXTI_LINE0, ExtiLine0, EXTI, Exti, 0);
channel!(EXTI_LINE1, ExtiLine1, EXTI, Exti, 1);
channel!(EXTI_LINE2, ExtiLine2, EXTI, Exti, 2);
channel!(EXTI_LINE3, ExtiLine3, EXTI, Exti, 3);
channel!(EXTI_LINE4, ExtiLine4, EXTI, Exti, 4);
channel!(EXTI_LINE5, ExtiLine5, EXTI, Exti, 5);
channel!(EXTI_LINE6, ExtiLine6, EXTI, Exti, 6);
channel!(EXTI_LINE7, ExtiLine7, EXTI, Exti, 7);
channel!(EXTI_LINE8, ExtiLine8, EXTI, Exti, 8);
channel!(EXTI_LINE9, ExtiLine9, EXTI, Exti, 9);
channel!(EXTI_LINE10, ExtiLine10, EXTI, Exti, 10);
channel!(EXTI_LINE11, ExtiLine11, EXTI, Exti, 11);
channel!(EXTI_LINE12, ExtiLine12, EXTI, Exti, 12);
channel!(EXTI_LINE13, ExtiLine13, EXTI, Exti, 13);
channel!(EXTI_LINE14, ExtiLine14, EXTI, Exti, 14);
channel!(EXTI_LINE15, ExtiLine15, EXTI, Exti, 15);
channel!(EXTI_LINE16, ExtiLine16, EXTI, Exti, 16);
channel!(EXTI_LINE17, ExtiLine17, EXTI, Exti, 17);
channel!(EXTI_LINE18, ExtiLine18, EXTI, Exti, 18);
channel!(EXTI_LINE19, ExtiLine19, EXTI, Exti, 19);
channel!(EXTI_LINE20, ExtiLine20, EXTI, Exti, 20);
channel!(EXTI_LINE21, ExtiLine21, EXTI, Exti, 21);
channel!(EXTI_LINE22, ExtiLine22, EXTI, Exti, 22);
