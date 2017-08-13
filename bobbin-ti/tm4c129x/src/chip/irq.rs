//! Interrupts

use ::core::marker::PhantomData;
pub type Handler = extern "C" fn();

pub const IRQ_ADC0SS0: IrqAdc0ss0 = Irq(14, Adc0ss0Id {});
pub const IRQ_ADC0SS1: IrqAdc0ss1 = Irq(15, Adc0ss1Id {});
pub const IRQ_ADC0SS2: IrqAdc0ss2 = Irq(16, Adc0ss2Id {});
pub const IRQ_ADC0SS3: IrqAdc0ss3 = Irq(17, Adc0ss3Id {});
pub const IRQ_ADC1SS0: IrqAdc1ss0 = Irq(46, Adc1ss0Id {});
pub const IRQ_ADC1SS1: IrqAdc1ss1 = Irq(47, Adc1ss1Id {});
pub const IRQ_ADC1SS2: IrqAdc1ss2 = Irq(48, Adc1ss2Id {});
pub const IRQ_ADC1SS3: IrqAdc1ss3 = Irq(49, Adc1ss3Id {});
pub const IRQ_WATCHDOG0: IrqWatchdog0 = Irq(18, Watchdog0Id {});
pub const IRQ_UDMA: IrqUdma = Irq(44, UdmaId {});
pub const IRQ_UDMAERR: IrqUdmaerr = Irq(45, UdmaerrId {});
pub const IRQ_PWM0_FAULT: IrqPwm0Fault = Irq(9, Pwm0FaultId {});
pub const IRQ_PWM0_CH0: IrqPwm0Ch0 = Irq(10, Pwm0Ch0Id {});
pub const IRQ_PWM0_CH1: IrqPwm0Ch1 = Irq(11, Pwm0Ch1Id {});
pub const IRQ_PWM0_CH2: IrqPwm0Ch2 = Irq(12, Pwm0Ch2Id {});
pub const IRQ_PWM0_CH3: IrqPwm0Ch3 = Irq(43, Pwm0Ch3Id {});
pub const IRQ_TIMER0A: IrqTimer0a = Irq(19, Timer0aId {});
pub const IRQ_TIMER0B: IrqTimer0b = Irq(20, Timer0bId {});
pub const IRQ_TIMER1A: IrqTimer1a = Irq(21, Timer1aId {});
pub const IRQ_TIMER1B: IrqTimer1b = Irq(22, Timer1bId {});
pub const IRQ_TIMER2A: IrqTimer2a = Irq(23, Timer2aId {});
pub const IRQ_TIMER2B: IrqTimer2b = Irq(24, Timer2bId {});
pub const IRQ_TIMER3A: IrqTimer3a = Irq(35, Timer3aId {});
pub const IRQ_TIMER3B: IrqTimer3b = Irq(36, Timer3bId {});
pub const IRQ_TIMER4A: IrqTimer4a = Irq(63, Timer4aId {});
pub const IRQ_TIMER4B: IrqTimer4b = Irq(64, Timer4bId {});
pub const IRQ_TIMER5A: IrqTimer5a = Irq(65, Timer5aId {});
pub const IRQ_TIMER5B: IrqTimer5b = Irq(66, Timer5bId {});
pub const IRQ_TIMER6A: IrqTimer6a = Irq(98, Timer6aId {});
pub const IRQ_TIMER6B: IrqTimer6b = Irq(99, Timer6bId {});
pub const IRQ_TIMER7A: IrqTimer7a = Irq(100, Timer7aId {});
pub const IRQ_TIMER7B: IrqTimer7b = Irq(101, Timer7bId {});
pub const IRQ_UART0: IrqUart0 = Irq(5, Uart0Id {});
pub const IRQ_UART1: IrqUart1 = Irq(6, Uart1Id {});
pub const IRQ_UART2: IrqUart2 = Irq(33, Uart2Id {});
pub const IRQ_UART3: IrqUart3 = Irq(56, Uart3Id {});
pub const IRQ_UART4: IrqUart4 = Irq(57, Uart4Id {});
pub const IRQ_UART5: IrqUart5 = Irq(58, Uart5Id {});
pub const IRQ_UART6: IrqUart6 = Irq(59, Uart6Id {});
pub const IRQ_UART7: IrqUart7 = Irq(60, Uart7Id {});
pub const IRQ_I2C0: IrqI2c0 = Irq(8, I2c0Id {});
pub const IRQ_I2C1: IrqI2c1 = Irq(37, I2c1Id {});
pub const IRQ_I2C2: IrqI2c2 = Irq(61, I2c2Id {});
pub const IRQ_I2C3: IrqI2c3 = Irq(62, I2c3Id {});
pub const IRQ_SSI0: IrqSsi0 = Irq(7, Ssi0Id {});
pub const IRQ_SSI1: IrqSsi1 = Irq(34, Ssi1Id {});
pub const IRQ_SSI2: IrqSsi2 = Irq(54, Ssi2Id {});
pub const IRQ_SSI3: IrqSsi3 = Irq(55, Ssi3Id {});
pub const IRQ_GPIOA: IrqGpioa = Irq(0, GpioaId {});
pub const IRQ_GPIOB: IrqGpiob = Irq(1, GpiobId {});
pub const IRQ_GPIOC: IrqGpioc = Irq(2, GpiocId {});
pub const IRQ_GPIOD: IrqGpiod = Irq(3, GpiodId {});
pub const IRQ_GPIOE: IrqGpioe = Irq(4, GpioeId {});
pub const IRQ_GPIOF: IrqGpiof = Irq(30, GpiofId {});
pub const IRQ_GPIOG: IrqGpiog = Irq(31, GpiogId {});
pub const IRQ_GPIOH: IrqGpioh = Irq(32, GpiohId {});
pub const IRQ_GPIOJ: IrqGpioj = Irq(54, GpiojId {});
pub const IRQ_GPIOA_AHB: IrqGpioaAhb = Irq(0, GpioaAhbId {});
pub const IRQ_GPIOB_AHB: IrqGpiobAhb = Irq(1, GpiobAhbId {});
pub const IRQ_GPIOC_AHB: IrqGpiocAhb = Irq(2, GpiocAhbId {});
pub const IRQ_GPIOD_AHB: IrqGpiodAhb = Irq(3, GpiodAhbId {});
pub const IRQ_GPIOE_AHB: IrqGpioeAhb = Irq(4, GpioeAhbId {});
pub const IRQ_GPIOF_AHB: IrqGpiofAhb = Irq(30, GpiofAhbId {});
pub const IRQ_GPIOG_AHB: IrqGpiogAhb = Irq(31, GpiogAhbId {});
pub const IRQ_GPIOH_AHB: IrqGpiohAhb = Irq(32, GpiohAhbId {});
pub const IRQ_GPIOJ_AHB: IrqGpiojAhb = Irq(54, GpiojAhbId {});
pub const IRQ_GPIOK: IrqGpiok = Irq(55, GpiokId {});
pub const IRQ_GPIOL: IrqGpiol = Irq(53, GpiolId {});
pub const IRQ_GPIOM: IrqGpiom = Irq(72, GpiomId {});
pub const IRQ_GPION: IrqGpion = Irq(73, GpionId {});
pub const IRQ_GPIOP0: IrqGpiop0 = Irq(76, Gpiop0Id {});
pub const IRQ_GPIOP1: IrqGpiop1 = Irq(77, Gpiop1Id {});
pub const IRQ_GPIOP2: IrqGpiop2 = Irq(78, Gpiop2Id {});
pub const IRQ_GPIOP3: IrqGpiop3 = Irq(79, Gpiop3Id {});
pub const IRQ_GPIOP4: IrqGpiop4 = Irq(80, Gpiop4Id {});
pub const IRQ_GPIOP5: IrqGpiop5 = Irq(81, Gpiop5Id {});
pub const IRQ_GPIOP6: IrqGpiop6 = Irq(82, Gpiop6Id {});
pub const IRQ_GPIOP7: IrqGpiop7 = Irq(83, Gpiop7Id {});
pub const IRQ_GPIOQ0: IrqGpioq0 = Irq(84, Gpioq0Id {});
pub const IRQ_GPIOQ1: IrqGpioq1 = Irq(85, Gpioq1Id {});
pub const IRQ_GPIOQ2: IrqGpioq2 = Irq(86, Gpioq2Id {});
pub const IRQ_GPIOQ3: IrqGpioq3 = Irq(87, Gpioq3Id {});
pub const IRQ_GPIOQ4: IrqGpioq4 = Irq(88, Gpioq4Id {});
pub const IRQ_GPIOQ5: IrqGpioq5 = Irq(89, Gpioq5Id {});
pub const IRQ_GPIOQ6: IrqGpioq6 = Irq(90, Gpioq6Id {});
pub const IRQ_GPIOQ7: IrqGpioq7 = Irq(91, Gpioq7Id {});

pub type IrqAdc0ss0 = Irq<Adc0ss0Id>;
pub type IrqAdc0ss1 = Irq<Adc0ss1Id>;
pub type IrqAdc0ss2 = Irq<Adc0ss2Id>;
pub type IrqAdc0ss3 = Irq<Adc0ss3Id>;
pub type IrqAdc1ss0 = Irq<Adc1ss0Id>;
pub type IrqAdc1ss1 = Irq<Adc1ss1Id>;
pub type IrqAdc1ss2 = Irq<Adc1ss2Id>;
pub type IrqAdc1ss3 = Irq<Adc1ss3Id>;
pub type IrqWatchdog0 = Irq<Watchdog0Id>;
pub type IrqUdma = Irq<UdmaId>;
pub type IrqUdmaerr = Irq<UdmaerrId>;
pub type IrqPwm0Fault = Irq<Pwm0FaultId>;
pub type IrqPwm0Ch0 = Irq<Pwm0Ch0Id>;
pub type IrqPwm0Ch1 = Irq<Pwm0Ch1Id>;
pub type IrqPwm0Ch2 = Irq<Pwm0Ch2Id>;
pub type IrqPwm0Ch3 = Irq<Pwm0Ch3Id>;
pub type IrqTimer0a = Irq<Timer0aId>;
pub type IrqTimer0b = Irq<Timer0bId>;
pub type IrqTimer1a = Irq<Timer1aId>;
pub type IrqTimer1b = Irq<Timer1bId>;
pub type IrqTimer2a = Irq<Timer2aId>;
pub type IrqTimer2b = Irq<Timer2bId>;
pub type IrqTimer3a = Irq<Timer3aId>;
pub type IrqTimer3b = Irq<Timer3bId>;
pub type IrqTimer4a = Irq<Timer4aId>;
pub type IrqTimer4b = Irq<Timer4bId>;
pub type IrqTimer5a = Irq<Timer5aId>;
pub type IrqTimer5b = Irq<Timer5bId>;
pub type IrqTimer6a = Irq<Timer6aId>;
pub type IrqTimer6b = Irq<Timer6bId>;
pub type IrqTimer7a = Irq<Timer7aId>;
pub type IrqTimer7b = Irq<Timer7bId>;
pub type IrqUart0 = Irq<Uart0Id>;
pub type IrqUart1 = Irq<Uart1Id>;
pub type IrqUart2 = Irq<Uart2Id>;
pub type IrqUart3 = Irq<Uart3Id>;
pub type IrqUart4 = Irq<Uart4Id>;
pub type IrqUart5 = Irq<Uart5Id>;
pub type IrqUart6 = Irq<Uart6Id>;
pub type IrqUart7 = Irq<Uart7Id>;
pub type IrqI2c0 = Irq<I2c0Id>;
pub type IrqI2c1 = Irq<I2c1Id>;
pub type IrqI2c2 = Irq<I2c2Id>;
pub type IrqI2c3 = Irq<I2c3Id>;
pub type IrqSsi0 = Irq<Ssi0Id>;
pub type IrqSsi1 = Irq<Ssi1Id>;
pub type IrqSsi2 = Irq<Ssi2Id>;
pub type IrqSsi3 = Irq<Ssi3Id>;
pub type IrqGpioa = Irq<GpioaId>;
pub type IrqGpiob = Irq<GpiobId>;
pub type IrqGpioc = Irq<GpiocId>;
pub type IrqGpiod = Irq<GpiodId>;
pub type IrqGpioe = Irq<GpioeId>;
pub type IrqGpiof = Irq<GpiofId>;
pub type IrqGpiog = Irq<GpiogId>;
pub type IrqGpioh = Irq<GpiohId>;
pub type IrqGpioj = Irq<GpiojId>;
pub type IrqGpioaAhb = Irq<GpioaAhbId>;
pub type IrqGpiobAhb = Irq<GpiobAhbId>;
pub type IrqGpiocAhb = Irq<GpiocAhbId>;
pub type IrqGpiodAhb = Irq<GpiodAhbId>;
pub type IrqGpioeAhb = Irq<GpioeAhbId>;
pub type IrqGpiofAhb = Irq<GpiofAhbId>;
pub type IrqGpiogAhb = Irq<GpiogAhbId>;
pub type IrqGpiohAhb = Irq<GpiohAhbId>;
pub type IrqGpiojAhb = Irq<GpiojAhbId>;
pub type IrqGpiok = Irq<GpiokId>;
pub type IrqGpiol = Irq<GpiolId>;
pub type IrqGpiom = Irq<GpiomId>;
pub type IrqGpion = Irq<GpionId>;
pub type IrqGpiop0 = Irq<Gpiop0Id>;
pub type IrqGpiop1 = Irq<Gpiop1Id>;
pub type IrqGpiop2 = Irq<Gpiop2Id>;
pub type IrqGpiop3 = Irq<Gpiop3Id>;
pub type IrqGpiop4 = Irq<Gpiop4Id>;
pub type IrqGpiop5 = Irq<Gpiop5Id>;
pub type IrqGpiop6 = Irq<Gpiop6Id>;
pub type IrqGpiop7 = Irq<Gpiop7Id>;
pub type IrqGpioq0 = Irq<Gpioq0Id>;
pub type IrqGpioq1 = Irq<Gpioq1Id>;
pub type IrqGpioq2 = Irq<Gpioq2Id>;
pub type IrqGpioq3 = Irq<Gpioq3Id>;
pub type IrqGpioq4 = Irq<Gpioq4Id>;
pub type IrqGpioq5 = Irq<Gpioq5Id>;
pub type IrqGpioq6 = Irq<Gpioq6Id>;
pub type IrqGpioq7 = Irq<Gpioq7Id>;

#[doc(hidden)]
pub struct Adc0ss0Id {} // IRQ 14
#[doc(hidden)]
pub struct Adc0ss1Id {} // IRQ 15
#[doc(hidden)]
pub struct Adc0ss2Id {} // IRQ 16
#[doc(hidden)]
pub struct Adc0ss3Id {} // IRQ 17
#[doc(hidden)]
pub struct Adc1ss0Id {} // IRQ 46
#[doc(hidden)]
pub struct Adc1ss1Id {} // IRQ 47
#[doc(hidden)]
pub struct Adc1ss2Id {} // IRQ 48
#[doc(hidden)]
pub struct Adc1ss3Id {} // IRQ 49
#[doc(hidden)]
pub struct Watchdog0Id {} // IRQ 18
#[doc(hidden)]
pub struct UdmaId {} // IRQ 44
#[doc(hidden)]
pub struct UdmaerrId {} // IRQ 45
#[doc(hidden)]
pub struct Pwm0FaultId {} // IRQ 9
#[doc(hidden)]
pub struct Pwm0Ch0Id {} // IRQ 10
#[doc(hidden)]
pub struct Pwm0Ch1Id {} // IRQ 11
#[doc(hidden)]
pub struct Pwm0Ch2Id {} // IRQ 12
#[doc(hidden)]
pub struct Pwm0Ch3Id {} // IRQ 43
#[doc(hidden)]
pub struct Timer0aId {} // IRQ 19
#[doc(hidden)]
pub struct Timer0bId {} // IRQ 20
#[doc(hidden)]
pub struct Timer1aId {} // IRQ 21
#[doc(hidden)]
pub struct Timer1bId {} // IRQ 22
#[doc(hidden)]
pub struct Timer2aId {} // IRQ 23
#[doc(hidden)]
pub struct Timer2bId {} // IRQ 24
#[doc(hidden)]
pub struct Timer3aId {} // IRQ 35
#[doc(hidden)]
pub struct Timer3bId {} // IRQ 36
#[doc(hidden)]
pub struct Timer4aId {} // IRQ 63
#[doc(hidden)]
pub struct Timer4bId {} // IRQ 64
#[doc(hidden)]
pub struct Timer5aId {} // IRQ 65
#[doc(hidden)]
pub struct Timer5bId {} // IRQ 66
#[doc(hidden)]
pub struct Timer6aId {} // IRQ 98
#[doc(hidden)]
pub struct Timer6bId {} // IRQ 99
#[doc(hidden)]
pub struct Timer7aId {} // IRQ 100
#[doc(hidden)]
pub struct Timer7bId {} // IRQ 101
#[doc(hidden)]
pub struct Uart0Id {} // IRQ 5
#[doc(hidden)]
pub struct Uart1Id {} // IRQ 6
#[doc(hidden)]
pub struct Uart2Id {} // IRQ 33
#[doc(hidden)]
pub struct Uart3Id {} // IRQ 56
#[doc(hidden)]
pub struct Uart4Id {} // IRQ 57
#[doc(hidden)]
pub struct Uart5Id {} // IRQ 58
#[doc(hidden)]
pub struct Uart6Id {} // IRQ 59
#[doc(hidden)]
pub struct Uart7Id {} // IRQ 60
#[doc(hidden)]
pub struct I2c0Id {} // IRQ 8
#[doc(hidden)]
pub struct I2c1Id {} // IRQ 37
#[doc(hidden)]
pub struct I2c2Id {} // IRQ 61
#[doc(hidden)]
pub struct I2c3Id {} // IRQ 62
#[doc(hidden)]
pub struct Ssi0Id {} // IRQ 7
#[doc(hidden)]
pub struct Ssi1Id {} // IRQ 34
#[doc(hidden)]
pub struct Ssi2Id {} // IRQ 54
#[doc(hidden)]
pub struct Ssi3Id {} // IRQ 55
#[doc(hidden)]
pub struct GpioaId {} // IRQ 0
#[doc(hidden)]
pub struct GpiobId {} // IRQ 1
#[doc(hidden)]
pub struct GpiocId {} // IRQ 2
#[doc(hidden)]
pub struct GpiodId {} // IRQ 3
#[doc(hidden)]
pub struct GpioeId {} // IRQ 4
#[doc(hidden)]
pub struct GpiofId {} // IRQ 30
#[doc(hidden)]
pub struct GpiogId {} // IRQ 31
#[doc(hidden)]
pub struct GpiohId {} // IRQ 32
#[doc(hidden)]
pub struct GpiojId {} // IRQ 54
#[doc(hidden)]
pub struct GpioaAhbId {} // IRQ 0
#[doc(hidden)]
pub struct GpiobAhbId {} // IRQ 1
#[doc(hidden)]
pub struct GpiocAhbId {} // IRQ 2
#[doc(hidden)]
pub struct GpiodAhbId {} // IRQ 3
#[doc(hidden)]
pub struct GpioeAhbId {} // IRQ 4
#[doc(hidden)]
pub struct GpiofAhbId {} // IRQ 30
#[doc(hidden)]
pub struct GpiogAhbId {} // IRQ 31
#[doc(hidden)]
pub struct GpiohAhbId {} // IRQ 32
#[doc(hidden)]
pub struct GpiojAhbId {} // IRQ 54
#[doc(hidden)]
pub struct GpiokId {} // IRQ 55
#[doc(hidden)]
pub struct GpiolId {} // IRQ 53
#[doc(hidden)]
pub struct GpiomId {} // IRQ 72
#[doc(hidden)]
pub struct GpionId {} // IRQ 73
#[doc(hidden)]
pub struct Gpiop0Id {} // IRQ 76
#[doc(hidden)]
pub struct Gpiop1Id {} // IRQ 77
#[doc(hidden)]
pub struct Gpiop2Id {} // IRQ 78
#[doc(hidden)]
pub struct Gpiop3Id {} // IRQ 79
#[doc(hidden)]
pub struct Gpiop4Id {} // IRQ 80
#[doc(hidden)]
pub struct Gpiop5Id {} // IRQ 81
#[doc(hidden)]
pub struct Gpiop6Id {} // IRQ 82
#[doc(hidden)]
pub struct Gpiop7Id {} // IRQ 83
#[doc(hidden)]
pub struct Gpioq0Id {} // IRQ 84
#[doc(hidden)]
pub struct Gpioq1Id {} // IRQ 85
#[doc(hidden)]
pub struct Gpioq2Id {} // IRQ 86
#[doc(hidden)]
pub struct Gpioq3Id {} // IRQ 87
#[doc(hidden)]
pub struct Gpioq4Id {} // IRQ 88
#[doc(hidden)]
pub struct Gpioq5Id {} // IRQ 89
#[doc(hidden)]
pub struct Gpioq6Id {} // IRQ 90
#[doc(hidden)]
pub struct Gpioq7Id {} // IRQ 91

pub fn set_handler(index: usize, handler: Option<Handler>) {
  unsafe { 
     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());
     R_INTERRUPT_HANDLERS[index] = handler
  };
}

use super::nvic::NVIC;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Irq<T>(usize, T);
impl<T> Irq<T> {
   pub fn index(&self) -> usize { self.0 }

   pub fn is_enabled(&self) -> bool { NVIC.iser((self.0 >> 5)).setena((self.0 & 0b11111)) != 0 }

   pub fn set_enabled(&self, value: bool) {
      if value {
         assert!(self.handler().is_some());
         NVIC.set_iser((self.0 >> 5), |r| r.set_setena((self.0 & 0b11111), 1));
      } else {
         NVIC.set_icer((self.0 >> 5), |r| r.set_clrena((self.0 & 0b11111), 1));
      }
   }

   pub fn is_pending(&self) -> bool {
       NVIC.ispr((self.0 >> 5)).setpend((self.0 & 0b11111)) != 0
   }

   pub fn set_pending(&self, value: bool) {
       if value {
           NVIC.set_ispr((self.0 >> 5), |r| r.set_setpend((self.0 & 0b11111), 1));
       } else {
           NVIC.set_icpr((self.0 >> 5), |r| r.set_clrpend((self.0 & 0b11111), 1));
       }
   }

   pub fn is_active(&self) -> bool {
       NVIC.iabr((self.0 >> 5)).active(self.0 & 0b11111) != 0
   }

   pub fn priority(&self) -> u8 {
       NVIC.ipr((self.0 >> 4)).pri(self.0 & 0b1111).into()
   }

   pub fn set_priority(&self, value: u8) {
       NVIC.with_ipr((self.0 >> 4), |r| r.set_pri(self.0 & 0b1111, value));
   }

   pub fn trigger_interrupt(&self) {
       NVIC.set_stir(|r| r.set_intid(self.0));
   }

   pub fn handler(&self) -> Option<Handler> { unsafe { R_INTERRUPT_HANDLERS[self.0] } }

   pub fn set_handler(&self, handler: Option<Handler>) {
      unsafe { assert!(R_INTERRUPT_HANDLERS[self.0].is_some() != handler.is_some()); };
      unsafe { R_INTERRUPT_HANDLERS[self.0] = handler };
   }
}

pub struct IrqHandle {}
pub struct IrqGuard<'a>(usize, PhantomData<&'a IrqHandle>);
impl<'a> IrqGuard<'a> {
   pub fn new(index: usize) -> Self {
      IrqGuard(index, PhantomData)
   }
}
impl<'a> Drop for IrqGuard<'a> {
   fn drop(&mut self) {
      set_handler(self.0, None)
   }
}


pub trait RegisterHandler {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a>;
}

pub trait HandleInterrupt {
   fn handle_interrupt(&self);
}

impl RegisterHandler for IrqAdc0ss0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(14, Some(wrapper::<F>));
       IrqGuard::new(14)
   }
}

impl RegisterHandler for IrqAdc0ss1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(15, Some(wrapper::<F>));
       IrqGuard::new(15)
   }
}

impl RegisterHandler for IrqAdc0ss2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(16, Some(wrapper::<F>));
       IrqGuard::new(16)
   }
}

impl RegisterHandler for IrqAdc0ss3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(17, Some(wrapper::<F>));
       IrqGuard::new(17)
   }
}

impl RegisterHandler for IrqAdc1ss0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(46, Some(wrapper::<F>));
       IrqGuard::new(46)
   }
}

impl RegisterHandler for IrqAdc1ss1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(47, Some(wrapper::<F>));
       IrqGuard::new(47)
   }
}

impl RegisterHandler for IrqAdc1ss2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(48, Some(wrapper::<F>));
       IrqGuard::new(48)
   }
}

impl RegisterHandler for IrqAdc1ss3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(49, Some(wrapper::<F>));
       IrqGuard::new(49)
   }
}

impl RegisterHandler for IrqWatchdog0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(18, Some(wrapper::<F>));
       IrqGuard::new(18)
   }
}

impl RegisterHandler for IrqUdma {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(44, Some(wrapper::<F>));
       IrqGuard::new(44)
   }
}

impl RegisterHandler for IrqUdmaerr {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(45, Some(wrapper::<F>));
       IrqGuard::new(45)
   }
}

impl RegisterHandler for IrqPwm0Fault {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(9, Some(wrapper::<F>));
       IrqGuard::new(9)
   }
}

impl RegisterHandler for IrqPwm0Ch0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(10, Some(wrapper::<F>));
       IrqGuard::new(10)
   }
}

impl RegisterHandler for IrqPwm0Ch1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(11, Some(wrapper::<F>));
       IrqGuard::new(11)
   }
}

impl RegisterHandler for IrqPwm0Ch2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(12, Some(wrapper::<F>));
       IrqGuard::new(12)
   }
}

impl RegisterHandler for IrqPwm0Ch3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(43, Some(wrapper::<F>));
       IrqGuard::new(43)
   }
}

impl RegisterHandler for IrqTimer0a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(19, Some(wrapper::<F>));
       IrqGuard::new(19)
   }
}

impl RegisterHandler for IrqTimer0b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(20, Some(wrapper::<F>));
       IrqGuard::new(20)
   }
}

impl RegisterHandler for IrqTimer1a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(21, Some(wrapper::<F>));
       IrqGuard::new(21)
   }
}

impl RegisterHandler for IrqTimer1b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(22, Some(wrapper::<F>));
       IrqGuard::new(22)
   }
}

impl RegisterHandler for IrqTimer2a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(23, Some(wrapper::<F>));
       IrqGuard::new(23)
   }
}

impl RegisterHandler for IrqTimer2b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(24, Some(wrapper::<F>));
       IrqGuard::new(24)
   }
}

impl RegisterHandler for IrqTimer3a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(35, Some(wrapper::<F>));
       IrqGuard::new(35)
   }
}

impl RegisterHandler for IrqTimer3b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(36, Some(wrapper::<F>));
       IrqGuard::new(36)
   }
}

impl RegisterHandler for IrqTimer4a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(63, Some(wrapper::<F>));
       IrqGuard::new(63)
   }
}

impl RegisterHandler for IrqTimer4b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(64, Some(wrapper::<F>));
       IrqGuard::new(64)
   }
}

impl RegisterHandler for IrqTimer5a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(65, Some(wrapper::<F>));
       IrqGuard::new(65)
   }
}

impl RegisterHandler for IrqTimer5b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(66, Some(wrapper::<F>));
       IrqGuard::new(66)
   }
}

impl RegisterHandler for IrqTimer6a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(98, Some(wrapper::<F>));
       IrqGuard::new(98)
   }
}

impl RegisterHandler for IrqTimer6b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(99, Some(wrapper::<F>));
       IrqGuard::new(99)
   }
}

impl RegisterHandler for IrqTimer7a {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(100, Some(wrapper::<F>));
       IrqGuard::new(100)
   }
}

impl RegisterHandler for IrqTimer7b {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(101, Some(wrapper::<F>));
       IrqGuard::new(101)
   }
}

impl RegisterHandler for IrqUart0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(5, Some(wrapper::<F>));
       IrqGuard::new(5)
   }
}

impl RegisterHandler for IrqUart1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(6, Some(wrapper::<F>));
       IrqGuard::new(6)
   }
}

impl RegisterHandler for IrqUart2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(33, Some(wrapper::<F>));
       IrqGuard::new(33)
   }
}

impl RegisterHandler for IrqUart3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(56, Some(wrapper::<F>));
       IrqGuard::new(56)
   }
}

impl RegisterHandler for IrqUart4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(57, Some(wrapper::<F>));
       IrqGuard::new(57)
   }
}

impl RegisterHandler for IrqUart5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(58, Some(wrapper::<F>));
       IrqGuard::new(58)
   }
}

impl RegisterHandler for IrqUart6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(59, Some(wrapper::<F>));
       IrqGuard::new(59)
   }
}

impl RegisterHandler for IrqUart7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(60, Some(wrapper::<F>));
       IrqGuard::new(60)
   }
}

impl RegisterHandler for IrqI2c0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(8, Some(wrapper::<F>));
       IrqGuard::new(8)
   }
}

impl RegisterHandler for IrqI2c1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(37, Some(wrapper::<F>));
       IrqGuard::new(37)
   }
}

impl RegisterHandler for IrqI2c2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(61, Some(wrapper::<F>));
       IrqGuard::new(61)
   }
}

impl RegisterHandler for IrqI2c3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(62, Some(wrapper::<F>));
       IrqGuard::new(62)
   }
}

impl RegisterHandler for IrqSsi0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(7, Some(wrapper::<F>));
       IrqGuard::new(7)
   }
}

impl RegisterHandler for IrqSsi1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(34, Some(wrapper::<F>));
       IrqGuard::new(34)
   }
}

impl RegisterHandler for IrqSsi2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(54, Some(wrapper::<F>));
       IrqGuard::new(54)
   }
}

impl RegisterHandler for IrqSsi3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(55, Some(wrapper::<F>));
       IrqGuard::new(55)
   }
}

impl RegisterHandler for IrqGpioa {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(0, Some(wrapper::<F>));
       IrqGuard::new(0)
   }
}

impl RegisterHandler for IrqGpiob {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(1, Some(wrapper::<F>));
       IrqGuard::new(1)
   }
}

impl RegisterHandler for IrqGpioc {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(2, Some(wrapper::<F>));
       IrqGuard::new(2)
   }
}

impl RegisterHandler for IrqGpiod {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(3, Some(wrapper::<F>));
       IrqGuard::new(3)
   }
}

impl RegisterHandler for IrqGpioe {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(4, Some(wrapper::<F>));
       IrqGuard::new(4)
   }
}

impl RegisterHandler for IrqGpiof {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(30, Some(wrapper::<F>));
       IrqGuard::new(30)
   }
}

impl RegisterHandler for IrqGpiog {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(31, Some(wrapper::<F>));
       IrqGuard::new(31)
   }
}

impl RegisterHandler for IrqGpioh {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(32, Some(wrapper::<F>));
       IrqGuard::new(32)
   }
}

impl RegisterHandler for IrqGpioj {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(54, Some(wrapper::<F>));
       IrqGuard::new(54)
   }
}

impl RegisterHandler for IrqGpioaAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(0, Some(wrapper::<F>));
       IrqGuard::new(0)
   }
}

impl RegisterHandler for IrqGpiobAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(1, Some(wrapper::<F>));
       IrqGuard::new(1)
   }
}

impl RegisterHandler for IrqGpiocAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(2, Some(wrapper::<F>));
       IrqGuard::new(2)
   }
}

impl RegisterHandler for IrqGpiodAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(3, Some(wrapper::<F>));
       IrqGuard::new(3)
   }
}

impl RegisterHandler for IrqGpioeAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(4, Some(wrapper::<F>));
       IrqGuard::new(4)
   }
}

impl RegisterHandler for IrqGpiofAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(30, Some(wrapper::<F>));
       IrqGuard::new(30)
   }
}

impl RegisterHandler for IrqGpiogAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(31, Some(wrapper::<F>));
       IrqGuard::new(31)
   }
}

impl RegisterHandler for IrqGpiohAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(32, Some(wrapper::<F>));
       IrqGuard::new(32)
   }
}

impl RegisterHandler for IrqGpiojAhb {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(54, Some(wrapper::<F>));
       IrqGuard::new(54)
   }
}

impl RegisterHandler for IrqGpiok {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(55, Some(wrapper::<F>));
       IrqGuard::new(55)
   }
}

impl RegisterHandler for IrqGpiol {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(53, Some(wrapper::<F>));
       IrqGuard::new(53)
   }
}

impl RegisterHandler for IrqGpiom {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(72, Some(wrapper::<F>));
       IrqGuard::new(72)
   }
}

impl RegisterHandler for IrqGpion {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(73, Some(wrapper::<F>));
       IrqGuard::new(73)
   }
}

impl RegisterHandler for IrqGpiop0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(76, Some(wrapper::<F>));
       IrqGuard::new(76)
   }
}

impl RegisterHandler for IrqGpiop1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(77, Some(wrapper::<F>));
       IrqGuard::new(77)
   }
}

impl RegisterHandler for IrqGpiop2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(78, Some(wrapper::<F>));
       IrqGuard::new(78)
   }
}

impl RegisterHandler for IrqGpiop3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(79, Some(wrapper::<F>));
       IrqGuard::new(79)
   }
}

impl RegisterHandler for IrqGpiop4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(80, Some(wrapper::<F>));
       IrqGuard::new(80)
   }
}

impl RegisterHandler for IrqGpiop5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(81, Some(wrapper::<F>));
       IrqGuard::new(81)
   }
}

impl RegisterHandler for IrqGpiop6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(82, Some(wrapper::<F>));
       IrqGuard::new(82)
   }
}

impl RegisterHandler for IrqGpiop7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(83, Some(wrapper::<F>));
       IrqGuard::new(83)
   }
}

impl RegisterHandler for IrqGpioq0 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(84, Some(wrapper::<F>));
       IrqGuard::new(84)
   }
}

impl RegisterHandler for IrqGpioq1 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(85, Some(wrapper::<F>));
       IrqGuard::new(85)
   }
}

impl RegisterHandler for IrqGpioq2 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(86, Some(wrapper::<F>));
       IrqGuard::new(86)
   }
}

impl RegisterHandler for IrqGpioq3 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(87, Some(wrapper::<F>));
       IrqGuard::new(87)
   }
}

impl RegisterHandler for IrqGpioq4 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(88, Some(wrapper::<F>));
       IrqGuard::new(88)
   }
}

impl RegisterHandler for IrqGpioq5 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(89, Some(wrapper::<F>));
       IrqGuard::new(89)
   }
}

impl RegisterHandler for IrqGpioq6 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(90, Some(wrapper::<F>));
       IrqGuard::new(90)
   }
}

impl RegisterHandler for IrqGpioq7 {
   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleInterrupt>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_interrupt() }
       }
       set_handler(91, Some(wrapper::<F>));
       IrqGuard::new(91)
   }
}

#[link_section = ".vector.interrupts"]
#[no_mangle]
pub static mut INTERRUPT_HANDLERS: [Option<Handler>; 114] = [
   None,                          // IRQ 0: No Description
   None,                          // IRQ 1: No Description
   None,                          // IRQ 2: No Description
   None,                          // IRQ 3: No Description
   None,                          // IRQ 4: No Description
   None,                          // IRQ 5: No Description
   None,                          // IRQ 6: No Description
   None,                          // IRQ 7: No Description
   None,                          // IRQ 8: No Description
   None,                          // IRQ 9: No Description
   None,
   None,
   None,
   None,
   None,                          // IRQ 14: No Description
   None,                          // IRQ 15: No Description
   None,                          // IRQ 16: No Description
   None,                          // IRQ 17: No Description
   None,                          // IRQ 18: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 28: No Description
   None,
   None,                          // IRQ 30: No Description
   None,                          // IRQ 31: No Description
   None,                          // IRQ 32: No Description
   None,                          // IRQ 33: No Description
   None,                          // IRQ 34: No Description
   None,
   None,
   None,                          // IRQ 37: No Description
   None,
   None,
   None,                          // IRQ 40: No Description
   None,
   None,
   None,
   None,                          // IRQ 44: No Description
   None,                          // IRQ 45: No Description
   None,                          // IRQ 46: No Description
   None,                          // IRQ 47: No Description
   None,                          // IRQ 48: No Description
   None,                          // IRQ 49: No Description
   None,
   None,
   None,
   None,                          // IRQ 53: No Description
   None,                          // IRQ 54: No Description
   None,                          // IRQ 55: No Description
   None,                          // IRQ 56: No Description
   None,                          // IRQ 57: No Description
   None,                          // IRQ 58: No Description
   None,                          // IRQ 59: No Description
   None,                          // IRQ 60: No Description
   None,                          // IRQ 61: No Description
   None,                          // IRQ 62: No Description
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,
   None,                          // IRQ 72: No Description
   None,                          // IRQ 73: No Description
   None,
   None,
   None,                          // IRQ 76: No Description
   None,                          // IRQ 77: No Description
   None,                          // IRQ 78: No Description
   None,                          // IRQ 79: No Description
   None,                          // IRQ 80: No Description
   None,                          // IRQ 81: No Description
   None,                          // IRQ 82: No Description
   None,                          // IRQ 83: No Description
   None,                          // IRQ 84: No Description
   None,                          // IRQ 85: No Description
   None,                          // IRQ 86: No Description
   None,                          // IRQ 87: No Description
   None,                          // IRQ 88: No Description
   None,                          // IRQ 89: No Description
   None,                          // IRQ 90: No Description
   None,                          // IRQ 91: No Description
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
   None,
   None,
   None,
   None,
   None,
   None,
];

#[link_section = ".bss.r_interrupts"]
#[no_mangle]
pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; 114] = [None; 114];

