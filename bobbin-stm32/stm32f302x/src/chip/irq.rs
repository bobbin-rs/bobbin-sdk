//! Interrupts

pub type Handler = extern "C" fn();


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

