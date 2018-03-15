#[allow(unused_imports)] pub use ::bobbin_common::*;
signal_type!(TIM, SigTim);
signal_type!(TX, SigTx);
signal_type!(RX, SigRx);
signal_type!(CTS, SigCts);
signal_type!(RTS, SigRts);
signal_type!(CK, SigCk);

// IWDG

// WWDG

// CRC

// EXTI

// TIM_GEN
channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
channel_signal!(super::tim_gen::Tim3Ch1, SigTim);
channel_signal!(super::tim_gen::Tim3Ch2, SigTim);
channel_signal!(super::tim_gen::Tim3Ch3, SigTim);
channel_signal!(super::tim_gen::Tim3Ch4, SigTim);
channel_signal!(super::tim_gen::Tim4Ch1, SigTim);
channel_signal!(super::tim_gen::Tim4Ch2, SigTim);
channel_signal!(super::tim_gen::Tim4Ch3, SigTim);
channel_signal!(super::tim_gen::Tim4Ch4, SigTim);

// TIM_ADV

// ADC

// SPI

// I2C

// GPIO

// USART
periph_signal!(super::usart::Usart1, SigTx);
periph_signal!(super::usart::Usart1, SigRx);
periph_signal!(super::usart::Usart1, SigCts);
periph_signal!(super::usart::Usart1, SigRts);
periph_signal!(super::usart::Usart1, SigCk);
periph_signal!(super::usart::Usart2, SigTx);
periph_signal!(super::usart::Usart2, SigRx);
periph_signal!(super::usart::Usart2, SigCts);
periph_signal!(super::usart::Usart2, SigRts);
periph_signal!(super::usart::Usart2, SigCk);
periph_signal!(super::usart::Usart3, SigTx);
periph_signal!(super::usart::Usart3, SigRx);
periph_signal!(super::usart::Usart3, SigCts);
periph_signal!(super::usart::Usart3, SigRts);
periph_signal!(super::usart::Usart3, SigCk);

// DMA

















































































































