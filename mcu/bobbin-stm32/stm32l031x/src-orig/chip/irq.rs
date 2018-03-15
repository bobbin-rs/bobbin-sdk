//! Interrupts

#[allow(unused_imports)] use bobbin_common::*;

pub type Handler = extern "C" fn();

irq!(IRQ_WWDG, IrqWwdg, 0);
irq!(IRQ_RTC, IrqRtc, 2);
irq!(IRQ_LPTIM1, IrqLptim1, 13);
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
irq!(IRQ_I2C1, IrqI2c1, 23);
irq!(IRQ_TIM2, IrqTim2, 15);
irq!(IRQ_TIM21, IrqTim21, 20);
irq!(IRQ_TIM22, IrqTim22, 22);
irq!(IRQ_USART2, IrqUsart2, 28);
irq!(IRQ_LPUART1, IrqLpuart1, 29);
irq!(IRQ_SPI1, IrqSpi1, 25);
irq!(IRQ_ADC1, IrqAdc1, 12);

#[cfg_attr(target_os="none", link_section=".vector.interrupts")]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 30] = [
    None,                          // IRQ 0: Window Watchdog interrupt
    None,                          // IRQ 1: PVD through EXTI line detection
    None,                          // IRQ 2: RTC global interrupt
    None,                          // IRQ 3: Flash global interrupt
    None,                          // IRQ 4: RCC global interrupt
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,                          // IRQ 12: ADC 1
    None,                          // IRQ 13: LPTIMER1 interrupt through EXTI29
    None,
    None,                          // IRQ 15: TIM2 global interrupt
    None,
    None,
    None,
    None,
    None,                          // IRQ 20: TIMER21 global interrupt
    None,
    None,                          // IRQ 22: TIMER22 global interrupt
    None,                          // IRQ 23: I2C1 global interrupt
    None,
    None,                          // IRQ 25: SPI1_global_interrupt
    None,
    None,
    None,                          // IRQ 28: USART2 global interrupt through EXTI26
    None,                          // IRQ 29: LPUART1 global interrupt through EXTI28
];

#[cfg_attr(target_os="none", link_section=".bss.r_interrupts")]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 30] = [None; 30];

