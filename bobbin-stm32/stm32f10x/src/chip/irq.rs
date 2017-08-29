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
irq!(IRQ_TIM2, IrqTim2, 28);
irq!(IRQ_TIM3, IrqTim3, 29);
irq!(IRQ_TIM4, IrqTim4, 30);
irq!(IRQ_TIM1_BRK, IrqTim1Brk, 24);
irq!(IRQ_TIM1_UP, IrqTim1Up, 25);
irq!(IRQ_TIM1_TRG_COM, IrqTim1TrgCom, 26);
irq!(IRQ_TIM1_CC, IrqTim1Cc, 27);
irq!(IRQ_ADC, IrqAdc, 18);
irq!(IRQ_ADC2, IrqAdc2, 18);
irq!(IRQ_SPI1_IRQ, IrqSpi1Irq, 35);
irq!(IRQ_SPI2_IRQ, IrqSpi2Irq, 36);
irq!(IRQ_SPI3_IRQ, IrqSpi3Irq, 51);
irq!(IRQ_I2C2_EV, IrqI2c2Ev, 33);
irq!(IRQ_I2C2_ER, IrqI2c2Er, 34);
irq!(IRQ_I2C1_EV, IrqI2c1Ev, 31);
irq!(IRQ_I2C1_ER, IrqI2c1Er, 32);
irq!(IRQ_USART1_IRQ, IrqUsart1Irq, 37);
irq!(IRQ_USART2_IRQ, IrqUsart2Irq, 38);
irq!(IRQ_USART3_IRQ, IrqUsart3Irq, 39);
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
irq!(IRQ_DMA2_CH5, IrqDma2Ch5, 59);


pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { 
     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
     R_INTERRUPT_HANDLERS[index] = handler
  };
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 68] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,
   None,
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
   None,                          // IRQ 18: ADC2 global interrupts
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 24: TIM1 Break interrupt
   None,                          // IRQ 25: TIM1 Update interrupt
   None,                          // IRQ 26: TIM1 Trigger and Commutation interrupt
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
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 48: FSMC global interrupt
   None,
   None,
   None,                          // IRQ 51: SPI3 global interrupt
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
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 68] = [None; 68];

