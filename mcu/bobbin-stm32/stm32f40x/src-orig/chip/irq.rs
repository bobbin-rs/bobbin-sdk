//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_WWDG, IrqWwdg, 0);
irq!(IRQ_EXTI0, IrqExti0, 6);
irq!(IRQ_EXTI1, IrqExti1, 7);
irq!(IRQ_EXTI2, IrqExti2, 8);
irq!(IRQ_EXTI3, IrqExti3, 9);
irq!(IRQ_EXTI4, IrqExti4, 10);
irq!(IRQ_EXTI5, IrqExti5, 23);
irq!(IRQ_EXTI6, IrqExti6, 23);
irq!(IRQ_EXTI7, IrqExti7, 23);
irq!(IRQ_EXTI8, IrqExti8, 23);
irq!(IRQ_EXTI9, IrqExti9, 23);
irq!(IRQ_EXTI10, IrqExti10, 40);
irq!(IRQ_EXTI11, IrqExti11, 40);
irq!(IRQ_EXTI12, IrqExti12, 40);
irq!(IRQ_EXTI13, IrqExti13, 40);
irq!(IRQ_EXTI14, IrqExti14, 40);
irq!(IRQ_EXTI15, IrqExti15, 40);
irq!(IRQ_EXTI16, IrqExti16, 1);
irq!(IRQ_EXTI17, IrqExti17, 41);
irq!(IRQ_EXTI18, IrqExti18, 42);
irq!(IRQ_EXTI19, IrqExti19, 62);
irq!(IRQ_EXTI20, IrqExti20, 76);
irq!(IRQ_EXTI21, IrqExti21, 2);
irq!(IRQ_EXTI22, IrqExti22, 3);
irq!(IRQ_TIM6, IrqTim6, 54);
irq!(IRQ_TIM7, IrqTim7, 55);
irq!(IRQ_TIM2, IrqTim2, 28);
irq!(IRQ_TIM3, IrqTim3, 29);
irq!(IRQ_TIM4, IrqTim4, 30);
irq!(IRQ_TIM5, IrqTim5, 50);
irq!(IRQ_TIM9, IrqTim9, 24);
irq!(IRQ_TIM10, IrqTim10, 25);
irq!(IRQ_TIM11, IrqTim11, 26);
irq!(IRQ_TIM12, IrqTim12, 43);
irq!(IRQ_TIM13, IrqTim13, 44);
irq!(IRQ_TIM14, IrqTim14, 45);
irq!(IRQ_TIM1_BRK, IrqTim1Brk, 24);
irq!(IRQ_TIM1_UP, IrqTim1Up, 25);
irq!(IRQ_TIM1_TRG_COM, IrqTim1TrgCom, 26);
irq!(IRQ_TIM1_CC, IrqTim1Cc, 27);
irq!(IRQ_TIM8_BRK, IrqTim8Brk, 43);
irq!(IRQ_TIM8_UP, IrqTim8Up, 44);
irq!(IRQ_TIM8_TRG_COM, IrqTim8TrgCom, 45);
irq!(IRQ_TIM8_CC, IrqTim8Cc, 46);
irq!(IRQ_ADC1, IrqAdc1, 18);
irq!(IRQ_ADC2, IrqAdc2, 18);
irq!(IRQ_ADC3, IrqAdc3, 18);
irq!(IRQ_SPI1, IrqSpi1, 35);
irq!(IRQ_SPI2, IrqSpi2, 36);
irq!(IRQ_SPI3, IrqSpi3, 51);
irq!(IRQ_SPI4, IrqSpi4, 84);
irq!(IRQ_SPI5, IrqSpi5, 85);
irq!(IRQ_SPI6, IrqSpi6, 86);
irq!(IRQ_I2C3_EV, IrqI2c3Ev, 72);
irq!(IRQ_I2C3_ER, IrqI2c3Er, 73);
irq!(IRQ_I2C2_EV, IrqI2c2Ev, 33);
irq!(IRQ_I2C2_ER, IrqI2c2Er, 34);
irq!(IRQ_I2C1_EV, IrqI2c1Ev, 31);
irq!(IRQ_I2C1_ER, IrqI2c1Er, 32);
irq!(IRQ_USART1, IrqUsart1, 37);
irq!(IRQ_USART2, IrqUsart2, 38);
irq!(IRQ_USART3, IrqUsart3, 39);
irq!(IRQ_UART4, IrqUart4, 52);
irq!(IRQ_UART5, IrqUart5, 53);
irq!(IRQ_USART6, IrqUsart6, 71);
irq!(IRQ_DMA1_STREAM0, IrqDma1Stream0, 11);
irq!(IRQ_DMA1_STREAM1, IrqDma1Stream1, 12);
irq!(IRQ_DMA1_STREAM2, IrqDma1Stream2, 13);
irq!(IRQ_DMA1_STREAM3, IrqDma1Stream3, 14);
irq!(IRQ_DMA1_STREAM4, IrqDma1Stream4, 15);
irq!(IRQ_DMA1_STREAM5, IrqDma1Stream5, 16);
irq!(IRQ_DMA1_STREAM6, IrqDma1Stream6, 17);
irq!(IRQ_DMA1_STREAM7, IrqDma1Stream7, 47);
irq!(IRQ_DMA2_STREAM0, IrqDma2Stream0, 56);
irq!(IRQ_DMA2_STREAM1, IrqDma2Stream1, 57);
irq!(IRQ_DMA2_STREAM2, IrqDma2Stream2, 58);
irq!(IRQ_DMA2_STREAM3, IrqDma2Stream3, 59);
irq!(IRQ_DMA2_STREAM4, IrqDma2Stream4, 60);
irq!(IRQ_DMA2_STREAM5, IrqDma2Stream5, 68);
irq!(IRQ_DMA2_STREAM6, IrqDma2Stream6, 69);
irq!(IRQ_DMA2_STREAM7, IrqDma2Stream7, 70);

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 91] = [
    None,                          // IRQ 0: Window Watchdog interrupt
    None,                          // IRQ 1: PVD through EXTI line detection interrupt
    None,
    None,
    None,
    None,                          // IRQ 5: RCC global interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 18: ADC3 global interrupts
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 24: TIM1 Break interrupt
    None,                          // IRQ 25: TIM1 Update interrupt
    None,                          // IRQ 26: TIM1 Trigger and Commutation interrupts
    None,                          // IRQ 27: TIM1 Capture Compare interrupt
    None,                          // IRQ 28: TIM2 global interrupt
    None,                          // IRQ 29: TIM3 global interrupt
    None,                          // IRQ 30: TIM4 global interrupt
    None,                          // IRQ 31: I2C1 event interrupt
    None,                          // IRQ 32: I2C1 error interrupt
    None,                          // IRQ 33: I2C2 event interrupt
    None,                          // IRQ 34: I2C2 error interrupt
    None,                          // IRQ 35: SPI1 global interrupt
    None,                          // IRQ 36: SPI2 global interrupt
    None,                          // IRQ 37: USART1 global interrupt
    None,                          // IRQ 38: USART2 global interrupt
    None,                          // IRQ 39: USART3 global interrupt
    None,
    None,
    None,
    None,                          // IRQ 43: TIM8 Break interrupt and TIM12 global interrupt
    None,                          // IRQ 44: TIM8 Update interrupt and TIM13 global interrupt
    None,                          // IRQ 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
    None,                          // IRQ 46: TIM8 Capture Compare interrupt
    None,
    None,
    None,
    None,                          // IRQ 50: TIM5 global interrupt
    None,                          // IRQ 51: SPI3 global interrupt
    None,                          // IRQ 52: UART4 global interrupt
    None,                          // IRQ 53: UART5 global interrupt
    None,                          // IRQ 54: TIM6 global interrupt
    None,                          // IRQ 55: TIM7 global interrupt
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 61: Ethernet global interrupt
    None,                          // IRQ 62: Ethernet Wakeup through EXTI line interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 71: USART6 global interrupt
    None,                          // IRQ 72: I2C3 event interrupt
    None,                          // IRQ 73: I2C3 error interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 84: SPI 4 global interrupt
    None,                          // IRQ 85: SPI 5 global interrupt
    None,                          // IRQ 86: SPI 6 global interrupt
    None,
    None,
    None,
    None,
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 91] = [None; 91];

