pub use stm32_common::chip::usart::*;

pub const USART2: Usart2 = Usart2 {};
pub const USART2_IMPL: UsartImpl = UsartImpl(0x40004400);
pub const USART2_IMPL_REF: &UsartImpl = &USART2_IMPL;

pub struct Usart2 {}
impl ::core::ops::Deref for Usart2 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART2_IMPL_REF }
}

impl super::sig::Signal<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Ck> for Usart2 {}


