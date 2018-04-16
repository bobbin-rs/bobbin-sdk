::bobbin_mcu::signal_type!(DAC, SigDac);
::bobbin_mcu::signal_type!(DCMI_HSYNC, SigDcmiHsync);
::bobbin_mcu::signal_type!(DCMI_VSYNC, SigDcmiVsync);
::bobbin_mcu::signal_type!(DCMI_PIXCLK, SigDcmiPixclk);
::bobbin_mcu::signal_type!(DCMI_D0, SigDcmiD0);
::bobbin_mcu::signal_type!(DCMI_D1, SigDcmiD1);
::bobbin_mcu::signal_type!(DCMI_D2, SigDcmiD2);
::bobbin_mcu::signal_type!(DCMI_D3, SigDcmiD3);
::bobbin_mcu::signal_type!(DCMI_D4, SigDcmiD4);
::bobbin_mcu::signal_type!(DCMI_D5, SigDcmiD5);
::bobbin_mcu::signal_type!(DCMI_D6, SigDcmiD6);
::bobbin_mcu::signal_type!(DCMI_D7, SigDcmiD7);
::bobbin_mcu::signal_type!(DCMI_D8, SigDcmiD8);
::bobbin_mcu::signal_type!(DCMI_D9, SigDcmiD9);
::bobbin_mcu::signal_type!(DCMI_D10, SigDcmiD10);
::bobbin_mcu::signal_type!(DCMI_D11, SigDcmiD11);
::bobbin_mcu::signal_type!(DCMI_D12, SigDcmiD12);
::bobbin_mcu::signal_type!(DCMI_D13, SigDcmiD13);
::bobbin_mcu::signal_type!(USB_ID, SigUsbId);
::bobbin_mcu::signal_type!(USB_DM, SigUsbDm);
::bobbin_mcu::signal_type!(USB_DP, SigUsbDp);
::bobbin_mcu::signal_type!(TIM, SigTim);
::bobbin_mcu::signal_type!(ADC, SigAdc);
::bobbin_mcu::signal_type!(SPI_NSS, SigSpiNss);
::bobbin_mcu::signal_type!(SPI_MISO, SigSpiMiso);
::bobbin_mcu::signal_type!(SPI_MOSI, SigSpiMosi);
::bobbin_mcu::signal_type!(SPI_SCK, SigSpiSck);
::bobbin_mcu::signal_type!(I2C_SCL, SigI2cScl);
::bobbin_mcu::signal_type!(I2C_SDA, SigI2cSda);
::bobbin_mcu::signal_type!(I2C_SMBAL, SigI2cSmbal);
::bobbin_mcu::signal_type!(CAN_TX, SigCanTx);
::bobbin_mcu::signal_type!(CAN_RX, SigCanRx);
::bobbin_mcu::signal_type!(USART_TX, SigUsartTx);
::bobbin_mcu::signal_type!(USART_RX, SigUsartRx);
::bobbin_mcu::signal_type!(USART_CTS, SigUsartCts);
::bobbin_mcu::signal_type!(USART_RTS, SigUsartRts);
::bobbin_mcu::signal_type!(USART_CK, SigUsartCk);

// DAC
::bobbin_mcu::channel_signal!(super::dac::DacCh1, SigDac);
::bobbin_mcu::channel_signal!(super::dac::DacCh2, SigDac);

// DCMI
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiHsync);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiVsync);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiPixclk);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD0);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD1);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD2);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD3);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD4);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD5);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD6);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD7);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD8);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD9);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD10);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD11);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD12);
::bobbin_mcu::periph_signal!(super::dcmi::Dcmi, SigDcmiD13);

// USB_FS_GLOBAL

// USB_FS_HOST
::bobbin_mcu::periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbId);
::bobbin_mcu::periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbDm);
::bobbin_mcu::periph_signal!(super::usb_fs_host::UsbFsHost, SigUsbDp);

// USB_FS_DEVICE
::bobbin_mcu::periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbId);
::bobbin_mcu::periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbDm);
::bobbin_mcu::periph_signal!(super::usb_fs_device::UsbFsDevice, SigUsbDp);

// USB_FS_PWRCLK

// IWDG

// WWDG

// CRC

// EXTI

// TIM_BAS

// TIM_GEN
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim2Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim3Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim4Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim5Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim5Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim5Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim5Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim9Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim9Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim10Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim11Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim12Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim12Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim13Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_gen::Tim14Ch1, SigTim);

// TIM_ADV
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim1Ch4, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch1, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch2, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch3, SigTim);
::bobbin_mcu::channel_signal!(super::tim_adv::Tim8Ch4, SigTim);

// LPTIM

// ADC
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch0, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc1Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch0, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc2Ch15, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch0, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch1, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch2, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch3, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch4, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch5, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch6, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch7, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch8, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch9, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch10, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch11, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch12, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch13, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch14, SigAdc);
::bobbin_mcu::channel_signal!(super::adc::Adc3Ch15, SigAdc);

// SPI
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi1, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi2, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi3, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi4, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi4, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi4, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi4, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi5, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi5, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi5, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi5, SigSpiSck);
::bobbin_mcu::periph_signal!(super::spi::Spi6, SigSpiNss);
::bobbin_mcu::periph_signal!(super::spi::Spi6, SigSpiMiso);
::bobbin_mcu::periph_signal!(super::spi::Spi6, SigSpiMosi);
::bobbin_mcu::periph_signal!(super::spi::Spi6, SigSpiSck);

// I2C
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c1, SigI2cSmbal);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c2, SigI2cSmbal);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c3, SigI2cSmbal);
::bobbin_mcu::periph_signal!(super::i2c::I2c4, SigI2cScl);
::bobbin_mcu::periph_signal!(super::i2c::I2c4, SigI2cSda);
::bobbin_mcu::periph_signal!(super::i2c::I2c4, SigI2cSmbal);

// CAN
::bobbin_mcu::periph_signal!(super::can::Can1, SigCanTx);
::bobbin_mcu::periph_signal!(super::can::Can1, SigCanRx);
::bobbin_mcu::periph_signal!(super::can::Can2, SigCanTx);
::bobbin_mcu::periph_signal!(super::can::Can2, SigCanRx);

// GPIO

// USART
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Usart1, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Usart2, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Usart3, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Uart4, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Uart5, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Usart6, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Usart6, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Usart6, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Usart6, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Usart6, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Uart7, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Uart7, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Uart7, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Uart7, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Uart7, SigUsartCk);
::bobbin_mcu::periph_signal!(super::usart::Uart8, SigUsartTx);
::bobbin_mcu::periph_signal!(super::usart::Uart8, SigUsartRx);
::bobbin_mcu::periph_signal!(super::usart::Uart8, SigUsartCts);
::bobbin_mcu::periph_signal!(super::usart::Uart8, SigUsartRts);
::bobbin_mcu::periph_signal!(super::usart::Uart8, SigUsartCk);

// DMA









































































































































































