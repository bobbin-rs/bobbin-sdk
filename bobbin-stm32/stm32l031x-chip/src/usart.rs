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


