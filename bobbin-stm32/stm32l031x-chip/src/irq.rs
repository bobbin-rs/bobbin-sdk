pub type Handler = unsafe extern "C" fn();

pub const IRQ_WWDG: Irq = Irq(0);        // IRQ 0: Window Watchdog interrupt
pub const IRQ_PVD: Irq = Irq(1);         // IRQ 1: PVD through EXTI line detection
pub const IRQ_RTC: Irq = Irq(2);         // IRQ 2: RTC global interrupt
pub const IRQ_FLASH: Irq = Irq(3);       // IRQ 3: Flash global interrupt
pub const IRQ_RCC: Irq = Irq(4);         // IRQ 4: RCC global interrupt
pub const IRQ_EXTI0_1: Irq = Irq(5);     // IRQ 5: EXTI Line[1:0] interrupts
pub const IRQ_EXTI2_3: Irq = Irq(6);     // IRQ 6: EXTI Line[3:2] interrupts
pub const IRQ_EXTI4_15: Irq = Irq(7);    // IRQ 7: EXTI Line15 and EXTI4 interrupts
pub const IRQ_DMA1_CHANNEL1: Irq = Irq(9); // IRQ 9: DMA1 Channel1 global interrupt
pub const IRQ_DMA1_CHANNEL2_3: Irq = Irq(10); // IRQ 10: DMA1 Channel2 and 3 interrupts
pub const IRQ_DMA1_CHANNEL4_7: Irq = Irq(11); // IRQ 11: DMA1 Channel4 to 7 interrupts
pub const IRQ_LPTIM1: Irq = Irq(13);     // IRQ 13: LPTIMER1 interrupt through EXTI29
pub const IRQ_TIM2: Irq = Irq(15);       // IRQ 15: TIM2 global interrupt
pub const IRQ_TIM21: Irq = Irq(20);      // IRQ 20: TIMER21 global interrupt
pub const IRQ_TIM22: Irq = Irq(22);      // IRQ 22: TIMER22 global interrupt
pub const IRQ_I2C1: Irq = Irq(23);       // IRQ 23: I2C1 global interrupt
pub const IRQ_SPI1: Irq = Irq(25);       // IRQ 25: SPI1_global_interrupt
pub const IRQ_USART2: Irq = Irq(28);     // IRQ 28: USART2 global interrupt
pub const IRQ_RNG_LPUART1: Irq = Irq(29); // IRQ 29: RNG global interrupt and LPUART1 global interrupt through

pub fn set_handler(irq: Irq, handler: Option<Handler>) {
  unsafe { R_INTERRUPT_HANDLERS[irq.0] = handler };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq(pub usize);

impl Irq {
   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { R_INTERRUPT_HANDLERS[self.0] = handler };
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 30] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection
   None,                          // IRQ 2: RTC global interrupt
   None,                          // IRQ 3: Flash global interrupt
   None,                          // IRQ 4: RCC global interrupt
   None,                          // IRQ 5: EXTI Line[1:0] interrupts
   None,                          // IRQ 6: EXTI Line[3:2] interrupts
   None,                          // IRQ 7: EXTI Line15 and EXTI4 interrupts
   None,
   None,                          // IRQ 9: DMA1 Channel1 global interrupt
   None,                          // IRQ 10: DMA1 Channel2 and 3 interrupts
   None,                          // IRQ 11: DMA1 Channel4 to 7 interrupts
   None,
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
   None,                          // IRQ 28: USART2 global interrupt
   None,                          // IRQ 29: RNG global interrupt and LPUART1 global interrupt through
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 30] = [None; 30];

