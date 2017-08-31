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
irq!(IRQ_EXTI17, IrqExti17, 2);
irq!(IRQ_EXTI18, IrqExti18, 42);
irq!(IRQ_EXTI19, IrqExti19, 2);
irq!(IRQ_EXTI20, IrqExti20, 3);
irq!(IRQ_EXTI21, IrqExti21, 64);
irq!(IRQ_EXTI22, IrqExti22, 64);
irq!(IRQ_EXTI23, IrqExti23, 31);
irq!(IRQ_EXTI24, IrqExti24, 32);
irq!(IRQ_EXTI25, IrqExti25, 37);
irq!(IRQ_EXTI26, IrqExti26, 38);
irq!(IRQ_EXTI28, IrqExti28, 39);
irq!(IRQ_EXTI29, IrqExti29, 64);
irq!(IRQ_EXTI30, IrqExti30, 65);
irq!(IRQ_EXTI31, IrqExti31, 65);
irq!(IRQ_EXTI32, IrqExti32, 65);
irq!(IRQ_EXTI33, IrqExti33, 66);
irq!(IRQ_EXTI34, IrqExti34, 52);
irq!(IRQ_EXTI35, IrqExti35, 53);
irq!(IRQ_USART1_EXTI25, IrqUsart1Exti25, 37);
irq!(IRQ_USART2_EXTI26, IrqUsart2Exti26, 38);
irq!(IRQ_USART3_EXTI28, IrqUsart3Exti28, 39);
irq!(IRQ_UART4_EXTI34, IrqUart4Exti34, 52);
irq!(IRQ_UART5_EXTI35, IrqUart5Exti35, 53);
irq!(IRQ_I2C1_EV, IrqI2c1Ev, 31);
irq!(IRQ_I2C1_ER, IrqI2c1Er, 32);
irq!(IRQ_I2C2_EV, IrqI2c2Ev, 33);
irq!(IRQ_I2C2_ER, IrqI2c2Er, 34);
irq!(IRQ_I2C3_EV, IrqI2c3Ev, 72);
irq!(IRQ_I2C3_ER, IrqI2c3Er, 73);
irq!(IRQ_SPI1, IrqSpi1, 35);
irq!(IRQ_SPI2, IrqSpi2, 36);
irq!(IRQ_SPI3, IrqSpi3, 51);
irq!(IRQ_ADC1, IrqAdc1, 18);
irq!(IRQ_TIM6_DACUNDER, IrqTim6Dacunder, 54);
irq!(IRQ_TIM6, IrqTim6, 54);
irq!(IRQ_TIM7, IrqTim7, 55);
irq!(IRQ_TIM2, IrqTim2, 28);
irq!(IRQ_TIM3, IrqTim3, 29);
irq!(IRQ_TIM4, IrqTim4, 30);
irq!(IRQ_TIM15, IrqTim15, 24);
irq!(IRQ_TIM16, IrqTim16, 25);
irq!(IRQ_TIM17, IrqTim17, 26);
irq!(IRQ_TIM1_BRK, IrqTim1Brk, 24);
irq!(IRQ_TIM1_UP, IrqTim1Up, 25);
irq!(IRQ_TIM1_TRG_COM, IrqTim1TrgCom, 26);
irq!(IRQ_TIM1_CC, IrqTim1Cc, 27);
irq!(IRQ_TIM8_BRK, IrqTim8Brk, 43);
irq!(IRQ_TIM8_UP, IrqTim8Up, 44);
irq!(IRQ_TIM8_TRG_COM, IrqTim8TrgCom, 45);
irq!(IRQ_TIM8_CC, IrqTim8Cc, 46);
irq!(IRQ_TIM20_BRK, IrqTim20Brk, 77);
irq!(IRQ_TIM20_UP, IrqTim20Up, 78);
irq!(IRQ_TIM20_TRG_COM, IrqTim20TrgCom, 79);
irq!(IRQ_TIM20_CC, IrqTim20Cc, 80);
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

pub fn handler(index: usize) -> Option<Handler> {
   unsafe { 
      R_INTERRUPT_HANDLERS[index]
   } 
}

pub fn set_handler(index: usize, handler: Option<Handler>) {
   unsafe { 
      assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
      R_INTERRUPT_HANDLERS[index] = handler
  };
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 82] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,
   None,                          // IRQ 3: RTC Wakeup interrupt through the EXTI line
   None,                          // IRQ 4: Flash global interrupt
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
   None,                          // IRQ 18: ADC1 and ADC2 global interrupt
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 24: TIM1 break interrupt
   None,                          // IRQ 25: TIM1 update interrupt
   None,                          // IRQ 26: TIM1 trigger and commutation interrupt
   None,                          // IRQ 27: TIM1 capture compare interrupt
   None,                          // IRQ 28: TIM2 global interrupt
   None,                          // IRQ 29: TIM3 global interrupt
   None,                          // IRQ 30: TIM4 global interrupt
   None,                          // IRQ 31: I2C1 event interrupt and EXTI Line23 interrupt
   None,                          // IRQ 32: I2C1 error interrupt
   None,                          // IRQ 33: I2C2 event interrupt & EXTI Line24 interrupt
   None,                          // IRQ 34: I2C2 error interrupt
   None,                          // IRQ 35: SPI1 global interrupt
   None,                          // IRQ 36: SPI2 global interrupt
   None,                          // IRQ 37: USART1 global interrupt and EXTI Line 25 interrupt
   None,                          // IRQ 38: USART2 global interrupt and EXTI Line 26 interrupt
   None,                          // IRQ 39: USART3 global interrupt and EXTI Line 28 interrupt
   None,
   None,                          // IRQ 41: RTC alarm interrupt
   None,
   None,                          // IRQ 43: TIM8 break interrupt
   None,                          // IRQ 44: TIM8 update interrupt
   None,                          // IRQ 45: TIM8 Trigger and commutation interrupts
   None,                          // IRQ 46: TIM8 capture compare interrupt
   None,
   None,
   None,
   None,
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global and EXTI Line 34 interrupts
   None,                          // IRQ 53: UART5 global and EXTI Line 35 interrupts
   None,                          // IRQ 54: TIM6 global interrupt
   None,                          // IRQ 55: TIM7 global interrupt
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
   None,
   None,
   None,
   None,
   None,                          // IRQ 72: I2C3 event interrupt
   None,                          // IRQ 73: I2C3 error interrupt
   None,
   None,
   None,
   None,                          // IRQ 77: TIM20 break interrupt
   None,                          // IRQ 78: TIM20 update interrupt
   None,                          // IRQ 79: TIM20 Trigger and commutation interrupts
   None,                          // IRQ 80: TIM20 capture compare interrupt
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 82] = [None; 82];

