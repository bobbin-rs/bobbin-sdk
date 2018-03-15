#[allow(unused_imports)] pub use ::bobbin_common::*;
signal_type!(FTM, SigFtm);
signal_type!(TX, SigTx);
signal_type!(RX, SigRx);

// CRC

// DMAMUX

// EDMA

// FTM
channel_signal!(super::ftm::Ftm0Ch0, SigFtm);
channel_signal!(super::ftm::Ftm0Ch1, SigFtm);
channel_signal!(super::ftm::Ftm0Ch2, SigFtm);
channel_signal!(super::ftm::Ftm0Ch3, SigFtm);
channel_signal!(super::ftm::Ftm0Ch4, SigFtm);
channel_signal!(super::ftm::Ftm0Ch5, SigFtm);
channel_signal!(super::ftm::Ftm0Ch6, SigFtm);
channel_signal!(super::ftm::Ftm0Ch7, SigFtm);
channel_signal!(super::ftm::Ftm1Ch0, SigFtm);
channel_signal!(super::ftm::Ftm1Ch1, SigFtm);
channel_signal!(super::ftm::Ftm1Ch2, SigFtm);
channel_signal!(super::ftm::Ftm1Ch3, SigFtm);
channel_signal!(super::ftm::Ftm1Ch4, SigFtm);
channel_signal!(super::ftm::Ftm1Ch5, SigFtm);
channel_signal!(super::ftm::Ftm1Ch6, SigFtm);
channel_signal!(super::ftm::Ftm1Ch7, SigFtm);
channel_signal!(super::ftm::Ftm2Ch0, SigFtm);
channel_signal!(super::ftm::Ftm2Ch1, SigFtm);
channel_signal!(super::ftm::Ftm2Ch2, SigFtm);
channel_signal!(super::ftm::Ftm2Ch3, SigFtm);
channel_signal!(super::ftm::Ftm2Ch4, SigFtm);
channel_signal!(super::ftm::Ftm2Ch5, SigFtm);
channel_signal!(super::ftm::Ftm2Ch6, SigFtm);
channel_signal!(super::ftm::Ftm2Ch7, SigFtm);

// PIT

// SPI

// I2C

// UART
periph_signal!(super::uart::Uart0, SigTx);
periph_signal!(super::uart::Uart0, SigRx);
periph_signal!(super::uart::Uart1, SigTx);
periph_signal!(super::uart::Uart1, SigRx);
periph_signal!(super::uart::Uart2, SigTx);
periph_signal!(super::uart::Uart2, SigRx);
periph_signal!(super::uart::Uart3, SigTx);
periph_signal!(super::uart::Uart3, SigRx);
periph_signal!(super::uart::Uart4, SigTx);
periph_signal!(super::uart::Uart4, SigRx);
periph_signal!(super::uart::Uart5, SigTx);
periph_signal!(super::uart::Uart5, SigRx);

// GPIO

// PORT













































































































































































































