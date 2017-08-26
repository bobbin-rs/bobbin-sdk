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

