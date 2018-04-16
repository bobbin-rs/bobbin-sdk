::bobbin_mcu::signal_type!(AIN, SigAin);
::bobbin_mcu::signal_type!(WO_0, SigWo0);
::bobbin_mcu::signal_type!(WO_1, SigWo1);
::bobbin_mcu::signal_type!(WO_2, SigWo2);
::bobbin_mcu::signal_type!(WO_3, SigWo3);
::bobbin_mcu::signal_type!(WO_4, SigWo4);
::bobbin_mcu::signal_type!(WO_5, SigWo5);
::bobbin_mcu::signal_type!(WO_6, SigWo6);
::bobbin_mcu::signal_type!(WO_7, SigWo7);
::bobbin_mcu::signal_type!(WO, SigWo);
::bobbin_mcu::signal_type!(PAD_0, SigPad0);
::bobbin_mcu::signal_type!(PAD_1, SigPad1);
::bobbin_mcu::signal_type!(PAD_2, SigPad2);
::bobbin_mcu::signal_type!(PAD_3, SigPad3);

// DMAC

// ADC
::bobbin_mcu::channel_signal!(super::adc::AdcCh0, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh1, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh2, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh3, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh4, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh5, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh6, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh7, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh8, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh9, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh10, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh11, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh12, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh13, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh14, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh15, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh16, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh17, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh18, SigAin);
::bobbin_mcu::channel_signal!(super::adc::AdcCh19, SigAin);

// DAC

// TCC
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo0);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo1);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo2);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo3);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo4);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo5);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo6);
::bobbin_mcu::periph_signal!(super::tcc::Tcc0, SigWo7);
::bobbin_mcu::periph_signal!(super::tcc::Tcc1, SigWo0);
::bobbin_mcu::periph_signal!(super::tcc::Tcc1, SigWo1);
::bobbin_mcu::periph_signal!(super::tcc::Tcc1, SigWo2);
::bobbin_mcu::periph_signal!(super::tcc::Tcc1, SigWo3);
::bobbin_mcu::periph_signal!(super::tcc::Tcc2, SigWo0);
::bobbin_mcu::periph_signal!(super::tcc::Tcc2, SigWo1);

// TC
::bobbin_mcu::channel_signal!(super::tc::Tc3Ch0, SigWo);
::bobbin_mcu::channel_signal!(super::tc::Tc3Ch1, SigWo);
::bobbin_mcu::channel_signal!(super::tc::Tc4Ch0, SigWo);
::bobbin_mcu::channel_signal!(super::tc::Tc4Ch1, SigWo);
::bobbin_mcu::channel_signal!(super::tc::Tc5Ch0, SigWo);
::bobbin_mcu::channel_signal!(super::tc::Tc5Ch1, SigWo);

// PORT

// SERCOM
::bobbin_mcu::periph_signal!(super::sercom::Sercom0, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom0, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom0, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom0, SigPad3);
::bobbin_mcu::periph_signal!(super::sercom::Sercom1, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom1, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom1, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom1, SigPad3);
::bobbin_mcu::periph_signal!(super::sercom::Sercom2, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom2, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom2, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom2, SigPad3);
::bobbin_mcu::periph_signal!(super::sercom::Sercom3, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom3, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom3, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom3, SigPad3);
::bobbin_mcu::periph_signal!(super::sercom::Sercom4, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom4, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom4, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom4, SigPad3);
::bobbin_mcu::periph_signal!(super::sercom::Sercom5, SigPad0);
::bobbin_mcu::periph_signal!(super::sercom::Sercom5, SigPad1);
::bobbin_mcu::periph_signal!(super::sercom::Sercom5, SigPad2);
::bobbin_mcu::periph_signal!(super::sercom::Sercom5, SigPad3);





















































