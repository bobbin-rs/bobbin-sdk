pub use stm32_common::chip::usart_f24::*;

pub const USART1: Usart1 = Usart1 {};
pub const USART1_IMPL: UsartImpl = UsartImpl(0x40011000);
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


pub const USART6: Usart6 = Usart6 {};
pub const USART6_IMPL: UsartImpl = UsartImpl(0x40011400);
pub const USART6_IMPL_REF: &UsartImpl = &USART6_IMPL;

pub struct Usart6 {}
impl ::core::ops::Deref for Usart6 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART6_IMPL_REF }
}


pub const UART7: Uart7 = Uart7 {};
pub const UART7_IMPL: UsartImpl = UsartImpl(0x40007800);
pub const UART7_IMPL_REF: &UsartImpl = &UART7_IMPL;

pub struct Uart7 {}
impl ::core::ops::Deref for Uart7 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART7_IMPL_REF }
}


pub const UART8: Uart8 = Uart8 {};
pub const UART8_IMPL: UsartImpl = UsartImpl(0x40007c00);
pub const UART8_IMPL_REF: &UsartImpl = &UART8_IMPL;

pub struct Uart8 {}
impl ::core::ops::Deref for Uart8 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART8_IMPL_REF }
}



