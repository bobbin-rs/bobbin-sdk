//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_RNG, IrqRng, 79);
irq!(IRQ_WWDG, IrqWwdg, 0);
irq!(IRQ_RTC, IrqRtc, 2);
irq!(IRQ_LPTIM1, IrqLptim1, 65);
irq!(IRQ_LPTIM2, IrqLptim2, 66);
irq!(IRQ_EXTI0, IrqExti0, 5);
irq!(IRQ_EXTI1, IrqExti1, 5);
irq!(IRQ_EXTI2, IrqExti2, 6);
irq!(IRQ_EXTI3, IrqExti3, 6);
irq!(IRQ_EXTI4, IrqExti4, 7);
irq!(IRQ_EXTI5, IrqExti5, 7);
irq!(IRQ_EXTI6, IrqExti6, 7);
irq!(IRQ_EXTI7, IrqExti7, 7);
irq!(IRQ_EXTI8, IrqExti8, 7);
irq!(IRQ_EXTI9, IrqExti9, 7);
irq!(IRQ_EXTI10, IrqExti10, 7);
irq!(IRQ_EXTI11, IrqExti11, 7);
irq!(IRQ_EXTI12, IrqExti12, 7);
irq!(IRQ_EXTI13, IrqExti13, 7);
irq!(IRQ_EXTI14, IrqExti14, 7);
irq!(IRQ_EXTI15, IrqExti15, 7);
irq!(IRQ_EXTI16, IrqExti16, 1);
irq!(IRQ_EXTI17, IrqExti17, 2);
irq!(IRQ_EXTI19, IrqExti19, 2);
irq!(IRQ_EXTI20, IrqExti20, 2);
irq!(IRQ_EXTI21, IrqExti21, 12);
irq!(IRQ_EXTI22, IrqExti22, 12);
irq!(IRQ_EXTI23, IrqExti23, 23);
irq!(IRQ_EXTI24, IrqExti24, 24);
irq!(IRQ_EXTI25, IrqExti25, 27);
irq!(IRQ_EXTI26, IrqExti26, 28);
irq!(IRQ_EXTI28, IrqExti28, 29);
irq!(IRQ_EXTI29, IrqExti29, 13);
irq!(IRQ_DMA1_CH1, IrqDma1Ch1, 9);
irq!(IRQ_DMA1_CH2, IrqDma1Ch2, 10);
irq!(IRQ_DMA1_CH3, IrqDma1Ch3, 10);
irq!(IRQ_DMA1_CH4, IrqDma1Ch4, 11);
irq!(IRQ_DMA1_CH5, IrqDma1Ch5, 11);
irq!(IRQ_DMA1_CH6, IrqDma1Ch6, 11);
irq!(IRQ_DMA1_CH7, IrqDma1Ch7, 11);
irq!(IRQ_DMA2_CH1, IrqDma2Ch1, 56);
irq!(IRQ_DMA2_CH2, IrqDma2Ch2, 57);
irq!(IRQ_DMA2_CH3, IrqDma2Ch3, 58);
irq!(IRQ_DMA2_CH4, IrqDma2Ch4, 59);
irq!(IRQ_DMA2_CH5, IrqDma2Ch5, 60);
irq!(IRQ_DMA2_CH6, IrqDma2Ch6, 68);
irq!(IRQ_DMA2_CH7, IrqDma2Ch7, 69);
irq!(IRQ_I2C1_EV, IrqI2c1Ev, 31);
irq!(IRQ_I2C1_ER, IrqI2c1Er, 32);
irq!(IRQ_I2C2_EV, IrqI2c2Ev, 33);
irq!(IRQ_I2C2_ER, IrqI2c2Er, 34);
irq!(IRQ_I2C3_EV, IrqI2c3Ev, 72);
irq!(IRQ_I2C3_ER, IrqI2c3Er, 73);
irq!(IRQ_TIM1_CC, IrqTim1Cc, 27);
irq!(IRQ_TIM8_BRK, IrqTim8Brk, 43);
irq!(IRQ_TIM8, IrqTim8, 44);
irq!(IRQ_TIM8_TRG_COM, IrqTim8TrgCom, 45);
irq!(IRQ_TIM8_CC, IrqTim8Cc, 46);
irq!(IRQ_TIM6_DAC, IrqTim6Dac, 54);
irq!(IRQ_TIM7, IrqTim7, 55);
irq!(IRQ_TIM2, IrqTim2, 28);
irq!(IRQ_TIM3, IrqTim3, 29);
irq!(IRQ_TIM4, IrqTim4, 30);
irq!(IRQ_TIM5, IrqTim5, 50);
irq!(IRQ_TIM15, IrqTim15, 24);
irq!(IRQ_TIM16, IrqTim16, 25);
irq!(IRQ_USART1, IrqUsart1, 37);
irq!(IRQ_USART2, IrqUsart2, 38);
irq!(IRQ_USART3, IrqUsart3, 39);
irq!(IRQ_UART4, IrqUart4, 52);
irq!(IRQ_UART5, IrqUart5, 53);
irq!(IRQ_LPUART1, IrqLpuart1, 77);
irq!(IRQ_SPI1, IrqSpi1, 35);
irq!(IRQ_SPI2, IrqSpi2, 36);
irq!(IRQ_SPI3, IrqSpi3, 51);
irq!(IRQ_ADC1, IrqAdc1, 18);
irq!(IRQ_ADC2, IrqAdc2, 18);
irq!(IRQ_ADC3, IrqAdc3, 47);

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

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 84] = [
    None,                          // IRQ 0: Window Watchdog interrupt
    None,
    None,                          // IRQ 2: RTC global interrupt
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
    None,                          // IRQ 18: ADC1 and ADC2 global interrupt
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 24: Timer 15 global interrupt
    None,                          // IRQ 25: Timer 16 global interrupt
    None,
    None,                          // IRQ 27: TIM1 Capture Compare interrupt
    None,                          // IRQ 28: TIM2 global interrupt
    None,                          // IRQ 29: TIM3 global interrupt
    None,                          // IRQ 30: TIM4 global interrupt
    None,                          // IRQ 31: I2C1 event interrupt
    None,                          // IRQ 32: I2C1 error interrupt
    None,                          // IRQ 33: I2C2 event interrupt
    None,                          // IRQ 34: I2C2 error interrupt
    None,                          // IRQ 35: SPI1_global_interrupt
    None,                          // IRQ 36: SPI2 global interrupt
    None,                          // IRQ 37: USART1 global interrupt
    None,                          // IRQ 38: USART2 global interrupt
    None,                          // IRQ 39: USART3 global interrupt
    None,
    None,
    None,
    None,                          // IRQ 43: TIM8 Break Interrupt
    None,                          // IRQ 44: TIM8 Update Interrupt
    None,                          // IRQ 45: TIM8 Trigger and Commutation Interrupt
    None,                          // IRQ 46: TIM8 Capture Compare Interrupt
    None,                          // IRQ 47: ADC3 global Interrupt
    None,
    None,
    None,                          // IRQ 50: TIM5 global Interrupt
    None,                          // IRQ 51: SPI3 global Interrupt
    None,                          // IRQ 52: UART4 global Interrupt
    None,                          // IRQ 53: UART5 global Interrupt
    None,                          // IRQ 54: TIM6 global and DAC1 and 2 underrun error interrupts
    None,                          // IRQ 55: TIM7 global interrupt
    None,                          // IRQ 56: DMA2 Channel 1 global Interrupt
    None,                          // IRQ 57: DMA2 Channel 2 global Interrupt
    None,                          // IRQ 58: DMA2 Channel 3 global Interrupt
    None,                          // IRQ 59: DMA2 Channel 4 global Interrupt
    None,                          // IRQ 60: DMA2 Channel 5 global Interrupt
    None,
    None,
    None,
    None,
    None,                          // IRQ 65: LP TIM1 interrupt
    None,                          // IRQ 66: LP TIM2 interrupt
    None,
    None,                          // IRQ 68: DMA2 Channel 6 global Interrupt
    None,                          // IRQ 69: DMA2 Channel 7 global Interrupt
    None,
    None,
    None,                          // IRQ 72: I2C3 event interrupt
    None,                          // IRQ 73: I2C3 error interrupt
    None,
    None,
    None,
    None,                          // IRQ 77: LPUART1
    None,
    None,                          // IRQ 79: RNG global interrupt
    None,
    None,
    None,
    None,
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 84] = [None; 84];

