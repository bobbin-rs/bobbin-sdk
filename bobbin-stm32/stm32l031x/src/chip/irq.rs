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

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 30] = [None; 30];

