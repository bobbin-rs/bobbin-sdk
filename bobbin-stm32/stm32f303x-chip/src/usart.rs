pub use stm32_common::chip::usart::*;

pub const USART1: Usart1 = Usart1 {};
pub const USART1_IMPL: UsartImpl = UsartImpl(0x40013800);
pub const USART1_IMPL_REF: &UsartImpl = &USART1_IMPL;

pub struct Usart1 {}
impl ::core::ops::Deref for Usart1 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART1_IMPL_REF }
}


pub const USART2: Usart2 = Usart2 {};
pub const USART2_IMPL: UsartImpl = UsartImpl(0x40004400);
pub const USART2_IMPL_REF: &UsartImpl = &USART2_IMPL;

pub struct Usart2 {}
impl ::core::ops::Deref for Usart2 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART2_IMPL_REF }
}


pub const USART3: Usart3 = Usart3 {};
pub const USART3_IMPL: UsartImpl = UsartImpl(0x40004800);
pub const USART3_IMPL_REF: &UsartImpl = &USART3_IMPL;

pub struct Usart3 {}
impl ::core::ops::Deref for Usart3 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART3_IMPL_REF }
}


pub const UART4: Uart4 = Uart4 {};
pub const UART4_IMPL: UsartImpl = UsartImpl(0x40004c00);
pub const UART4_IMPL_REF: &UsartImpl = &UART4_IMPL;

pub struct Uart4 {}
impl ::core::ops::Deref for Uart4 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART4_IMPL_REF }
}


pub const UART5: Uart5 = Uart5 {};
pub const UART5_IMPL: UsartImpl = UsartImpl(0x40005000);
pub const UART5_IMPL_REF: &UsartImpl = &UART5_IMPL;

pub struct Uart5 {}
impl ::core::ops::Deref for Uart5 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART5_IMPL_REF }
}



