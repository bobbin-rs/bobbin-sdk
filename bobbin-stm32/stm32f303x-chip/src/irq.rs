pub type Handler = unsafe extern "C" fn();

pub const IRQ_WWDG: Irq = Irq(0);        // IRQ 0: Window Watchdog interrupt
pub const IRQ_PVD: Irq = Irq(1);         // IRQ 1: PVD through EXTI line detection interrupt
pub const IRQ_TAMP_STAMP: Irq = Irq(2);  // IRQ 2: Tamper and TimeStamp interrupts
pub const IRQ_RTC_WKUP: Irq = Irq(3);    // IRQ 3: RTC Wakeup interrupt through the EXTI line
pub const IRQ_FLASH: Irq = Irq(4);       // IRQ 4: Flash global interrupt
pub const IRQ_RCC: Irq = Irq(5);         // IRQ 5: RCC global interrupt
pub const IRQ_EXTI0: Irq = Irq(6);       // IRQ 6: EXTI Line0 interrupt
pub const IRQ_EXTI1: Irq = Irq(7);       // IRQ 7: EXTI Line3 interrupt
pub const IRQ_EXTI2_TSC: Irq = Irq(8);   // IRQ 8: EXTI Line2 and Touch sensing interrupts
pub const IRQ_EXTI3: Irq = Irq(9);       // IRQ 9: EXTI Line3 interrupt
pub const IRQ_EXTI4: Irq = Irq(10);      // IRQ 10: EXTI Line4 interrupt
pub const IRQ_DMA1_CH1: Irq = Irq(11);   // IRQ 11: DMA1 channel 1 interrupt
pub const IRQ_DMA1_CH2: Irq = Irq(12);   // IRQ 12: DMA1 channel 2 interrupt
pub const IRQ_DMA1_CH3: Irq = Irq(13);   // IRQ 13: DMA1 channel 3 interrupt
pub const IRQ_DMA1_CH4: Irq = Irq(14);   // IRQ 14: DMA1 channel 4 interrupt
pub const IRQ_DMA1_CH5: Irq = Irq(15);   // IRQ 15: DMA1 channel 5 interrupt
pub const IRQ_DMA1_CH6: Irq = Irq(16);   // IRQ 16: DMA1 channel 6 interrupt
pub const IRQ_DMA1_CH7: Irq = Irq(17);   // IRQ 17: DMA1 channel 7interrupt
pub const IRQ_ADC1_2: Irq = Irq(18);     // IRQ 18: ADC1 and ADC2 global interrupt
pub const IRQ_EXTI9_5: Irq = Irq(23);    // IRQ 23: EXTI Line5 to Line9 interrupts
pub const IRQ_TIM1_BRK_TIM15: Irq = Irq(24); // IRQ 24: TIM1 Break/TIM15 global interruts
pub const IRQ_TIM1_UP_TIM16: Irq = Irq(25); // IRQ 25: TIM1 Update/TIM16 global interrupts
pub const IRQ_TIM1_TRG_COM_TIM17: Irq = Irq(26); // IRQ 26: TIM1 trigger and commutation/TIM17 interrupts
pub const IRQ_TIM1_CC: Irq = Irq(27);    // IRQ 27: TIM1 capture compare interrupt
pub const IRQ_TIM2: Irq = Irq(28);       // IRQ 28: TIM2 global interrupt
pub const IRQ_TIM3: Irq = Irq(29);       // IRQ 29: TIM3 global interrupt
pub const IRQ_TIM4: Irq = Irq(30);       // IRQ 30: TIM4 global interrupt
pub const IRQ_I2C1_EV_EXTI23: Irq = Irq(31); // IRQ 31: I2C1 event interrupt and EXTI Line23 interrupt
pub const IRQ_I2C1_ER: Irq = Irq(32);    // IRQ 32: I2C1 error interrupt
pub const IRQ_I2C2_EV_EXTI24: Irq = Irq(33); // IRQ 33: I2C2 event interrupt & EXTI Line24 interrupt
pub const IRQ_I2C2_ER: Irq = Irq(34);    // IRQ 34: I2C2 error interrupt
pub const IRQ_SPI1: Irq = Irq(35);       // IRQ 35: SPI1 global interrupt
pub const IRQ_SPI2: Irq = Irq(36);       // IRQ 36: SPI2 global interrupt
pub const IRQ_USART1_EXTI25: Irq = Irq(37); // IRQ 37: USART1 global interrupt and EXTI Line 25 interrupt
pub const IRQ_USART2_EXTI26: Irq = Irq(38); // IRQ 38: USART2 global interrupt and EXTI Line 26 interrupt
pub const IRQ_USART3_EXTI28: Irq = Irq(39); // IRQ 39: USART3 global interrupt and EXTI Line 28 interrupt
pub const IRQ_EXTI15_10: Irq = Irq(40);  // IRQ 40: EXTI Line15 to Line10 interrupts
pub const IRQ_RTC_ALARM: Irq = Irq(41);  // IRQ 41: RTC alarm interrupt
pub const IRQ_TIM8_BRK: Irq = Irq(43);   // IRQ 43: TIM8 break interrupt
pub const IRQ_TIM8_UP: Irq = Irq(44);    // IRQ 44: TIM8 update interrupt
pub const IRQ_TIM8_TRG_COM: Irq = Irq(45); // IRQ 45: TIM8 Trigger and commutation interrupts
pub const IRQ_TIM8_CC: Irq = Irq(46);    // IRQ 46: TIM8 capture compare interrupt
pub const IRQ_ADC3: Irq = Irq(47);       // IRQ 47: ADC3 global interrupt
pub const IRQ_SPI3: Irq = Irq(51);       // IRQ 51: SPI3 global interrupt
pub const IRQ_UART4_EXTI34: Irq = Irq(52); // IRQ 52: UART4 global and EXTI Line 34 interrupts
pub const IRQ_UART5_EXTI35: Irq = Irq(53); // IRQ 53: UART5 global and EXTI Line 35 interrupts
pub const IRQ_TIM6_DACUNDER: Irq = Irq(54); // IRQ 54: TIM6 global and DAC12 underrun interrupts
pub const IRQ_TIM7: Irq = Irq(55);       // IRQ 55: TIM7 global interrupt
pub const IRQ_DMA2_CH1: Irq = Irq(56);   // IRQ 56: DMA2 channel1 global interrupt
pub const IRQ_DMA2_CH2: Irq = Irq(57);   // IRQ 57: DMA2 channel2 global interrupt
pub const IRQ_DMA2_CH3: Irq = Irq(58);   // IRQ 58: DMA2 channel3 global interrupt
pub const IRQ_DMA2_CH4: Irq = Irq(59);   // IRQ 59: DMA2 channel4 global interrupt
pub const IRQ_DMA2_CH5: Irq = Irq(60);   // IRQ 60: DMA2 channel5 global interrupt
pub const IRQ_ADC4: Irq = Irq(61);       // IRQ 61: ADC4 global interrupt
pub const IRQ_USB_WKUP_EXTI: Irq = Irq(76); // IRQ 76: USB wakeup from Suspend and EXTI Line 18

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
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 82] = [
   None,                          // IRQ 0: Window Watchdog interrupt
   None,                          // IRQ 1: PVD through EXTI line detection interrupt
   None,                          // IRQ 2: Tamper and TimeStamp interrupts
   None,                          // IRQ 3: RTC Wakeup interrupt through the EXTI line
   None,                          // IRQ 4: Flash global interrupt
   None,                          // IRQ 5: RCC global interrupt
   None,                          // IRQ 6: EXTI Line0 interrupt
   None,                          // IRQ 7: EXTI Line3 interrupt
   None,                          // IRQ 8: EXTI Line2 and Touch sensing interrupts
   None,                          // IRQ 9: EXTI Line3 interrupt
   None,                          // IRQ 10: EXTI Line4 interrupt
   None,                          // IRQ 11: DMA1 channel 1 interrupt
   None,                          // IRQ 12: DMA1 channel 2 interrupt
   None,                          // IRQ 13: DMA1 channel 3 interrupt
   None,                          // IRQ 14: DMA1 channel 4 interrupt
   None,                          // IRQ 15: DMA1 channel 5 interrupt
   None,                          // IRQ 16: DMA1 channel 6 interrupt
   None,                          // IRQ 17: DMA1 channel 7interrupt
   None,                          // IRQ 18: ADC1 and ADC2 global interrupt
   None,
   None,
   None,
   None,
   None,                          // IRQ 23: EXTI Line5 to Line9 interrupts
   None,                          // IRQ 24: TIM1 Break/TIM15 global interruts
   None,                          // IRQ 25: TIM1 Update/TIM16 global interrupts
   None,                          // IRQ 26: TIM1 trigger and commutation/TIM17 interrupts
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
   None,                          // IRQ 40: EXTI Line15 to Line10 interrupts
   None,                          // IRQ 41: RTC alarm interrupt
   None,
   None,                          // IRQ 43: TIM8 break interrupt
   None,                          // IRQ 44: TIM8 update interrupt
   None,                          // IRQ 45: TIM8 Trigger and commutation interrupts
   None,                          // IRQ 46: TIM8 capture compare interrupt
   None,                          // IRQ 47: ADC3 global interrupt
   None,
   None,
   None,
   None,                          // IRQ 51: SPI3 global interrupt
   None,                          // IRQ 52: UART4 global and EXTI Line 34 interrupts
   None,                          // IRQ 53: UART5 global and EXTI Line 35 interrupts
   None,                          // IRQ 54: TIM6 global and DAC12 underrun interrupts
   None,                          // IRQ 55: TIM7 global interrupt
   None,                          // IRQ 56: DMA2 channel1 global interrupt
   None,                          // IRQ 57: DMA2 channel2 global interrupt
   None,                          // IRQ 58: DMA2 channel3 global interrupt
   None,                          // IRQ 59: DMA2 channel4 global interrupt
   None,                          // IRQ 60: DMA2 channel5 global interrupt
   None,                          // IRQ 61: ADC4 global interrupt
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
   None,                          // IRQ 76: USB wakeup from Suspend and EXTI Line 18
   None,
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 82] = [None; 82];

