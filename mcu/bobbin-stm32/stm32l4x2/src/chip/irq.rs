//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_TIM6_DACUNDER, IrqTim6Dacunder, 54);
irq!(IRQ_DMA1_CH1, IrqDma1Ch1, 11);
irq!(IRQ_DMA1_CH2, IrqDma1Ch2, 12);
irq!(IRQ_DMA1_CH3, IrqDma1Ch3, 13);
irq!(IRQ_DMA1_CH4, IrqDma1Ch4, 14);
irq!(IRQ_DMA1_CH5, IrqDma1Ch5, 15);
irq!(IRQ_DMA1_CH6, IrqDma1Ch6, 16);
irq!(IRQ_DMA1_CH7, IrqDma1Ch7, 17);
irq!(IRQ_DMA2_CH1, IrqDma2Ch1, 56);
irq!(IRQ_DMA2_CH2, IrqDma2Ch2, 57);
irq!(IRQ_DMA2_CH3, IrqDma2Ch3, 58);
irq!(IRQ_DMA2_CH4, IrqDma2Ch4, 59);
irq!(IRQ_DMA2_CH5, IrqDma2Ch5, 60);
irq!(IRQ_DMA2_CH6, IrqDma2Ch6, 68);
irq!(IRQ_DMA2_CH7, IrqDma2Ch7, 69);
irq!(IRQ_LCD, IrqLcd, 78);
irq!(IRQ_TSC, IrqTsc, 77);
irq!(IRQ_WWDG, IrqWwdg, 0);
irq!(IRQ_COMP, IrqComp, 64);
irq!(IRQ_I2C1_EV, IrqI2c1Ev, 31);
irq!(IRQ_I2C1_ER, IrqI2c1Er, 32);
irq!(IRQ_I2C2_EV, IrqI2c2Ev, 33);
irq!(IRQ_I2C2_ER, IrqI2c2Er, 34);
irq!(IRQ_I2C3_EV, IrqI2c3Ev, 72);
irq!(IRQ_I2C3_ER, IrqI2c3Er, 73);
irq!(IRQ_I2C4_EV, IrqI2c4Ev, 83);
irq!(IRQ_FLASH, IrqFlash, 4);
irq!(IRQ_RNG, IrqRng, 80);
irq!(IRQ_AES, IrqAes, 79);
irq!(IRQ_ADC1, IrqAdc1, 18);
irq!(IRQ_SAI1, IrqSai1, 74);
irq!(IRQ_TIM2, IrqTim2, 28);
irq!(IRQ_TIM3, IrqTim3, 29);
irq!(IRQ_TIM1_BRK_TIM15, IrqTim1BrkTim15, 24);
irq!(IRQ_TIM1_UP_TIM16, IrqTim1UpTim16, 25);
irq!(IRQ_TIM1_BRK, IrqTim1Brk, 24);
irq!(IRQ_TIM1_UP, IrqTim1Up, 25);
irq!(IRQ_TIM1_TRG_COM, IrqTim1TrgCom, 26);
irq!(IRQ_TIM1_CC, IrqTim1Cc, 27);
irq!(IRQ_TIM6, IrqTim6, 54);
irq!(IRQ_TIM7, IrqTim7, 55);
irq!(IRQ_LPTIM1, IrqLptim1, 65);
irq!(IRQ_LPTIM2, IrqLptim2, 66);
irq!(IRQ_USART1, IrqUsart1, 37);
irq!(IRQ_USART2, IrqUsart2, 38);
irq!(IRQ_USART3, IrqUsart3, 39);
irq!(IRQ_UART4, IrqUart4, 52);
irq!(IRQ_LPUART1, IrqLpuart1, 70);
irq!(IRQ_SPI1, IrqSpi1, 35);
irq!(IRQ_SPI2, IrqSpi2, 36);
irq!(IRQ_SPI3, IrqSpi3, 51);
irq!(IRQ_SDMMC1, IrqSdmmc1, 49);
irq!(IRQ_PVD_PVM, IrqPvdPvm, 1);
irq!(IRQ_EXTI0, IrqExti0, 6);
irq!(IRQ_EXTI1, IrqExti1, 7);
irq!(IRQ_EXTI2, IrqExti2, 8);
irq!(IRQ_EXTI3, IrqExti3, 9);
irq!(IRQ_EXTI4, IrqExti4, 10);
irq!(IRQ_EXTI9_5, IrqExti95, 23);
irq!(IRQ_EXTI15_10, IrqExti1510, 40);
irq!(IRQ_FPU, IrqFpu, 81);
irq!(IRQ_CAN1_TX, IrqCan1Tx, 19);
irq!(IRQ_CAN1_RX0, IrqCan1Rx0, 20);
irq!(IRQ_CAN1_RX1, IrqCan1Rx1, 21);
irq!(IRQ_CAN1_SCE, IrqCan1Sce, 22);
irq!(IRQ_RTC_TAMP_STAMP, IrqRtcTampStamp, 2);
irq!(IRQ_RTC_WKUP, IrqRtcWkup, 3);
irq!(IRQ_RTC_ALARM, IrqRtcAlarm, 41);
irq!(IRQ_SWPMI1, IrqSwpmi1, 76);
irq!(IRQ_CRS, IrqCrs, 82);
irq!(IRQ_USB_FS, IrqUsbFs, 67);
irq!(IRQ_DFSDM1_FLT3, IrqDfsdm1Flt3, 42);
irq!(IRQ_DFSDM1, IrqDfsdm1, 61);
irq!(IRQ_DFSDM2, IrqDfsdm2, 62);
irq!(IRQ_DFSDM1_FLT2, IrqDfsdm1Flt2, 63);
irq!(IRQ_I2C4_ER, IrqI2c4Er, 84);
irq!(IRQ_QUADSPI, IrqQuadspi, 71);

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 85] = [
    None,                          // IRQ 0: Window Watchdog interrupt
    None,                          // IRQ 1: PVD through EXTI line detection
    None,                          // IRQ 2: Tamper and TimeStamp interrupts
    None,                          // IRQ 3: RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts
    None,                          // IRQ 4: Flash global interrupt
    None,                          // IRQ 5: RCC global interrupt
    None,                          // IRQ 6: EXTI Line 0 interrupt
    None,                          // IRQ 7: EXTI Line 1 interrupt
    None,                          // IRQ 8: EXTI Line 2 interrupt
    None,                          // IRQ 9: EXTI Line 3 interrupt
    None,                          // IRQ 10: EXTI Line4 interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 18: ADC1 and ADC2 global interrupt
    None,                          // IRQ 19: CAN1 TX interrupts
    None,                          // IRQ 20: CAN1 RX0 interrupts
    None,                          // IRQ 21: CAN1 RX1 interrupts
    None,                          // IRQ 22: CAN1 SCE interrupt
    None,                          // IRQ 23: EXTI Line5 to Line9 interrupts
    None,                          // IRQ 24: Timer 15 global interrupt
    None,                          // IRQ 25: Timer 16 global interrupt
    None,                          // IRQ 26: TIM1 trigger and commutation interrupt
    None,                          // IRQ 27: TIM1 Capture Compare interrupt
    None,                          // IRQ 28: TIM2 global interrupt
    None,                          // IRQ 29: TIM3 global interrupt
    None,
    None,                          // IRQ 31: I2C1 event interrupt
    None,                          // IRQ 32: I2C1 error interrupt
    None,                          // IRQ 33: I2C2 event interrupt
    None,                          // IRQ 34: I2C2 error interrupt
    None,                          // IRQ 35: SPI1 global interrupt
    None,                          // IRQ 36: SPI2 global interrupt
    None,                          // IRQ 37: USART1 global interrupt
    None,                          // IRQ 38: USART2 global interrupt
    None,                          // IRQ 39: USART3 global interrupt
    None,                          // IRQ 40: EXTI Lines 10 to 15 interrupts
    None,                          // IRQ 41: RTC alarms through EXTI line 18 interrupts
    None,                          // IRQ 42: DFSDM1_FLT3 global interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 49: SDMMC global Interrupt
    None,
    None,                          // IRQ 51: SPI3 global Interrupt
    None,                          // IRQ 52: UART4 global Interrupt
    None,
    None,                          // IRQ 54: TIM6 global and DAC1 and 2 underrun error interrupts
    None,                          // IRQ 55: TIM7 global interrupt
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 61: DFSDM1_FLT0 global interrupt
    None,                          // IRQ 62: DFSDM1_FLT1 global interrupt
    None,                          // IRQ 63: DFSDM1_FLT2 global interrupt
    None,                          // IRQ 64: COMP1 and COMP2 interrupts
    None,                          // IRQ 65: LP TIM1 interrupt
    None,                          // IRQ 66: LP TIM2 interrupt
    None,                          // IRQ 67: USB event interrupt through EXTI
    None,
    None,
    None,                          // IRQ 70: LPUART1 global interrupt
    None,                          // IRQ 71: Quad SPI global interrupt
    None,                          // IRQ 72: I2C3 event interrupt
    None,                          // IRQ 73: I2C3 error interrupt
    None,                          // IRQ 74: SAI1 global interrupt
    None,
    None,                          // IRQ 76: SWPMI1 global interrupt
    None,                          // IRQ 77: TSC global interrupt
    None,                          // IRQ 78: LCD global interrupt
    None,                          // IRQ 79: AES global interrupt
    None,                          // IRQ 80: RNG global interrupt
    None,                          // IRQ 81: Floating point interrupt
    None,                          // IRQ 82: CRS interrupt
    None,                          // IRQ 83: I2C4 event interrupt, wakeup through EXTI
    None,                          // IRQ 84: I2C4 error interrupt
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 85] = [None; 85];

