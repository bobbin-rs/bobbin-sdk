pub use ::stm32_common::exti::*;

::bobbin_mcu::periph!( EXTI, Exti, EXTI_PERIPH, ExtiPeriph, EXTI_OWNED, EXTI_REF_COUNT, 0x40010400, 0x00, 0x1a);

::bobbin_mcu::channel!(EXTI_LINE0, ExtiLine0, exti_line0, EXTI, Exti, EXTI_LINE0_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE0_OWNED, EXTI_LINE0_REF_COUNT, 0);
::bobbin_mcu::channel!(EXTI_LINE1, ExtiLine1, exti_line1, EXTI, Exti, EXTI_LINE1_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE1_OWNED, EXTI_LINE1_REF_COUNT, 1);
::bobbin_mcu::channel!(EXTI_LINE2, ExtiLine2, exti_line2, EXTI, Exti, EXTI_LINE2_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE2_OWNED, EXTI_LINE2_REF_COUNT, 2);
::bobbin_mcu::channel!(EXTI_LINE3, ExtiLine3, exti_line3, EXTI, Exti, EXTI_LINE3_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE3_OWNED, EXTI_LINE3_REF_COUNT, 3);
::bobbin_mcu::channel!(EXTI_LINE4, ExtiLine4, exti_line4, EXTI, Exti, EXTI_LINE4_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE4_OWNED, EXTI_LINE4_REF_COUNT, 4);
::bobbin_mcu::channel!(EXTI_LINE5, ExtiLine5, exti_line5, EXTI, Exti, EXTI_LINE5_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE5_OWNED, EXTI_LINE5_REF_COUNT, 5);
::bobbin_mcu::channel!(EXTI_LINE6, ExtiLine6, exti_line6, EXTI, Exti, EXTI_LINE6_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE6_OWNED, EXTI_LINE6_REF_COUNT, 6);
::bobbin_mcu::channel!(EXTI_LINE7, ExtiLine7, exti_line7, EXTI, Exti, EXTI_LINE7_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE7_OWNED, EXTI_LINE7_REF_COUNT, 7);
::bobbin_mcu::channel!(EXTI_LINE8, ExtiLine8, exti_line8, EXTI, Exti, EXTI_LINE8_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE8_OWNED, EXTI_LINE8_REF_COUNT, 8);
::bobbin_mcu::channel!(EXTI_LINE9, ExtiLine9, exti_line9, EXTI, Exti, EXTI_LINE9_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE9_OWNED, EXTI_LINE9_REF_COUNT, 9);
::bobbin_mcu::channel!(EXTI_LINE10, ExtiLine10, exti_line10, EXTI, Exti, EXTI_LINE10_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE10_OWNED, EXTI_LINE10_REF_COUNT, 10);
::bobbin_mcu::channel!(EXTI_LINE11, ExtiLine11, exti_line11, EXTI, Exti, EXTI_LINE11_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE11_OWNED, EXTI_LINE11_REF_COUNT, 11);
::bobbin_mcu::channel!(EXTI_LINE12, ExtiLine12, exti_line12, EXTI, Exti, EXTI_LINE12_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE12_OWNED, EXTI_LINE12_REF_COUNT, 12);
::bobbin_mcu::channel!(EXTI_LINE13, ExtiLine13, exti_line13, EXTI, Exti, EXTI_LINE13_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE13_OWNED, EXTI_LINE13_REF_COUNT, 13);
::bobbin_mcu::channel!(EXTI_LINE14, ExtiLine14, exti_line14, EXTI, Exti, EXTI_LINE14_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE14_OWNED, EXTI_LINE14_REF_COUNT, 14);
::bobbin_mcu::channel!(EXTI_LINE15, ExtiLine15, exti_line15, EXTI, Exti, EXTI_LINE15_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE15_OWNED, EXTI_LINE15_REF_COUNT, 15);
::bobbin_mcu::channel!(EXTI_LINE16, ExtiLine16, exti_line16, EXTI, Exti, EXTI_LINE16_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE16_OWNED, EXTI_LINE16_REF_COUNT, 16);
::bobbin_mcu::channel!(EXTI_LINE17, ExtiLine17, exti_line17, EXTI, Exti, EXTI_LINE17_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE17_OWNED, EXTI_LINE17_REF_COUNT, 17);
::bobbin_mcu::channel!(EXTI_LINE18, ExtiLine18, exti_line18, EXTI, Exti, EXTI_LINE18_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE18_OWNED, EXTI_LINE18_REF_COUNT, 18);
::bobbin_mcu::channel!(EXTI_LINE19, ExtiLine19, exti_line19, EXTI, Exti, EXTI_LINE19_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE19_OWNED, EXTI_LINE19_REF_COUNT, 19);
::bobbin_mcu::channel!(EXTI_LINE20, ExtiLine20, exti_line20, EXTI, Exti, EXTI_LINE20_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE20_OWNED, EXTI_LINE20_REF_COUNT, 20);
::bobbin_mcu::channel!(EXTI_LINE21, ExtiLine21, exti_line21, EXTI, Exti, EXTI_LINE21_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE21_OWNED, EXTI_LINE21_REF_COUNT, 21);
::bobbin_mcu::channel!(EXTI_LINE22, ExtiLine22, exti_line22, EXTI, Exti, EXTI_LINE22_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE22_OWNED, EXTI_LINE22_REF_COUNT, 22);
::bobbin_mcu::channel!(EXTI_LINE23, ExtiLine23, exti_line23, EXTI, Exti, EXTI_LINE23_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE23_OWNED, EXTI_LINE23_REF_COUNT, 23);
::bobbin_mcu::channel!(EXTI_LINE24, ExtiLine24, exti_line24, EXTI, Exti, EXTI_LINE24_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE24_OWNED, EXTI_LINE24_REF_COUNT, 24);
::bobbin_mcu::channel!(EXTI_LINE25, ExtiLine25, exti_line25, EXTI, Exti, EXTI_LINE25_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE25_OWNED, EXTI_LINE25_REF_COUNT, 25);
::bobbin_mcu::channel!(EXTI_LINE26, ExtiLine26, exti_line26, EXTI, Exti, EXTI_LINE26_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE26_OWNED, EXTI_LINE26_REF_COUNT, 26);
::bobbin_mcu::channel!(EXTI_LINE27, ExtiLine27, exti_line27, EXTI, Exti, EXTI_LINE27_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE27_OWNED, EXTI_LINE27_REF_COUNT, 27);
::bobbin_mcu::channel!(EXTI_LINE28, ExtiLine28, exti_line28, EXTI, Exti, EXTI_LINE28_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE28_OWNED, EXTI_LINE28_REF_COUNT, 28);
::bobbin_mcu::channel!(EXTI_LINE29, ExtiLine29, exti_line29, EXTI, Exti, EXTI_LINE29_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE29_OWNED, EXTI_LINE29_REF_COUNT, 29);
::bobbin_mcu::channel!(EXTI_LINE30, ExtiLine30, exti_line30, EXTI, Exti, EXTI_LINE30_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE30_OWNED, EXTI_LINE30_REF_COUNT, 30);
::bobbin_mcu::channel!(EXTI_LINE31, ExtiLine31, exti_line31, EXTI, Exti, EXTI_LINE31_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE31_OWNED, EXTI_LINE31_REF_COUNT, 31);
::bobbin_mcu::channel!(EXTI_LINE32, ExtiLine32, exti_line32, EXTI, Exti, EXTI_LINE32_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE32_OWNED, EXTI_LINE32_REF_COUNT, 32);
::bobbin_mcu::channel!(EXTI_LINE33, ExtiLine33, exti_line33, EXTI, Exti, EXTI_LINE33_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE33_OWNED, EXTI_LINE33_REF_COUNT, 33);
::bobbin_mcu::channel!(EXTI_LINE34, ExtiLine34, exti_line34, EXTI, Exti, EXTI_LINE34_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE34_OWNED, EXTI_LINE34_REF_COUNT, 34);
::bobbin_mcu::channel!(EXTI_LINE35, ExtiLine35, exti_line35, EXTI, Exti, EXTI_LINE35_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE35_OWNED, EXTI_LINE35_REF_COUNT, 35);
::bobbin_mcu::channel!(EXTI_LINE36, ExtiLine36, exti_line36, EXTI, Exti, EXTI_LINE36_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE36_OWNED, EXTI_LINE36_REF_COUNT, 36);
::bobbin_mcu::channel!(EXTI_LINE37, ExtiLine37, exti_line37, EXTI, Exti, EXTI_LINE37_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE37_OWNED, EXTI_LINE37_REF_COUNT, 37);
::bobbin_mcu::channel!(EXTI_LINE38, ExtiLine38, exti_line38, EXTI, Exti, EXTI_LINE38_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE38_OWNED, EXTI_LINE38_REF_COUNT, 38);
::bobbin_mcu::channel!(EXTI_LINE39, ExtiLine39, exti_line39, EXTI, Exti, EXTI_LINE39_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE39_OWNED, EXTI_LINE39_REF_COUNT, 39);
::bobbin_mcu::channel!(EXTI_LINE40, ExtiLine40, exti_line40, EXTI, Exti, EXTI_LINE40_CH, ExtiCh, EXTI_PERIPH, EXTI_LINE40_OWNED, EXTI_LINE40_REF_COUNT, 40);
