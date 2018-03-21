#[allow(unused_imports)] pub use ::bobbin_common::*;

signal_type!(AIN, SigAin);
signal_type!(WO_0, SigWo0);
signal_type!(WO_1, SigWo1);
signal_type!(WO_2, SigWo2);
signal_type!(WO_3, SigWo3);
signal_type!(WO_4, SigWo4);
signal_type!(WO_5, SigWo5);
signal_type!(WO_6, SigWo6);
signal_type!(WO_7, SigWo7);
signal_type!(WO, SigWo);
signal_type!(PAD_0, SigPad0);
signal_type!(PAD_1, SigPad1);
signal_type!(PAD_2, SigPad2);
signal_type!(PAD_3, SigPad3);

// DMAC

// ADC
channel_signal!(super::adc::AdcCh0, SigAin);
channel_signal!(super::adc::AdcCh1, SigAin);
channel_signal!(super::adc::AdcCh2, SigAin);
channel_signal!(super::adc::AdcCh3, SigAin);
channel_signal!(super::adc::AdcCh4, SigAin);
channel_signal!(super::adc::AdcCh5, SigAin);
channel_signal!(super::adc::AdcCh6, SigAin);
channel_signal!(super::adc::AdcCh7, SigAin);
channel_signal!(super::adc::AdcCh8, SigAin);
channel_signal!(super::adc::AdcCh9, SigAin);
channel_signal!(super::adc::AdcCh10, SigAin);
channel_signal!(super::adc::AdcCh11, SigAin);
channel_signal!(super::adc::AdcCh12, SigAin);
channel_signal!(super::adc::AdcCh13, SigAin);
channel_signal!(super::adc::AdcCh14, SigAin);
channel_signal!(super::adc::AdcCh15, SigAin);
channel_signal!(super::adc::AdcCh16, SigAin);
channel_signal!(super::adc::AdcCh17, SigAin);
channel_signal!(super::adc::AdcCh18, SigAin);
channel_signal!(super::adc::AdcCh19, SigAin);

// TCC
periph_signal!(super::tcc::Tcc0, SigWo0);
periph_signal!(super::tcc::Tcc0, SigWo1);
periph_signal!(super::tcc::Tcc0, SigWo2);
periph_signal!(super::tcc::Tcc0, SigWo3);
periph_signal!(super::tcc::Tcc0, SigWo4);
periph_signal!(super::tcc::Tcc0, SigWo5);
periph_signal!(super::tcc::Tcc0, SigWo6);
periph_signal!(super::tcc::Tcc0, SigWo7);
periph_signal!(super::tcc::Tcc1, SigWo0);
periph_signal!(super::tcc::Tcc1, SigWo1);
periph_signal!(super::tcc::Tcc1, SigWo2);
periph_signal!(super::tcc::Tcc1, SigWo3);
periph_signal!(super::tcc::Tcc2, SigWo0);
periph_signal!(super::tcc::Tcc2, SigWo1);

// TC
channel_signal!(super::tc::Tc3Ch0, SigWo);
channel_signal!(super::tc::Tc3Ch1, SigWo);
channel_signal!(super::tc::Tc4Ch0, SigWo);
channel_signal!(super::tc::Tc4Ch1, SigWo);
channel_signal!(super::tc::Tc5Ch0, SigWo);
channel_signal!(super::tc::Tc5Ch1, SigWo);

// PORT

// SERCOM
periph_signal!(super::sercom::Sercom0, SigPad0);
periph_signal!(super::sercom::Sercom0, SigPad1);
periph_signal!(super::sercom::Sercom0, SigPad2);
periph_signal!(super::sercom::Sercom0, SigPad3);
periph_signal!(super::sercom::Sercom1, SigPad0);
periph_signal!(super::sercom::Sercom1, SigPad1);
periph_signal!(super::sercom::Sercom1, SigPad2);
periph_signal!(super::sercom::Sercom1, SigPad3);
periph_signal!(super::sercom::Sercom2, SigPad0);
periph_signal!(super::sercom::Sercom2, SigPad1);
periph_signal!(super::sercom::Sercom2, SigPad2);
periph_signal!(super::sercom::Sercom2, SigPad3);
periph_signal!(super::sercom::Sercom3, SigPad0);
periph_signal!(super::sercom::Sercom3, SigPad1);
periph_signal!(super::sercom::Sercom3, SigPad2);
periph_signal!(super::sercom::Sercom3, SigPad3);
periph_signal!(super::sercom::Sercom4, SigPad0);
periph_signal!(super::sercom::Sercom4, SigPad1);
periph_signal!(super::sercom::Sercom4, SigPad2);
periph_signal!(super::sercom::Sercom4, SigPad3);
periph_signal!(super::sercom::Sercom5, SigPad0);
periph_signal!(super::sercom::Sercom5, SigPad1);
periph_signal!(super::sercom::Sercom5, SigPad2);
periph_signal!(super::sercom::Sercom5, SigPad3);





















































