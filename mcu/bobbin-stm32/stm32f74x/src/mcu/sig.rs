#[allow(unused_imports)] pub use ::bobbin_common::*;

signal_type!(DCMI_HSYNC, SigDcmiHsync);
signal_type!(DCMI_VSYNC, SigDcmiVsync);
signal_type!(DCMI_PIXCLK, SigDcmiPixclk);
signal_type!(DCMI_D0, SigDcmiD0);
signal_type!(DCMI_D1, SigDcmiD1);
signal_type!(DCMI_D2, SigDcmiD2);
signal_type!(DCMI_D3, SigDcmiD3);
signal_type!(DCMI_D4, SigDcmiD4);
signal_type!(DCMI_D5, SigDcmiD5);
signal_type!(DCMI_D6, SigDcmiD6);
signal_type!(DCMI_D7, SigDcmiD7);
signal_type!(DCMI_D8, SigDcmiD8);
signal_type!(DCMI_D9, SigDcmiD9);
signal_type!(DCMI_D10, SigDcmiD10);
signal_type!(DCMI_D11, SigDcmiD11);
signal_type!(DCMI_D12, SigDcmiD12);
signal_type!(DCMI_D13, SigDcmiD13);
signal_type!(USB_ID, SigUsbId);
signal_type!(USB_DM, SigUsbDm);
signal_type!(USB_DP, SigUsbDp);
signal_type!(TIM, SigTim);
signal_type!(ADC, SigAdc);
signal_type!(SPI_NSS, SigSpiNss);
signal_type!(SPI_MISO, SigSpiMiso);
signal_type!(SPI_MOSI, SigSpiMosi);
signal_type!(SPI_SCK, SigSpiSck);
signal_type!(I2C_SCL, SigI2cScl);
signal_type!(I2C_SDA, SigI2cSda);
signal_type!(I2C_SMBAL, SigI2cSmbal);
signal_type!(CAN_TX, SigCanTx);
signal_type!(CAN_RX, SigCanRx);
signal_type!(USART_TX, SigUsartTx);
signal_type!(USART_RX, SigUsartRx);
signal_type!(USART_CTS, SigUsartCts);
signal_type!(USART_RTS, SigUsartRts);
signal_type!(USART_CK, SigUsartCk);

// DCMI
periph_signal!(super::dcmi::Dcmi, SigDcmiHsync);
periph_signal!(super::dcmi::Dcmi, SigDcmiVsync);
periph_signal!(super::dcmi::Dcmi, SigDcmiPixclk);
periph_signal!(super::dcmi::Dcmi, SigDcmiD0);
periph_signal!(super::dcmi::Dcmi, SigDcmiD1);
periph_signal!(super::dcmi::Dcmi, SigDcmiD2);
periph_signal!(super::dcmi::Dcmi, SigDcmiD3);
periph_signal!(super::dcmi::Dcmi, SigDcmiD4);
periph_signal!(super::dcmi::Dcmi, SigDcmiD5);
periph_signal!(super::dcmi::Dcmi, SigDcmiD6);
periph_signal!(super::dcmi::Dcmi, SigDcmiD7);
periph_signal!(super::dcmi::Dcmi, SigDcmiD8);
periph_signal!(super::dcmi::Dcmi, SigDcmiD9);
periph_signal!(super::dcmi::Dcmi, SigDcmiD10);
periph_signal!(super::dcmi::Dcmi, SigDcmiD11);
periph_signal!(super::dcmi::Dcmi, SigDcmiD12);
periph_signal!(super::dcmi::Dcmi, SigDcmiD13);

// USB_FS_GLOBAL

// USB_FS_HOST
periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbId);
periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbDm);
periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbDp);

// USB_FS_DEVICE
periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbId);
periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbDm);
periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbDp);

// USB_FS_PWRCLK

// IWDG

// WWDG

// CRC

// EXTI

// TIM_BAS

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
channel_signal!(super::tim_gen::Tim5Ch1, SigTim);
channel_signal!(super::tim_gen::Tim5Ch2, SigTim);
channel_signal!(super::tim_gen::Tim5Ch3, SigTim);
channel_signal!(super::tim_gen::Tim5Ch4, SigTim);
channel_signal!(super::tim_gen::Tim9Ch1, SigTim);
channel_signal!(super::tim_gen::Tim9Ch2, SigTim);
channel_signal!(super::tim_gen::Tim10Ch1, SigTim);
channel_signal!(super::tim_gen::Tim11Ch1, SigTim);
channel_signal!(super::tim_gen::Tim12Ch1, SigTim);
channel_signal!(super::tim_gen::Tim12Ch2, SigTim);
channel_signal!(super::tim_gen::Tim13Ch1, SigTim);
channel_signal!(super::tim_gen::Tim14Ch1, SigTim);

// TIM_ADV
channel_signal!(super::tim_adv::Tim1Ch1, SigTim);
channel_signal!(super::tim_adv::Tim1Ch2, SigTim);
channel_signal!(super::tim_adv::Tim1Ch3, SigTim);
channel_signal!(super::tim_adv::Tim1Ch4, SigTim);
channel_signal!(super::tim_adv::Tim8Ch1, SigTim);
channel_signal!(super::tim_adv::Tim8Ch2, SigTim);
channel_signal!(super::tim_adv::Tim8Ch3, SigTim);
channel_signal!(super::tim_adv::Tim8Ch4, SigTim);

// LPTIM

// ADC
channel_signal!(super::adc::Adc1Ch0, SigAdc);
channel_signal!(super::adc::Adc1Ch1, SigAdc);
channel_signal!(super::adc::Adc1Ch2, SigAdc);
channel_signal!(super::adc::Adc1Ch3, SigAdc);
channel_signal!(super::adc::Adc1Ch4, SigAdc);
channel_signal!(super::adc::Adc1Ch5, SigAdc);
channel_signal!(super::adc::Adc1Ch6, SigAdc);
channel_signal!(super::adc::Adc1Ch7, SigAdc);
channel_signal!(super::adc::Adc1Ch8, SigAdc);
channel_signal!(super::adc::Adc1Ch9, SigAdc);
channel_signal!(super::adc::Adc1Ch10, SigAdc);
channel_signal!(super::adc::Adc1Ch11, SigAdc);
channel_signal!(super::adc::Adc1Ch12, SigAdc);
channel_signal!(super::adc::Adc1Ch13, SigAdc);
channel_signal!(super::adc::Adc1Ch14, SigAdc);
channel_signal!(super::adc::Adc1Ch15, SigAdc);
channel_signal!(super::adc::Adc2Ch0, SigAdc);
channel_signal!(super::adc::Adc2Ch1, SigAdc);
channel_signal!(super::adc::Adc2Ch2, SigAdc);
channel_signal!(super::adc::Adc2Ch3, SigAdc);
channel_signal!(super::adc::Adc2Ch4, SigAdc);
channel_signal!(super::adc::Adc2Ch5, SigAdc);
channel_signal!(super::adc::Adc2Ch6, SigAdc);
channel_signal!(super::adc::Adc2Ch7, SigAdc);
channel_signal!(super::adc::Adc2Ch8, SigAdc);
channel_signal!(super::adc::Adc2Ch9, SigAdc);
channel_signal!(super::adc::Adc2Ch10, SigAdc);
channel_signal!(super::adc::Adc2Ch11, SigAdc);
channel_signal!(super::adc::Adc2Ch12, SigAdc);
channel_signal!(super::adc::Adc2Ch13, SigAdc);
channel_signal!(super::adc::Adc2Ch14, SigAdc);
channel_signal!(super::adc::Adc2Ch15, SigAdc);
channel_signal!(super::adc::Adc3Ch0, SigAdc);
channel_signal!(super::adc::Adc3Ch1, SigAdc);
channel_signal!(super::adc::Adc3Ch2, SigAdc);
channel_signal!(super::adc::Adc3Ch3, SigAdc);
channel_signal!(super::adc::Adc3Ch4, SigAdc);
channel_signal!(super::adc::Adc3Ch5, SigAdc);
channel_signal!(super::adc::Adc3Ch6, SigAdc);
channel_signal!(super::adc::Adc3Ch7, SigAdc);
channel_signal!(super::adc::Adc3Ch8, SigAdc);
channel_signal!(super::adc::Adc3Ch9, SigAdc);
channel_signal!(super::adc::Adc3Ch10, SigAdc);
channel_signal!(super::adc::Adc3Ch11, SigAdc);
channel_signal!(super::adc::Adc3Ch12, SigAdc);
channel_signal!(super::adc::Adc3Ch13, SigAdc);
channel_signal!(super::adc::Adc3Ch14, SigAdc);
channel_signal!(super::adc::Adc3Ch15, SigAdc);

// SPI
periph_signal!(super::spi::Spi1, SigSpiNss);
periph_signal!(super::spi::Spi1, SigSpiMiso);
periph_signal!(super::spi::Spi1, SigSpiMosi);
periph_signal!(super::spi::Spi1, SigSpiSck);
periph_signal!(super::spi::Spi2, SigSpiNss);
periph_signal!(super::spi::Spi2, SigSpiMiso);
periph_signal!(super::spi::Spi2, SigSpiMosi);
periph_signal!(super::spi::Spi2, SigSpiSck);
periph_signal!(super::spi::Spi3, SigSpiNss);
periph_signal!(super::spi::Spi3, SigSpiMiso);
periph_signal!(super::spi::Spi3, SigSpiMosi);
periph_signal!(super::spi::Spi3, SigSpiSck);
periph_signal!(super::spi::Spi4, SigSpiNss);
periph_signal!(super::spi::Spi4, SigSpiMiso);
periph_signal!(super::spi::Spi4, SigSpiMosi);
periph_signal!(super::spi::Spi4, SigSpiSck);
periph_signal!(super::spi::Spi5, SigSpiNss);
periph_signal!(super::spi::Spi5, SigSpiMiso);
periph_signal!(super::spi::Spi5, SigSpiMosi);
periph_signal!(super::spi::Spi5, SigSpiSck);
periph_signal!(super::spi::Spi6, SigSpiNss);
periph_signal!(super::spi::Spi6, SigSpiMiso);
periph_signal!(super::spi::Spi6, SigSpiMosi);
periph_signal!(super::spi::Spi6, SigSpiSck);

// I2C
periph_signal!(super::i2c::I2c1, SigI2cScl);
periph_signal!(super::i2c::I2c1, SigI2cSda);
periph_signal!(super::i2c::I2c1, SigI2cSmbal);
periph_signal!(super::i2c::I2c2, SigI2cScl);
periph_signal!(super::i2c::I2c2, SigI2cSda);
periph_signal!(super::i2c::I2c2, SigI2cSmbal);
periph_signal!(super::i2c::I2c3, SigI2cScl);
periph_signal!(super::i2c::I2c3, SigI2cSda);
periph_signal!(super::i2c::I2c3, SigI2cSmbal);
periph_signal!(super::i2c::I2c4, SigI2cScl);
periph_signal!(super::i2c::I2c4, SigI2cSda);
periph_signal!(super::i2c::I2c4, SigI2cSmbal);

// CAN
periph_signal!(super::can::Can1, SigCanTx);
periph_signal!(super::can::Can1, SigCanRx);
periph_signal!(super::can::Can2, SigCanTx);
periph_signal!(super::can::Can2, SigCanRx);

// GPIO

// USART
periph_signal!(super::usart::Usart1, SigUsartTx);
periph_signal!(super::usart::Usart1, SigUsartRx);
periph_signal!(super::usart::Usart1, SigUsartCts);
periph_signal!(super::usart::Usart1, SigUsartRts);
periph_signal!(super::usart::Usart1, SigUsartCk);
periph_signal!(super::usart::Usart2, SigUsartTx);
periph_signal!(super::usart::Usart2, SigUsartRx);
periph_signal!(super::usart::Usart2, SigUsartCts);
periph_signal!(super::usart::Usart2, SigUsartRts);
periph_signal!(super::usart::Usart2, SigUsartCk);
periph_signal!(super::usart::Usart3, SigUsartTx);
periph_signal!(super::usart::Usart3, SigUsartRx);
periph_signal!(super::usart::Usart3, SigUsartCts);
periph_signal!(super::usart::Usart3, SigUsartRts);
periph_signal!(super::usart::Usart3, SigUsartCk);
periph_signal!(super::usart::Uart4, SigUsartTx);
periph_signal!(super::usart::Uart4, SigUsartRx);
periph_signal!(super::usart::Uart4, SigUsartCts);
periph_signal!(super::usart::Uart4, SigUsartRts);
periph_signal!(super::usart::Uart4, SigUsartCk);
periph_signal!(super::usart::Uart5, SigUsartTx);
periph_signal!(super::usart::Uart5, SigUsartRx);
periph_signal!(super::usart::Uart5, SigUsartCts);
periph_signal!(super::usart::Uart5, SigUsartRts);
periph_signal!(super::usart::Uart5, SigUsartCk);
periph_signal!(super::usart::Usart6, SigUsartTx);
periph_signal!(super::usart::Usart6, SigUsartRx);
periph_signal!(super::usart::Usart6, SigUsartCts);
periph_signal!(super::usart::Usart6, SigUsartRts);
periph_signal!(super::usart::Usart6, SigUsartCk);
periph_signal!(super::usart::Uart7, SigUsartTx);
periph_signal!(super::usart::Uart7, SigUsartRx);
periph_signal!(super::usart::Uart7, SigUsartCts);
periph_signal!(super::usart::Uart7, SigUsartRts);
periph_signal!(super::usart::Uart7, SigUsartCk);
periph_signal!(super::usart::Uart8, SigUsartTx);
periph_signal!(super::usart::Uart8, SigUsartRx);
periph_signal!(super::usart::Uart8, SigUsartCts);
periph_signal!(super::usart::Uart8, SigUsartRts);
periph_signal!(super::usart::Uart8, SigUsartCk);

// DMA









































































































































































