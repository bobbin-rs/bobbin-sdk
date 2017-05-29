pub use kinetis_common::chip::port::*;

pub const PORTA: Port = Port(0x40049000);
pub const PORTB: Port = Port(0x4004a000);
pub const PORTC: Port = Port(0x4004b000);
pub const PORTD: Port = Port(0x4004c000);
pub const PORTE: Port = Port(0x4004d000);

pub trait Pin {
   fn port(&self) -> Port;
   fn index(&self) -> usize;
}

pub trait AfAdc0Se0 {
   fn af_adc0_se0(&self) -> usize;
}

pub trait AfCmp0In0 {
   fn af_cmp0_in0(&self) -> usize;
}

pub trait AfPta0 {
   fn af_pta0(&self) -> usize;
}

pub trait AfFtm2Ch1 {
   fn af_ftm2_ch1(&self) -> usize;
}

pub trait AfLpi2c0Scls {
   fn af_lpi2c0_scls(&self) -> usize;
}

pub trait AfFxioD2 {
   fn af_fxio_d2(&self) -> usize;
}

pub trait AfFtm2QdPha {
   fn af_ftm2_qd_pha(&self) -> usize;
}

pub trait AfLpuart0Cts {
   fn af_lpuart0_cts(&self) -> usize;
}

pub trait AfTrgmuxOut3 {
   fn af_trgmux_out3(&self) -> usize;
}

pub trait AfAdc0Se1 {
   fn af_adc0_se1(&self) -> usize;
}

pub trait AfCmp0In1 {
   fn af_cmp0_in1(&self) -> usize;
}

pub trait AfPta1 {
   fn af_pta1(&self) -> usize;
}

pub trait AfFtm1Ch1 {
   fn af_ftm1_ch1(&self) -> usize;
}

pub trait AfLpi2c0Sdas {
   fn af_lpi2c0_sdas(&self) -> usize;
}

pub trait AfFxioD3 {
   fn af_fxio_d3(&self) -> usize;
}

pub trait AfFtm1QdPha {
   fn af_ftm1_qd_pha(&self) -> usize;
}

pub trait AfLpuart0Rts {
   fn af_lpuart0_rts(&self) -> usize;
}

pub trait AfTrgmuxOut0 {
   fn af_trgmux_out0(&self) -> usize;
}

pub trait AfAdc1Se0 {
   fn af_adc1_se0(&self) -> usize;
}

pub trait AfPta2 {
   fn af_pta2(&self) -> usize;
}

pub trait AfFtm3Ch0 {
   fn af_ftm3_ch0(&self) -> usize;
}

pub trait AfLpi2c0Sda {
   fn af_lpi2c0_sda(&self) -> usize;
}

pub trait AfEwmOutB {
   fn af_ewm_out_b(&self) -> usize;
}

pub trait AfFxioD4 {
   fn af_fxio_d4(&self) -> usize;
}

pub trait AfLpuart0Rx {
   fn af_lpuart0_rx(&self) -> usize;
}

pub trait AfAdc1Se1 {
   fn af_adc1_se1(&self) -> usize;
}

pub trait AfPta3 {
   fn af_pta3(&self) -> usize;
}

pub trait AfFtm3Ch1 {
   fn af_ftm3_ch1(&self) -> usize;
}

pub trait AfLpi2c0Scl {
   fn af_lpi2c0_scl(&self) -> usize;
}

pub trait AfEwmIn {
   fn af_ewm_in(&self) -> usize;
}

pub trait AfFxioD5 {
   fn af_fxio_d5(&self) -> usize;
}

pub trait AfLpuart0Tx {
   fn af_lpuart0_tx(&self) -> usize;
}

pub trait AfPta4 {
   fn af_pta4(&self) -> usize;
}

pub trait AfCmp0Out {
   fn af_cmp0_out(&self) -> usize;
}

pub trait AfJtagTms {
   fn af_jtag_tms(&self) -> usize;
}

pub trait AfSwdDio {
   fn af_swd_dio(&self) -> usize;
}

pub trait AfPta5 {
   fn af_pta5(&self) -> usize;
}

pub trait AfTclk1 {
   fn af_tclk1(&self) -> usize;
}

pub trait AfResetB {
   fn af_reset_b(&self) -> usize;
}

pub trait AfAdc0Se2 {
   fn af_adc0_se2(&self) -> usize;
}

pub trait AfPta6 {
   fn af_pta6(&self) -> usize;
}

pub trait AfFtm0Flt1 {
   fn af_ftm0_flt1(&self) -> usize;
}

pub trait AfLpspi1Pcs1 {
   fn af_lpspi1_pcs1(&self) -> usize;
}

pub trait AfLpuart1Cts {
   fn af_lpuart1_cts(&self) -> usize;
}

pub trait AfAdc0Se3 {
   fn af_adc0_se3(&self) -> usize;
}

pub trait AfPta7 {
   fn af_pta7(&self) -> usize;
}

pub trait AfFtm0Flt2 {
   fn af_ftm0_flt2(&self) -> usize;
}

pub trait AfRtcClkin {
   fn af_rtc_clkin(&self) -> usize;
}

pub trait AfLpuart1Rts {
   fn af_lpuart1_rts(&self) -> usize;
}

pub trait AfPta8 {
   fn af_pta8(&self) -> usize;
}

pub trait AfLpuart2Rx {
   fn af_lpuart2_rx(&self) -> usize;
}

pub trait AfLpspi2Sout {
   fn af_lpspi2_sout(&self) -> usize;
}

pub trait AfFxioD6 {
   fn af_fxio_d6(&self) -> usize;
}

pub trait AfFtm3Flt3 {
   fn af_ftm3_flt3(&self) -> usize;
}

pub trait AfPta9 {
   fn af_pta9(&self) -> usize;
}

pub trait AfLpuart2Tx {
   fn af_lpuart2_tx(&self) -> usize;
}

pub trait AfLpspi2Pcs0 {
   fn af_lpspi2_pcs0(&self) -> usize;
}

pub trait AfFxioD7 {
   fn af_fxio_d7(&self) -> usize;
}

pub trait AfFtm3Flt2 {
   fn af_ftm3_flt2(&self) -> usize;
}

pub trait AfFtm1Flt3 {
   fn af_ftm1_flt3(&self) -> usize;
}

pub trait AfPta10 {
   fn af_pta10(&self) -> usize;
}

pub trait AfFtm1Ch4 {
   fn af_ftm1_ch4(&self) -> usize;
}

pub trait AfFxioD0 {
   fn af_fxio_d0(&self) -> usize;
}

pub trait AfJtagTdo {
   fn af_jtag_tdo(&self) -> usize;
}

pub trait AfNoetmTraceSwo {
   fn af_noetm_trace_swo(&self) -> usize;
}

pub trait AfPta11 {
   fn af_pta11(&self) -> usize;
}

pub trait AfFtm1Ch5 {
   fn af_ftm1_ch5(&self) -> usize;
}

pub trait AfFxioD1 {
   fn af_fxio_d1(&self) -> usize;
}

pub trait AfCmp0Rrt {
   fn af_cmp0_rrt(&self) -> usize;
}

pub trait AfPta12 {
   fn af_pta12(&self) -> usize;
}

pub trait AfFtm1Ch6 {
   fn af_ftm1_ch6(&self) -> usize;
}

pub trait AfCan1Rx {
   fn af_can1_rx(&self) -> usize;
}

pub trait AfFtm2QdPhb {
   fn af_ftm2_qd_phb(&self) -> usize;
}

pub trait AfPta13 {
   fn af_pta13(&self) -> usize;
}

pub trait AfFtm1Ch7 {
   fn af_ftm1_ch7(&self) -> usize;
}

pub trait AfCan1Tx {
   fn af_can1_tx(&self) -> usize;
}

pub trait AfPta14 {
   fn af_pta14(&self) -> usize;
}

pub trait AfFtm0Flt0 {
   fn af_ftm0_flt0(&self) -> usize;
}

pub trait AfFtm3Flt1 {
   fn af_ftm3_flt1(&self) -> usize;
}

pub trait AfFtm1Flt0 {
   fn af_ftm1_flt0(&self) -> usize;
}

pub trait AfAdc1Se12 {
   fn af_adc1_se12(&self) -> usize;
}

pub trait AfPta15 {
   fn af_pta15(&self) -> usize;
}

pub trait AfFtm1Ch2 {
   fn af_ftm1_ch2(&self) -> usize;
}

pub trait AfLpspi0Pcs3 {
   fn af_lpspi0_pcs3(&self) -> usize;
}

pub trait AfLpspi2Pcs3 {
   fn af_lpspi2_pcs3(&self) -> usize;
}

pub trait AfAdc1Se13 {
   fn af_adc1_se13(&self) -> usize;
}

pub trait AfPta16 {
   fn af_pta16(&self) -> usize;
}

pub trait AfFtm1Ch3 {
   fn af_ftm1_ch3(&self) -> usize;
}

pub trait AfLpspi1Pcs2 {
   fn af_lpspi1_pcs2(&self) -> usize;
}

pub trait AfPta17 {
   fn af_pta17(&self) -> usize;
}

pub trait AfFtm0Ch6 {
   fn af_ftm0_ch6(&self) -> usize;
}

pub trait AfFtm3Flt0 {
   fn af_ftm3_flt0(&self) -> usize;
}

pub trait AfAdc0Se4 {
   fn af_adc0_se4(&self) -> usize;
}

pub trait AfAdc1Se14 {
   fn af_adc1_se14(&self) -> usize;
}

pub trait AfPtb0 {
   fn af_ptb0(&self) -> usize;
}

pub trait AfLpspi0Pcs0 {
   fn af_lpspi0_pcs0(&self) -> usize;
}

pub trait AfLptmr0Alt3 {
   fn af_lptmr0_alt3(&self) -> usize;
}

pub trait AfCan0Rx {
   fn af_can0_rx(&self) -> usize;
}

pub trait AfAdc0Se5 {
   fn af_adc0_se5(&self) -> usize;
}

pub trait AfAdc1Se15 {
   fn af_adc1_se15(&self) -> usize;
}

pub trait AfPtb1 {
   fn af_ptb1(&self) -> usize;
}

pub trait AfLpspi0Sout {
   fn af_lpspi0_sout(&self) -> usize;
}

pub trait AfTclk0 {
   fn af_tclk0(&self) -> usize;
}

pub trait AfCan0Tx {
   fn af_can0_tx(&self) -> usize;
}

pub trait AfAdc0Se6 {
   fn af_adc0_se6(&self) -> usize;
}

pub trait AfPtb2 {
   fn af_ptb2(&self) -> usize;
}

pub trait AfFtm1Ch0 {
   fn af_ftm1_ch0(&self) -> usize;
}

pub trait AfLpspi0Sck {
   fn af_lpspi0_sck(&self) -> usize;
}

pub trait AfFtm1QdPhb {
   fn af_ftm1_qd_phb(&self) -> usize;
}

pub trait AfTrgmuxIn3 {
   fn af_trgmux_in3(&self) -> usize;
}

pub trait AfAdc0Se7 {
   fn af_adc0_se7(&self) -> usize;
}

pub trait AfPtb3 {
   fn af_ptb3(&self) -> usize;
}

pub trait AfLpspi0Sin {
   fn af_lpspi0_sin(&self) -> usize;
}

pub trait AfTrgmuxIn2 {
   fn af_trgmux_in2(&self) -> usize;
}

pub trait AfPtb4 {
   fn af_ptb4(&self) -> usize;
}

pub trait AfFtm0Ch4 {
   fn af_ftm0_ch4(&self) -> usize;
}

pub trait AfTrgmuxIn1 {
   fn af_trgmux_in1(&self) -> usize;
}

pub trait AfPtb5 {
   fn af_ptb5(&self) -> usize;
}

pub trait AfFtm0Ch5 {
   fn af_ftm0_ch5(&self) -> usize;
}

pub trait AfLpspi0Pcs1 {
   fn af_lpspi0_pcs1(&self) -> usize;
}

pub trait AfClkout {
   fn af_clkout(&self) -> usize;
}

pub trait AfTrgmuxIn0 {
   fn af_trgmux_in0(&self) -> usize;
}

pub trait AfXtal {
   fn af_xtal(&self) -> usize;
}

pub trait AfPtb6 {
   fn af_ptb6(&self) -> usize;
}

pub trait AfExtal {
   fn af_extal(&self) -> usize;
}

pub trait AfPtb7 {
   fn af_ptb7(&self) -> usize;
}

pub trait AfPtb8 {
   fn af_ptb8(&self) -> usize;
}

pub trait AfPtb9 {
   fn af_ptb9(&self) -> usize;
}

pub trait AfPtb10 {
   fn af_ptb10(&self) -> usize;
}

pub trait AfFtm3Ch2 {
   fn af_ftm3_ch2(&self) -> usize;
}

pub trait AfPtb11 {
   fn af_ptb11(&self) -> usize;
}

pub trait AfFtm3Ch3 {
   fn af_ftm3_ch3(&self) -> usize;
}

pub trait AfLpi2c0Hreq {
   fn af_lpi2c0_hreq(&self) -> usize;
}

pub trait AfAdc1Se7 {
   fn af_adc1_se7(&self) -> usize;
}

pub trait AfPtb12 {
   fn af_ptb12(&self) -> usize;
}

pub trait AfFtm0Ch0 {
   fn af_ftm0_ch0(&self) -> usize;
}

pub trait AfCan2Rx {
   fn af_can2_rx(&self) -> usize;
}

pub trait AfAdc1Se8 {
   fn af_adc1_se8(&self) -> usize;
}

pub trait AfAdc0Se8 {
   fn af_adc0_se8(&self) -> usize;
}

pub trait AfPtb13 {
   fn af_ptb13(&self) -> usize;
}

pub trait AfFtm0Ch1 {
   fn af_ftm0_ch1(&self) -> usize;
}

pub trait AfCan2Tx {
   fn af_can2_tx(&self) -> usize;
}

pub trait AfAdc1Se9 {
   fn af_adc1_se9(&self) -> usize;
}

pub trait AfAdc0Se9 {
   fn af_adc0_se9(&self) -> usize;
}

pub trait AfPtb14 {
   fn af_ptb14(&self) -> usize;
}

pub trait AfFtm0Ch2 {
   fn af_ftm0_ch2(&self) -> usize;
}

pub trait AfLpspi1Sck {
   fn af_lpspi1_sck(&self) -> usize;
}

pub trait AfPtb15 {
   fn af_ptb15(&self) -> usize;
}

pub trait AfFtm0Ch3 {
   fn af_ftm0_ch3(&self) -> usize;
}

pub trait AfLpspi1Sin {
   fn af_lpspi1_sin(&self) -> usize;
}

pub trait AfPtb16 {
   fn af_ptb16(&self) -> usize;
}

pub trait AfLpspi1Sout {
   fn af_lpspi1_sout(&self) -> usize;
}

pub trait AfPtb17 {
   fn af_ptb17(&self) -> usize;
}

pub trait AfLpspi1Pcs3 {
   fn af_lpspi1_pcs3(&self) -> usize;
}

pub trait AfPtc0 {
   fn af_ptc0(&self) -> usize;
}

pub trait AfLpspi2Sin {
   fn af_lpspi2_sin(&self) -> usize;
}

pub trait AfPtc1 {
   fn af_ptc1(&self) -> usize;
}

pub trait AfAdc0Se10 {
   fn af_adc0_se10(&self) -> usize;
}

pub trait AfCmp0In5 {
   fn af_cmp0_in5(&self) -> usize;
}

pub trait AfPtc2 {
   fn af_ptc2(&self) -> usize;
}

pub trait AfAdc0Se11 {
   fn af_adc0_se11(&self) -> usize;
}

pub trait AfCmp0In4 {
   fn af_cmp0_in4(&self) -> usize;
}

pub trait AfPtc3 {
   fn af_ptc3(&self) -> usize;
}

pub trait AfCmp0In2 {
   fn af_cmp0_in2(&self) -> usize;
}

pub trait AfPtc4 {
   fn af_ptc4(&self) -> usize;
}

pub trait AfRtcClkout {
   fn af_rtc_clkout(&self) -> usize;
}

pub trait AfJtagTclk {
   fn af_jtag_tclk(&self) -> usize;
}

pub trait AfSwdClk {
   fn af_swd_clk(&self) -> usize;
}

pub trait AfPtc5 {
   fn af_ptc5(&self) -> usize;
}

pub trait AfFtm2Ch0 {
   fn af_ftm2_ch0(&self) -> usize;
}

pub trait AfJtagTdi {
   fn af_jtag_tdi(&self) -> usize;
}

pub trait AfAdc1Se4 {
   fn af_adc1_se4(&self) -> usize;
}

pub trait AfPtc6 {
   fn af_ptc6(&self) -> usize;
}

pub trait AfLpuart1Rx {
   fn af_lpuart1_rx(&self) -> usize;
}

pub trait AfAdc1Se5 {
   fn af_adc1_se5(&self) -> usize;
}

pub trait AfPtc7 {
   fn af_ptc7(&self) -> usize;
}

pub trait AfLpuart1Tx {
   fn af_lpuart1_tx(&self) -> usize;
}

pub trait AfPtc8 {
   fn af_ptc8(&self) -> usize;
}

pub trait AfPtc9 {
   fn af_ptc9(&self) -> usize;
}

pub trait AfFtm1Flt1 {
   fn af_ftm1_flt1(&self) -> usize;
}

pub trait AfPtc10 {
   fn af_ptc10(&self) -> usize;
}

pub trait AfFtm3Ch4 {
   fn af_ftm3_ch4(&self) -> usize;
}

pub trait AfTrgmuxIn11 {
   fn af_trgmux_in11(&self) -> usize;
}

pub trait AfPtc11 {
   fn af_ptc11(&self) -> usize;
}

pub trait AfFtm3Ch5 {
   fn af_ftm3_ch5(&self) -> usize;
}

pub trait AfTrgmuxIn10 {
   fn af_trgmux_in10(&self) -> usize;
}

pub trait AfPtc12 {
   fn af_ptc12(&self) -> usize;
}

pub trait AfFtm3Ch6 {
   fn af_ftm3_ch6(&self) -> usize;
}

pub trait AfFtm2Ch6 {
   fn af_ftm2_ch6(&self) -> usize;
}

pub trait AfLpuart2Cts {
   fn af_lpuart2_cts(&self) -> usize;
}

pub trait AfPtc13 {
   fn af_ptc13(&self) -> usize;
}

pub trait AfFtm3Ch7 {
   fn af_ftm3_ch7(&self) -> usize;
}

pub trait AfFtm2Ch7 {
   fn af_ftm2_ch7(&self) -> usize;
}

pub trait AfLpuart2Rts {
   fn af_lpuart2_rts(&self) -> usize;
}

pub trait AfAdc0Se12 {
   fn af_adc0_se12(&self) -> usize;
}

pub trait AfPtc14 {
   fn af_ptc14(&self) -> usize;
}

pub trait AfTrgmuxIn9 {
   fn af_trgmux_in9(&self) -> usize;
}

pub trait AfAdc0Se13 {
   fn af_adc0_se13(&self) -> usize;
}

pub trait AfPtc15 {
   fn af_ptc15(&self) -> usize;
}

pub trait AfLpspi2Sck {
   fn af_lpspi2_sck(&self) -> usize;
}

pub trait AfTrgmuxIn8 {
   fn af_trgmux_in8(&self) -> usize;
}

pub trait AfAdc0Se14 {
   fn af_adc0_se14(&self) -> usize;
}

pub trait AfPtc16 {
   fn af_ptc16(&self) -> usize;
}

pub trait AfFtm1Flt2 {
   fn af_ftm1_flt2(&self) -> usize;
}

pub trait AfAdc0Se15 {
   fn af_adc0_se15(&self) -> usize;
}

pub trait AfPtc17 {
   fn af_ptc17(&self) -> usize;
}

pub trait AfPtd0 {
   fn af_ptd0(&self) -> usize;
}

pub trait AfTrgmuxOut1 {
   fn af_trgmux_out1(&self) -> usize;
}

pub trait AfPtd1 {
   fn af_ptd1(&self) -> usize;
}

pub trait AfTrgmuxOut2 {
   fn af_trgmux_out2(&self) -> usize;
}

pub trait AfAdc1Se2 {
   fn af_adc1_se2(&self) -> usize;
}

pub trait AfPtd2 {
   fn af_ptd2(&self) -> usize;
}

pub trait AfTrgmuxIn5 {
   fn af_trgmux_in5(&self) -> usize;
}

pub trait AfAdc1Se3 {
   fn af_adc1_se3(&self) -> usize;
}

pub trait AfPtd3 {
   fn af_ptd3(&self) -> usize;
}

pub trait AfLpspi1Pcs0 {
   fn af_lpspi1_pcs0(&self) -> usize;
}

pub trait AfTrgmuxIn4 {
   fn af_trgmux_in4(&self) -> usize;
}

pub trait AfNmiB {
   fn af_nmi_b(&self) -> usize;
}

pub trait AfAdc1Se6 {
   fn af_adc1_se6(&self) -> usize;
}

pub trait AfPtd4 {
   fn af_ptd4(&self) -> usize;
}

pub trait AfFtm0Flt3 {
   fn af_ftm0_flt3(&self) -> usize;
}

pub trait AfPtd5 {
   fn af_ptd5(&self) -> usize;
}

pub trait AfFtm2Ch3 {
   fn af_ftm2_ch3(&self) -> usize;
}

pub trait AfLptmr0Alt2 {
   fn af_lptmr0_alt2(&self) -> usize;
}

pub trait AfFtm2Flt1 {
   fn af_ftm2_flt1(&self) -> usize;
}

pub trait AfTrgmuxIn7 {
   fn af_trgmux_in7(&self) -> usize;
}

pub trait AfCmp0In7 {
   fn af_cmp0_in7(&self) -> usize;
}

pub trait AfPtd6 {
   fn af_ptd6(&self) -> usize;
}

pub trait AfFtm2Flt2 {
   fn af_ftm2_flt2(&self) -> usize;
}

pub trait AfCmp0In6 {
   fn af_cmp0_in6(&self) -> usize;
}

pub trait AfPtd7 {
   fn af_ptd7(&self) -> usize;
}

pub trait AfFtm2Flt3 {
   fn af_ftm2_flt3(&self) -> usize;
}

pub trait AfPtd8 {
   fn af_ptd8(&self) -> usize;
}

pub trait AfPtd9 {
   fn af_ptd9(&self) -> usize;
}

pub trait AfPtd10 {
   fn af_ptd10(&self) -> usize;
}

pub trait AfPtd11 {
   fn af_ptd11(&self) -> usize;
}

pub trait AfPtd12 {
   fn af_ptd12(&self) -> usize;
}

pub trait AfFtm2Ch2 {
   fn af_ftm2_ch2(&self) -> usize;
}

pub trait AfPtd13 {
   fn af_ptd13(&self) -> usize;
}

pub trait AfFtm2Ch4 {
   fn af_ftm2_ch4(&self) -> usize;
}

pub trait AfPtd14 {
   fn af_ptd14(&self) -> usize;
}

pub trait AfFtm2Ch5 {
   fn af_ftm2_ch5(&self) -> usize;
}

pub trait AfPtd15 {
   fn af_ptd15(&self) -> usize;
}

pub trait AfPtd16 {
   fn af_ptd16(&self) -> usize;
}

pub trait AfPtd17 {
   fn af_ptd17(&self) -> usize;
}

pub trait AfPte0 {
   fn af_pte0(&self) -> usize;
}

pub trait AfPte1 {
   fn af_pte1(&self) -> usize;
}

pub trait AfAdc1Se10 {
   fn af_adc1_se10(&self) -> usize;
}

pub trait AfPte2 {
   fn af_pte2(&self) -> usize;
}

pub trait AfPte3 {
   fn af_pte3(&self) -> usize;
}

pub trait AfFtm2Flt0 {
   fn af_ftm2_flt0(&self) -> usize;
}

pub trait AfTrgmuxIn6 {
   fn af_trgmux_in6(&self) -> usize;
}

pub trait AfPte4 {
   fn af_pte4(&self) -> usize;
}

pub trait AfPte5 {
   fn af_pte5(&self) -> usize;
}

pub trait AfTclk2 {
   fn af_tclk2(&self) -> usize;
}

pub trait AfAdc1Se11 {
   fn af_adc1_se11(&self) -> usize;
}

pub trait AfPte6 {
   fn af_pte6(&self) -> usize;
}

pub trait AfLpspi0Pcs2 {
   fn af_lpspi0_pcs2(&self) -> usize;
}

pub trait AfPte7 {
   fn af_pte7(&self) -> usize;
}

pub trait AfFtm0Ch7 {
   fn af_ftm0_ch7(&self) -> usize;
}

pub trait AfCmp0In3 {
   fn af_cmp0_in3(&self) -> usize;
}

pub trait AfPte8 {
   fn af_pte8(&self) -> usize;
}

pub trait AfPte9 {
   fn af_pte9(&self) -> usize;
}

pub trait AfPte10 {
   fn af_pte10(&self) -> usize;
}

pub trait AfLpspi2Pcs1 {
   fn af_lpspi2_pcs1(&self) -> usize;
}

pub trait AfTrgmuxOut4 {
   fn af_trgmux_out4(&self) -> usize;
}

pub trait AfPte11 {
   fn af_pte11(&self) -> usize;
}

pub trait AfLptmr0Alt1 {
   fn af_lptmr0_alt1(&self) -> usize;
}

pub trait AfTrgmuxOut5 {
   fn af_trgmux_out5(&self) -> usize;
}

pub trait AfPte12 {
   fn af_pte12(&self) -> usize;
}

pub trait AfPte13 {
   fn af_pte13(&self) -> usize;
}

pub trait AfLpspi2Pcs2 {
   fn af_lpspi2_pcs2(&self) -> usize;
}

pub trait AfPte14 {
   fn af_pte14(&self) -> usize;
}

pub trait AfPte15 {
   fn af_pte15(&self) -> usize;
}

pub trait AfTrgmuxOut6 {
   fn af_trgmux_out6(&self) -> usize;
}

pub trait AfPte16 {
   fn af_pte16(&self) -> usize;
}

pub trait AfTrgmuxOut7 {
   fn af_trgmux_out7(&self) -> usize;
}

pub const PTA0: Pta0 = Pta0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta0 {}

impl Pin for Pta0 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se0 for Pta0 {
   fn af_adc0_se0(&self) -> usize { 0 }
}

impl AfCmp0In0 for Pta0 {
   fn af_cmp0_in0(&self) -> usize { 0 }
}

impl AfPta0 for Pta0 {
   fn af_pta0(&self) -> usize { 1 }
}

impl AfFtm2Ch1 for Pta0 {
   fn af_ftm2_ch1(&self) -> usize { 2 }
}

impl AfLpi2c0Scls for Pta0 {
   fn af_lpi2c0_scls(&self) -> usize { 3 }
}

impl AfFxioD2 for Pta0 {
   fn af_fxio_d2(&self) -> usize { 4 }
}

impl AfFtm2QdPha for Pta0 {
   fn af_ftm2_qd_pha(&self) -> usize { 5 }
}

impl AfLpuart0Cts for Pta0 {
   fn af_lpuart0_cts(&self) -> usize { 6 }
}

impl AfTrgmuxOut3 for Pta0 {
   fn af_trgmux_out3(&self) -> usize { 7 }
}

pub const PTA1: Pta1 = Pta1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta1 {}

impl Pin for Pta1 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se1 for Pta1 {
   fn af_adc0_se1(&self) -> usize { 0 }
}

impl AfCmp0In1 for Pta1 {
   fn af_cmp0_in1(&self) -> usize { 0 }
}

impl AfPta1 for Pta1 {
   fn af_pta1(&self) -> usize { 1 }
}

impl AfFtm1Ch1 for Pta1 {
   fn af_ftm1_ch1(&self) -> usize { 2 }
}

impl AfLpi2c0Sdas for Pta1 {
   fn af_lpi2c0_sdas(&self) -> usize { 3 }
}

impl AfFxioD3 for Pta1 {
   fn af_fxio_d3(&self) -> usize { 4 }
}

impl AfFtm1QdPha for Pta1 {
   fn af_ftm1_qd_pha(&self) -> usize { 5 }
}

impl AfLpuart0Rts for Pta1 {
   fn af_lpuart0_rts(&self) -> usize { 6 }
}

impl AfTrgmuxOut0 for Pta1 {
   fn af_trgmux_out0(&self) -> usize { 7 }
}

pub const PTA2: Pta2 = Pta2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta2 {}

impl Pin for Pta2 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 2 }
}

impl AfAdc1Se0 for Pta2 {
   fn af_adc1_se0(&self) -> usize { 0 }
}

impl AfPta2 for Pta2 {
   fn af_pta2(&self) -> usize { 1 }
}

impl AfFtm3Ch0 for Pta2 {
   fn af_ftm3_ch0(&self) -> usize { 2 }
}

impl AfLpi2c0Sda for Pta2 {
   fn af_lpi2c0_sda(&self) -> usize { 3 }
}

impl AfEwmOutB for Pta2 {
   fn af_ewm_out_b(&self) -> usize { 4 }
}

impl AfFxioD4 for Pta2 {
   fn af_fxio_d4(&self) -> usize { 5 }
}

impl AfLpuart0Rx for Pta2 {
   fn af_lpuart0_rx(&self) -> usize { 6 }
}

pub const PTA3: Pta3 = Pta3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta3 {}

impl Pin for Pta3 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 3 }
}

impl AfAdc1Se1 for Pta3 {
   fn af_adc1_se1(&self) -> usize { 0 }
}

impl AfPta3 for Pta3 {
   fn af_pta3(&self) -> usize { 1 }
}

impl AfFtm3Ch1 for Pta3 {
   fn af_ftm3_ch1(&self) -> usize { 2 }
}

impl AfLpi2c0Scl for Pta3 {
   fn af_lpi2c0_scl(&self) -> usize { 3 }
}

impl AfEwmIn for Pta3 {
   fn af_ewm_in(&self) -> usize { 4 }
}

impl AfFxioD5 for Pta3 {
   fn af_fxio_d5(&self) -> usize { 5 }
}

impl AfLpuart0Tx for Pta3 {
   fn af_lpuart0_tx(&self) -> usize { 6 }
}

pub const PTA4: Pta4 = Pta4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta4 {}

impl Pin for Pta4 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 4 }
}

impl AfPta4 for Pta4 {
   fn af_pta4(&self) -> usize { 1 }
}

impl AfCmp0Out for Pta4 {
   fn af_cmp0_out(&self) -> usize { 4 }
}

impl AfEwmOutB for Pta4 {
   fn af_ewm_out_b(&self) -> usize { 5 }
}

impl AfJtagTms for Pta4 {
   fn af_jtag_tms(&self) -> usize { 7 }
}

impl AfSwdDio for Pta4 {
   fn af_swd_dio(&self) -> usize { 7 }
}

pub const PTA5: Pta5 = Pta5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta5 {}

impl Pin for Pta5 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 5 }
}

impl AfPta5 for Pta5 {
   fn af_pta5(&self) -> usize { 1 }
}

impl AfTclk1 for Pta5 {
   fn af_tclk1(&self) -> usize { 3 }
}

impl AfResetB for Pta5 {
   fn af_reset_b(&self) -> usize { 7 }
}

pub const PTA6: Pta6 = Pta6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta6 {}

impl Pin for Pta6 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 6 }
}

impl AfAdc0Se2 for Pta6 {
   fn af_adc0_se2(&self) -> usize { 0 }
}

impl AfPta6 for Pta6 {
   fn af_pta6(&self) -> usize { 1 }
}

impl AfFtm0Flt1 for Pta6 {
   fn af_ftm0_flt1(&self) -> usize { 2 }
}

impl AfLpspi1Pcs1 for Pta6 {
   fn af_lpspi1_pcs1(&self) -> usize { 3 }
}

impl AfLpuart1Cts for Pta6 {
   fn af_lpuart1_cts(&self) -> usize { 6 }
}

pub const PTA7: Pta7 = Pta7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta7 {}

impl Pin for Pta7 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 7 }
}

impl AfAdc0Se3 for Pta7 {
   fn af_adc0_se3(&self) -> usize { 0 }
}

impl AfPta7 for Pta7 {
   fn af_pta7(&self) -> usize { 1 }
}

impl AfFtm0Flt2 for Pta7 {
   fn af_ftm0_flt2(&self) -> usize { 2 }
}

impl AfRtcClkin for Pta7 {
   fn af_rtc_clkin(&self) -> usize { 4 }
}

impl AfLpuart1Rts for Pta7 {
   fn af_lpuart1_rts(&self) -> usize { 6 }
}

pub const PTA8: Pta8 = Pta8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta8 {}

impl Pin for Pta8 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 8 }
}

impl AfPta8 for Pta8 {
   fn af_pta8(&self) -> usize { 1 }
}

impl AfLpuart2Rx for Pta8 {
   fn af_lpuart2_rx(&self) -> usize { 2 }
}

impl AfLpspi2Sout for Pta8 {
   fn af_lpspi2_sout(&self) -> usize { 3 }
}

impl AfFxioD6 for Pta8 {
   fn af_fxio_d6(&self) -> usize { 4 }
}

impl AfFtm3Flt3 for Pta8 {
   fn af_ftm3_flt3(&self) -> usize { 5 }
}

pub const PTA9: Pta9 = Pta9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta9 {}

impl Pin for Pta9 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 9 }
}

impl AfPta9 for Pta9 {
   fn af_pta9(&self) -> usize { 1 }
}

impl AfLpuart2Tx for Pta9 {
   fn af_lpuart2_tx(&self) -> usize { 2 }
}

impl AfLpspi2Pcs0 for Pta9 {
   fn af_lpspi2_pcs0(&self) -> usize { 3 }
}

impl AfFxioD7 for Pta9 {
   fn af_fxio_d7(&self) -> usize { 4 }
}

impl AfFtm3Flt2 for Pta9 {
   fn af_ftm3_flt2(&self) -> usize { 5 }
}

impl AfFtm1Flt3 for Pta9 {
   fn af_ftm1_flt3(&self) -> usize { 6 }
}

pub const PTA10: Pta10 = Pta10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta10 {}

impl Pin for Pta10 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 10 }
}

impl AfPta10 for Pta10 {
   fn af_pta10(&self) -> usize { 1 }
}

impl AfFtm1Ch4 for Pta10 {
   fn af_ftm1_ch4(&self) -> usize { 2 }
}

impl AfFxioD0 for Pta10 {
   fn af_fxio_d0(&self) -> usize { 4 }
}

impl AfJtagTdo for Pta10 {
   fn af_jtag_tdo(&self) -> usize { 7 }
}

impl AfNoetmTraceSwo for Pta10 {
   fn af_noetm_trace_swo(&self) -> usize { 7 }
}

pub const PTA11: Pta11 = Pta11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta11 {}

impl Pin for Pta11 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 11 }
}

impl AfPta11 for Pta11 {
   fn af_pta11(&self) -> usize { 1 }
}

impl AfFtm1Ch5 for Pta11 {
   fn af_ftm1_ch5(&self) -> usize { 2 }
}

impl AfFxioD1 for Pta11 {
   fn af_fxio_d1(&self) -> usize { 4 }
}

impl AfCmp0Rrt for Pta11 {
   fn af_cmp0_rrt(&self) -> usize { 5 }
}

pub const PTA12: Pta12 = Pta12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta12 {}

impl Pin for Pta12 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 12 }
}

impl AfPta12 for Pta12 {
   fn af_pta12(&self) -> usize { 1 }
}

impl AfFtm1Ch6 for Pta12 {
   fn af_ftm1_ch6(&self) -> usize { 2 }
}

impl AfCan1Rx for Pta12 {
   fn af_can1_rx(&self) -> usize { 3 }
}

impl AfFtm2QdPhb for Pta12 {
   fn af_ftm2_qd_phb(&self) -> usize { 6 }
}

pub const PTA13: Pta13 = Pta13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta13 {}

impl Pin for Pta13 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 13 }
}

impl AfPta13 for Pta13 {
   fn af_pta13(&self) -> usize { 1 }
}

impl AfFtm1Ch7 for Pta13 {
   fn af_ftm1_ch7(&self) -> usize { 2 }
}

impl AfCan1Tx for Pta13 {
   fn af_can1_tx(&self) -> usize { 3 }
}

impl AfFtm2QdPha for Pta13 {
   fn af_ftm2_qd_pha(&self) -> usize { 6 }
}

pub const PTA14: Pta14 = Pta14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta14 {}

impl Pin for Pta14 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 14 }
}

impl AfPta14 for Pta14 {
   fn af_pta14(&self) -> usize { 1 }
}

impl AfFtm0Flt0 for Pta14 {
   fn af_ftm0_flt0(&self) -> usize { 2 }
}

impl AfFtm3Flt1 for Pta14 {
   fn af_ftm3_flt1(&self) -> usize { 3 }
}

impl AfEwmIn for Pta14 {
   fn af_ewm_in(&self) -> usize { 4 }
}

impl AfFtm1Flt0 for Pta14 {
   fn af_ftm1_flt0(&self) -> usize { 6 }
}

pub const PTA15: Pta15 = Pta15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta15 {}

impl Pin for Pta15 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 15 }
}

impl AfAdc1Se12 for Pta15 {
   fn af_adc1_se12(&self) -> usize { 0 }
}

impl AfPta15 for Pta15 {
   fn af_pta15(&self) -> usize { 1 }
}

impl AfFtm1Ch2 for Pta15 {
   fn af_ftm1_ch2(&self) -> usize { 2 }
}

impl AfLpspi0Pcs3 for Pta15 {
   fn af_lpspi0_pcs3(&self) -> usize { 3 }
}

impl AfLpspi2Pcs3 for Pta15 {
   fn af_lpspi2_pcs3(&self) -> usize { 4 }
}

pub const PTA16: Pta16 = Pta16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta16 {}

impl Pin for Pta16 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 16 }
}

impl AfAdc1Se13 for Pta16 {
   fn af_adc1_se13(&self) -> usize { 0 }
}

impl AfPta16 for Pta16 {
   fn af_pta16(&self) -> usize { 1 }
}

impl AfFtm1Ch3 for Pta16 {
   fn af_ftm1_ch3(&self) -> usize { 2 }
}

impl AfLpspi1Pcs2 for Pta16 {
   fn af_lpspi1_pcs2(&self) -> usize { 3 }
}

pub const PTA17: Pta17 = Pta17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta17 {}

impl Pin for Pta17 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 17 }
}

impl AfPta17 for Pta17 {
   fn af_pta17(&self) -> usize { 1 }
}

impl AfFtm0Ch6 for Pta17 {
   fn af_ftm0_ch6(&self) -> usize { 2 }
}

impl AfFtm3Flt0 for Pta17 {
   fn af_ftm3_flt0(&self) -> usize { 3 }
}

impl AfEwmOutB for Pta17 {
   fn af_ewm_out_b(&self) -> usize { 4 }
}

pub const PTB0: Ptb0 = Ptb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb0 {}

impl Pin for Ptb0 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se4 for Ptb0 {
   fn af_adc0_se4(&self) -> usize { 0 }
}

impl AfAdc1Se14 for Ptb0 {
   fn af_adc1_se14(&self) -> usize { 0 }
}

impl AfPtb0 for Ptb0 {
   fn af_ptb0(&self) -> usize { 1 }
}

impl AfLpuart0Rx for Ptb0 {
   fn af_lpuart0_rx(&self) -> usize { 2 }
}

impl AfLpspi0Pcs0 for Ptb0 {
   fn af_lpspi0_pcs0(&self) -> usize { 3 }
}

impl AfLptmr0Alt3 for Ptb0 {
   fn af_lptmr0_alt3(&self) -> usize { 4 }
}

impl AfCan0Rx for Ptb0 {
   fn af_can0_rx(&self) -> usize { 5 }
}

pub const PTB1: Ptb1 = Ptb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb1 {}

impl Pin for Ptb1 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se5 for Ptb1 {
   fn af_adc0_se5(&self) -> usize { 0 }
}

impl AfAdc1Se15 for Ptb1 {
   fn af_adc1_se15(&self) -> usize { 0 }
}

impl AfPtb1 for Ptb1 {
   fn af_ptb1(&self) -> usize { 1 }
}

impl AfLpuart0Tx for Ptb1 {
   fn af_lpuart0_tx(&self) -> usize { 2 }
}

impl AfLpspi0Sout for Ptb1 {
   fn af_lpspi0_sout(&self) -> usize { 3 }
}

impl AfTclk0 for Ptb1 {
   fn af_tclk0(&self) -> usize { 4 }
}

impl AfCan0Tx for Ptb1 {
   fn af_can0_tx(&self) -> usize { 5 }
}

pub const PTB2: Ptb2 = Ptb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb2 {}

impl Pin for Ptb2 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se6 for Ptb2 {
   fn af_adc0_se6(&self) -> usize { 0 }
}

impl AfPtb2 for Ptb2 {
   fn af_ptb2(&self) -> usize { 1 }
}

impl AfFtm1Ch0 for Ptb2 {
   fn af_ftm1_ch0(&self) -> usize { 2 }
}

impl AfLpspi0Sck for Ptb2 {
   fn af_lpspi0_sck(&self) -> usize { 3 }
}

impl AfFtm1QdPhb for Ptb2 {
   fn af_ftm1_qd_phb(&self) -> usize { 4 }
}

impl AfTrgmuxIn3 for Ptb2 {
   fn af_trgmux_in3(&self) -> usize { 6 }
}

pub const PTB3: Ptb3 = Ptb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb3 {}

impl Pin for Ptb3 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 3 }
}

impl AfAdc0Se7 for Ptb3 {
   fn af_adc0_se7(&self) -> usize { 0 }
}

impl AfPtb3 for Ptb3 {
   fn af_ptb3(&self) -> usize { 1 }
}

impl AfFtm1Ch1 for Ptb3 {
   fn af_ftm1_ch1(&self) -> usize { 2 }
}

impl AfLpspi0Sin for Ptb3 {
   fn af_lpspi0_sin(&self) -> usize { 3 }
}

impl AfFtm1QdPha for Ptb3 {
   fn af_ftm1_qd_pha(&self) -> usize { 4 }
}

impl AfTrgmuxIn2 for Ptb3 {
   fn af_trgmux_in2(&self) -> usize { 6 }
}

pub const PTB4: Ptb4 = Ptb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb4 {}

impl Pin for Ptb4 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 4 }
}

impl AfPtb4 for Ptb4 {
   fn af_ptb4(&self) -> usize { 1 }
}

impl AfFtm0Ch4 for Ptb4 {
   fn af_ftm0_ch4(&self) -> usize { 2 }
}

impl AfLpspi0Sout for Ptb4 {
   fn af_lpspi0_sout(&self) -> usize { 3 }
}

impl AfTrgmuxIn1 for Ptb4 {
   fn af_trgmux_in1(&self) -> usize { 6 }
}

pub const PTB5: Ptb5 = Ptb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb5 {}

impl Pin for Ptb5 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 5 }
}

impl AfPtb5 for Ptb5 {
   fn af_ptb5(&self) -> usize { 1 }
}

impl AfFtm0Ch5 for Ptb5 {
   fn af_ftm0_ch5(&self) -> usize { 2 }
}

impl AfLpspi0Pcs1 for Ptb5 {
   fn af_lpspi0_pcs1(&self) -> usize { 3 }
}

impl AfLpspi0Pcs0 for Ptb5 {
   fn af_lpspi0_pcs0(&self) -> usize { 4 }
}

impl AfClkout for Ptb5 {
   fn af_clkout(&self) -> usize { 5 }
}

impl AfTrgmuxIn0 for Ptb5 {
   fn af_trgmux_in0(&self) -> usize { 6 }
}

pub const PTB6: Ptb6 = Ptb6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb6 {}

impl Pin for Ptb6 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 6 }
}

impl AfXtal for Ptb6 {
   fn af_xtal(&self) -> usize { 0 }
}

impl AfPtb6 for Ptb6 {
   fn af_ptb6(&self) -> usize { 1 }
}

impl AfLpi2c0Sda for Ptb6 {
   fn af_lpi2c0_sda(&self) -> usize { 2 }
}

pub const PTB7: Ptb7 = Ptb7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb7 {}

impl Pin for Ptb7 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 7 }
}

impl AfExtal for Ptb7 {
   fn af_extal(&self) -> usize { 0 }
}

impl AfPtb7 for Ptb7 {
   fn af_ptb7(&self) -> usize { 1 }
}

impl AfLpi2c0Scl for Ptb7 {
   fn af_lpi2c0_scl(&self) -> usize { 2 }
}

pub const PTB8: Ptb8 = Ptb8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb8 {}

impl Pin for Ptb8 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 8 }
}

impl AfPtb8 for Ptb8 {
   fn af_ptb8(&self) -> usize { 1 }
}

impl AfFtm3Ch0 for Ptb8 {
   fn af_ftm3_ch0(&self) -> usize { 2 }
}

pub const PTB9: Ptb9 = Ptb9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb9 {}

impl Pin for Ptb9 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 9 }
}

impl AfPtb9 for Ptb9 {
   fn af_ptb9(&self) -> usize { 1 }
}

impl AfFtm3Ch1 for Ptb9 {
   fn af_ftm3_ch1(&self) -> usize { 2 }
}

impl AfLpi2c0Scls for Ptb9 {
   fn af_lpi2c0_scls(&self) -> usize { 3 }
}

pub const PTB10: Ptb10 = Ptb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb10 {}

impl Pin for Ptb10 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 10 }
}

impl AfPtb10 for Ptb10 {
   fn af_ptb10(&self) -> usize { 1 }
}

impl AfFtm3Ch2 for Ptb10 {
   fn af_ftm3_ch2(&self) -> usize { 2 }
}

impl AfLpi2c0Sdas for Ptb10 {
   fn af_lpi2c0_sdas(&self) -> usize { 3 }
}

pub const PTB11: Ptb11 = Ptb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb11 {}

impl Pin for Ptb11 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 11 }
}

impl AfPtb11 for Ptb11 {
   fn af_ptb11(&self) -> usize { 1 }
}

impl AfFtm3Ch3 for Ptb11 {
   fn af_ftm3_ch3(&self) -> usize { 2 }
}

impl AfLpi2c0Hreq for Ptb11 {
   fn af_lpi2c0_hreq(&self) -> usize { 3 }
}

pub const PTB12: Ptb12 = Ptb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb12 {}

impl Pin for Ptb12 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 12 }
}

impl AfAdc1Se7 for Ptb12 {
   fn af_adc1_se7(&self) -> usize { 0 }
}

impl AfPtb12 for Ptb12 {
   fn af_ptb12(&self) -> usize { 1 }
}

impl AfFtm0Ch0 for Ptb12 {
   fn af_ftm0_ch0(&self) -> usize { 2 }
}

impl AfFtm3Flt2 for Ptb12 {
   fn af_ftm3_flt2(&self) -> usize { 3 }
}

impl AfCan2Rx for Ptb12 {
   fn af_can2_rx(&self) -> usize { 4 }
}

pub const PTB13: Ptb13 = Ptb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb13 {}

impl Pin for Ptb13 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 13 }
}

impl AfAdc1Se8 for Ptb13 {
   fn af_adc1_se8(&self) -> usize { 0 }
}

impl AfAdc0Se8 for Ptb13 {
   fn af_adc0_se8(&self) -> usize { 0 }
}

impl AfPtb13 for Ptb13 {
   fn af_ptb13(&self) -> usize { 1 }
}

impl AfFtm0Ch1 for Ptb13 {
   fn af_ftm0_ch1(&self) -> usize { 2 }
}

impl AfFtm3Flt1 for Ptb13 {
   fn af_ftm3_flt1(&self) -> usize { 3 }
}

impl AfCan2Tx for Ptb13 {
   fn af_can2_tx(&self) -> usize { 4 }
}

pub const PTB14: Ptb14 = Ptb14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb14 {}

impl Pin for Ptb14 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 14 }
}

impl AfAdc1Se9 for Ptb14 {
   fn af_adc1_se9(&self) -> usize { 0 }
}

impl AfAdc0Se9 for Ptb14 {
   fn af_adc0_se9(&self) -> usize { 0 }
}

impl AfPtb14 for Ptb14 {
   fn af_ptb14(&self) -> usize { 1 }
}

impl AfFtm0Ch2 for Ptb14 {
   fn af_ftm0_ch2(&self) -> usize { 2 }
}

impl AfLpspi1Sck for Ptb14 {
   fn af_lpspi1_sck(&self) -> usize { 3 }
}

pub const PTB15: Ptb15 = Ptb15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb15 {}

impl Pin for Ptb15 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 15 }
}

impl AfAdc1Se14 for Ptb15 {
   fn af_adc1_se14(&self) -> usize { 0 }
}

impl AfPtb15 for Ptb15 {
   fn af_ptb15(&self) -> usize { 1 }
}

impl AfFtm0Ch3 for Ptb15 {
   fn af_ftm0_ch3(&self) -> usize { 2 }
}

impl AfLpspi1Sin for Ptb15 {
   fn af_lpspi1_sin(&self) -> usize { 3 }
}

pub const PTB16: Ptb16 = Ptb16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb16 {}

impl Pin for Ptb16 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 16 }
}

impl AfAdc1Se15 for Ptb16 {
   fn af_adc1_se15(&self) -> usize { 0 }
}

impl AfPtb16 for Ptb16 {
   fn af_ptb16(&self) -> usize { 1 }
}

impl AfFtm0Ch4 for Ptb16 {
   fn af_ftm0_ch4(&self) -> usize { 2 }
}

impl AfLpspi1Sout for Ptb16 {
   fn af_lpspi1_sout(&self) -> usize { 3 }
}

pub const PTB17: Ptb17 = Ptb17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb17 {}

impl Pin for Ptb17 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 17 }
}

impl AfPtb17 for Ptb17 {
   fn af_ptb17(&self) -> usize { 1 }
}

impl AfFtm0Ch5 for Ptb17 {
   fn af_ftm0_ch5(&self) -> usize { 2 }
}

impl AfLpspi1Pcs3 for Ptb17 {
   fn af_lpspi1_pcs3(&self) -> usize { 3 }
}

pub const PTC0: Ptc0 = Ptc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc0 {}

impl Pin for Ptc0 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se8 for Ptc0 {
   fn af_adc0_se8(&self) -> usize { 0 }
}

impl AfPtc0 for Ptc0 {
   fn af_ptc0(&self) -> usize { 1 }
}

impl AfFtm0Ch0 for Ptc0 {
   fn af_ftm0_ch0(&self) -> usize { 2 }
}

impl AfLpspi2Sin for Ptc0 {
   fn af_lpspi2_sin(&self) -> usize { 3 }
}

impl AfFtm1Ch6 for Ptc0 {
   fn af_ftm1_ch6(&self) -> usize { 6 }
}

pub const PTC1: Ptc1 = Ptc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc1 {}

impl Pin for Ptc1 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se9 for Ptc1 {
   fn af_adc0_se9(&self) -> usize { 0 }
}

impl AfPtc1 for Ptc1 {
   fn af_ptc1(&self) -> usize { 1 }
}

impl AfFtm0Ch1 for Ptc1 {
   fn af_ftm0_ch1(&self) -> usize { 2 }
}

impl AfLpspi2Sout for Ptc1 {
   fn af_lpspi2_sout(&self) -> usize { 3 }
}

impl AfFtm1Ch7 for Ptc1 {
   fn af_ftm1_ch7(&self) -> usize { 6 }
}

pub const PTC2: Ptc2 = Ptc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc2 {}

impl Pin for Ptc2 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se10 for Ptc2 {
   fn af_adc0_se10(&self) -> usize { 0 }
}

impl AfCmp0In5 for Ptc2 {
   fn af_cmp0_in5(&self) -> usize { 0 }
}

impl AfPtc2 for Ptc2 {
   fn af_ptc2(&self) -> usize { 1 }
}

impl AfFtm0Ch2 for Ptc2 {
   fn af_ftm0_ch2(&self) -> usize { 2 }
}

impl AfCan0Rx for Ptc2 {
   fn af_can0_rx(&self) -> usize { 3 }
}

impl AfLpuart0Rx for Ptc2 {
   fn af_lpuart0_rx(&self) -> usize { 4 }
}

pub const PTC3: Ptc3 = Ptc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc3 {}

impl Pin for Ptc3 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 3 }
}

impl AfAdc0Se11 for Ptc3 {
   fn af_adc0_se11(&self) -> usize { 0 }
}

impl AfCmp0In4 for Ptc3 {
   fn af_cmp0_in4(&self) -> usize { 0 }
}

impl AfPtc3 for Ptc3 {
   fn af_ptc3(&self) -> usize { 1 }
}

impl AfFtm0Ch3 for Ptc3 {
   fn af_ftm0_ch3(&self) -> usize { 2 }
}

impl AfCan0Tx for Ptc3 {
   fn af_can0_tx(&self) -> usize { 3 }
}

impl AfLpuart0Tx for Ptc3 {
   fn af_lpuart0_tx(&self) -> usize { 4 }
}

pub const PTC4: Ptc4 = Ptc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc4 {}

impl Pin for Ptc4 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 4 }
}

impl AfCmp0In2 for Ptc4 {
   fn af_cmp0_in2(&self) -> usize { 0 }
}

impl AfPtc4 for Ptc4 {
   fn af_ptc4(&self) -> usize { 1 }
}

impl AfFtm1Ch0 for Ptc4 {
   fn af_ftm1_ch0(&self) -> usize { 2 }
}

impl AfRtcClkout for Ptc4 {
   fn af_rtc_clkout(&self) -> usize { 3 }
}

impl AfEwmIn for Ptc4 {
   fn af_ewm_in(&self) -> usize { 5 }
}

impl AfFtm1QdPhb for Ptc4 {
   fn af_ftm1_qd_phb(&self) -> usize { 6 }
}

impl AfJtagTclk for Ptc4 {
   fn af_jtag_tclk(&self) -> usize { 7 }
}

impl AfSwdClk for Ptc4 {
   fn af_swd_clk(&self) -> usize { 7 }
}

pub const PTC5: Ptc5 = Ptc5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc5 {}

impl Pin for Ptc5 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 5 }
}

impl AfPtc5 for Ptc5 {
   fn af_ptc5(&self) -> usize { 1 }
}

impl AfFtm2Ch0 for Ptc5 {
   fn af_ftm2_ch0(&self) -> usize { 2 }
}

impl AfRtcClkout for Ptc5 {
   fn af_rtc_clkout(&self) -> usize { 3 }
}

impl AfFtm2QdPhb for Ptc5 {
   fn af_ftm2_qd_phb(&self) -> usize { 6 }
}

impl AfJtagTdi for Ptc5 {
   fn af_jtag_tdi(&self) -> usize { 7 }
}

pub const PTC6: Ptc6 = Ptc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc6 {}

impl Pin for Ptc6 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 6 }
}

impl AfAdc1Se4 for Ptc6 {
   fn af_adc1_se4(&self) -> usize { 0 }
}

impl AfPtc6 for Ptc6 {
   fn af_ptc6(&self) -> usize { 1 }
}

impl AfLpuart1Rx for Ptc6 {
   fn af_lpuart1_rx(&self) -> usize { 2 }
}

impl AfCan1Rx for Ptc6 {
   fn af_can1_rx(&self) -> usize { 3 }
}

impl AfFtm3Ch2 for Ptc6 {
   fn af_ftm3_ch2(&self) -> usize { 4 }
}

impl AfFtm1QdPhb for Ptc6 {
   fn af_ftm1_qd_phb(&self) -> usize { 6 }
}

pub const PTC7: Ptc7 = Ptc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc7 {}

impl Pin for Ptc7 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 7 }
}

impl AfAdc1Se5 for Ptc7 {
   fn af_adc1_se5(&self) -> usize { 0 }
}

impl AfPtc7 for Ptc7 {
   fn af_ptc7(&self) -> usize { 1 }
}

impl AfLpuart1Tx for Ptc7 {
   fn af_lpuart1_tx(&self) -> usize { 2 }
}

impl AfCan1Tx for Ptc7 {
   fn af_can1_tx(&self) -> usize { 3 }
}

impl AfFtm3Ch3 for Ptc7 {
   fn af_ftm3_ch3(&self) -> usize { 4 }
}

impl AfFtm1QdPha for Ptc7 {
   fn af_ftm1_qd_pha(&self) -> usize { 6 }
}

pub const PTC8: Ptc8 = Ptc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc8 {}

impl Pin for Ptc8 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 8 }
}

impl AfPtc8 for Ptc8 {
   fn af_ptc8(&self) -> usize { 1 }
}

impl AfLpuart1Rx for Ptc8 {
   fn af_lpuart1_rx(&self) -> usize { 2 }
}

impl AfFtm1Flt0 for Ptc8 {
   fn af_ftm1_flt0(&self) -> usize { 3 }
}

impl AfLpuart0Cts for Ptc8 {
   fn af_lpuart0_cts(&self) -> usize { 6 }
}

pub const PTC9: Ptc9 = Ptc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc9 {}

impl Pin for Ptc9 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 9 }
}

impl AfPtc9 for Ptc9 {
   fn af_ptc9(&self) -> usize { 1 }
}

impl AfLpuart1Tx for Ptc9 {
   fn af_lpuart1_tx(&self) -> usize { 2 }
}

impl AfFtm1Flt1 for Ptc9 {
   fn af_ftm1_flt1(&self) -> usize { 3 }
}

impl AfLpuart0Rts for Ptc9 {
   fn af_lpuart0_rts(&self) -> usize { 6 }
}

pub const PTC10: Ptc10 = Ptc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc10 {}

impl Pin for Ptc10 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 10 }
}

impl AfPtc10 for Ptc10 {
   fn af_ptc10(&self) -> usize { 1 }
}

impl AfFtm3Ch4 for Ptc10 {
   fn af_ftm3_ch4(&self) -> usize { 2 }
}

impl AfTrgmuxIn11 for Ptc10 {
   fn af_trgmux_in11(&self) -> usize { 6 }
}

pub const PTC11: Ptc11 = Ptc11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc11 {}

impl Pin for Ptc11 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 11 }
}

impl AfPtc11 for Ptc11 {
   fn af_ptc11(&self) -> usize { 1 }
}

impl AfFtm3Ch5 for Ptc11 {
   fn af_ftm3_ch5(&self) -> usize { 2 }
}

impl AfTrgmuxIn10 for Ptc11 {
   fn af_trgmux_in10(&self) -> usize { 6 }
}

pub const PTC12: Ptc12 = Ptc12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc12 {}

impl Pin for Ptc12 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 12 }
}

impl AfPtc12 for Ptc12 {
   fn af_ptc12(&self) -> usize { 1 }
}

impl AfFtm3Ch6 for Ptc12 {
   fn af_ftm3_ch6(&self) -> usize { 2 }
}

impl AfFtm2Ch6 for Ptc12 {
   fn af_ftm2_ch6(&self) -> usize { 3 }
}

impl AfLpuart2Cts for Ptc12 {
   fn af_lpuart2_cts(&self) -> usize { 4 }
}

pub const PTC13: Ptc13 = Ptc13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc13 {}

impl Pin for Ptc13 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 13 }
}

impl AfPtc13 for Ptc13 {
   fn af_ptc13(&self) -> usize { 1 }
}

impl AfFtm3Ch7 for Ptc13 {
   fn af_ftm3_ch7(&self) -> usize { 2 }
}

impl AfFtm2Ch7 for Ptc13 {
   fn af_ftm2_ch7(&self) -> usize { 3 }
}

impl AfLpuart2Rts for Ptc13 {
   fn af_lpuart2_rts(&self) -> usize { 4 }
}

pub const PTC14: Ptc14 = Ptc14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc14 {}

impl Pin for Ptc14 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 14 }
}

impl AfAdc0Se12 for Ptc14 {
   fn af_adc0_se12(&self) -> usize { 0 }
}

impl AfPtc14 for Ptc14 {
   fn af_ptc14(&self) -> usize { 1 }
}

impl AfFtm1Ch2 for Ptc14 {
   fn af_ftm1_ch2(&self) -> usize { 2 }
}

impl AfLpspi2Pcs0 for Ptc14 {
   fn af_lpspi2_pcs0(&self) -> usize { 3 }
}

impl AfTrgmuxIn9 for Ptc14 {
   fn af_trgmux_in9(&self) -> usize { 6 }
}

pub const PTC15: Ptc15 = Ptc15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc15 {}

impl Pin for Ptc15 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 15 }
}

impl AfAdc0Se13 for Ptc15 {
   fn af_adc0_se13(&self) -> usize { 0 }
}

impl AfPtc15 for Ptc15 {
   fn af_ptc15(&self) -> usize { 1 }
}

impl AfFtm1Ch3 for Ptc15 {
   fn af_ftm1_ch3(&self) -> usize { 2 }
}

impl AfLpspi2Sck for Ptc15 {
   fn af_lpspi2_sck(&self) -> usize { 3 }
}

impl AfTrgmuxIn8 for Ptc15 {
   fn af_trgmux_in8(&self) -> usize { 6 }
}

pub const PTC16: Ptc16 = Ptc16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc16 {}

impl Pin for Ptc16 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 16 }
}

impl AfAdc0Se14 for Ptc16 {
   fn af_adc0_se14(&self) -> usize { 0 }
}

impl AfPtc16 for Ptc16 {
   fn af_ptc16(&self) -> usize { 1 }
}

impl AfFtm1Flt2 for Ptc16 {
   fn af_ftm1_flt2(&self) -> usize { 2 }
}

impl AfCan2Rx for Ptc16 {
   fn af_can2_rx(&self) -> usize { 3 }
}

pub const PTC17: Ptc17 = Ptc17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc17 {}

impl Pin for Ptc17 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 17 }
}

impl AfAdc0Se15 for Ptc17 {
   fn af_adc0_se15(&self) -> usize { 0 }
}

impl AfPtc17 for Ptc17 {
   fn af_ptc17(&self) -> usize { 1 }
}

impl AfFtm1Flt3 for Ptc17 {
   fn af_ftm1_flt3(&self) -> usize { 2 }
}

impl AfCan2Tx for Ptc17 {
   fn af_can2_tx(&self) -> usize { 3 }
}

pub const PTD0: Ptd0 = Ptd0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd0 {}

impl Pin for Ptd0 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 0 }
}

impl AfPtd0 for Ptd0 {
   fn af_ptd0(&self) -> usize { 1 }
}

impl AfFtm0Ch2 for Ptd0 {
   fn af_ftm0_ch2(&self) -> usize { 2 }
}

impl AfLpspi1Sck for Ptd0 {
   fn af_lpspi1_sck(&self) -> usize { 3 }
}

impl AfFtm2Ch0 for Ptd0 {
   fn af_ftm2_ch0(&self) -> usize { 4 }
}

impl AfFxioD0 for Ptd0 {
   fn af_fxio_d0(&self) -> usize { 6 }
}

impl AfTrgmuxOut1 for Ptd0 {
   fn af_trgmux_out1(&self) -> usize { 7 }
}

pub const PTD1: Ptd1 = Ptd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd1 {}

impl Pin for Ptd1 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 1 }
}

impl AfPtd1 for Ptd1 {
   fn af_ptd1(&self) -> usize { 1 }
}

impl AfFtm0Ch3 for Ptd1 {
   fn af_ftm0_ch3(&self) -> usize { 2 }
}

impl AfLpspi1Sin for Ptd1 {
   fn af_lpspi1_sin(&self) -> usize { 3 }
}

impl AfFtm2Ch1 for Ptd1 {
   fn af_ftm2_ch1(&self) -> usize { 4 }
}

impl AfFxioD1 for Ptd1 {
   fn af_fxio_d1(&self) -> usize { 6 }
}

impl AfTrgmuxOut2 for Ptd1 {
   fn af_trgmux_out2(&self) -> usize { 7 }
}

pub const PTD2: Ptd2 = Ptd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd2 {}

impl Pin for Ptd2 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 2 }
}

impl AfAdc1Se2 for Ptd2 {
   fn af_adc1_se2(&self) -> usize { 0 }
}

impl AfPtd2 for Ptd2 {
   fn af_ptd2(&self) -> usize { 1 }
}

impl AfFtm3Ch4 for Ptd2 {
   fn af_ftm3_ch4(&self) -> usize { 2 }
}

impl AfLpspi1Sout for Ptd2 {
   fn af_lpspi1_sout(&self) -> usize { 3 }
}

impl AfFxioD4 for Ptd2 {
   fn af_fxio_d4(&self) -> usize { 4 }
}

impl AfFxioD6 for Ptd2 {
   fn af_fxio_d6(&self) -> usize { 5 }
}

impl AfTrgmuxIn5 for Ptd2 {
   fn af_trgmux_in5(&self) -> usize { 6 }
}

pub const PTD3: Ptd3 = Ptd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd3 {}

impl Pin for Ptd3 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 3 }
}

impl AfAdc1Se3 for Ptd3 {
   fn af_adc1_se3(&self) -> usize { 0 }
}

impl AfPtd3 for Ptd3 {
   fn af_ptd3(&self) -> usize { 1 }
}

impl AfFtm3Ch5 for Ptd3 {
   fn af_ftm3_ch5(&self) -> usize { 2 }
}

impl AfLpspi1Pcs0 for Ptd3 {
   fn af_lpspi1_pcs0(&self) -> usize { 3 }
}

impl AfFxioD5 for Ptd3 {
   fn af_fxio_d5(&self) -> usize { 4 }
}

impl AfFxioD7 for Ptd3 {
   fn af_fxio_d7(&self) -> usize { 5 }
}

impl AfTrgmuxIn4 for Ptd3 {
   fn af_trgmux_in4(&self) -> usize { 6 }
}

impl AfNmiB for Ptd3 {
   fn af_nmi_b(&self) -> usize { 7 }
}

pub const PTD4: Ptd4 = Ptd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd4 {}

impl Pin for Ptd4 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 4 }
}

impl AfAdc1Se6 for Ptd4 {
   fn af_adc1_se6(&self) -> usize { 0 }
}

impl AfPtd4 for Ptd4 {
   fn af_ptd4(&self) -> usize { 1 }
}

impl AfFtm0Flt3 for Ptd4 {
   fn af_ftm0_flt3(&self) -> usize { 2 }
}

impl AfFtm3Flt3 for Ptd4 {
   fn af_ftm3_flt3(&self) -> usize { 3 }
}

pub const PTD5: Ptd5 = Ptd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd5 {}

impl Pin for Ptd5 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 5 }
}

impl AfPtd5 for Ptd5 {
   fn af_ptd5(&self) -> usize { 1 }
}

impl AfFtm2Ch3 for Ptd5 {
   fn af_ftm2_ch3(&self) -> usize { 2 }
}

impl AfLptmr0Alt2 for Ptd5 {
   fn af_lptmr0_alt2(&self) -> usize { 3 }
}

impl AfFtm2Flt1 for Ptd5 {
   fn af_ftm2_flt1(&self) -> usize { 4 }
}

impl AfTrgmuxIn7 for Ptd5 {
   fn af_trgmux_in7(&self) -> usize { 6 }
}

pub const PTD6: Ptd6 = Ptd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd6 {}

impl Pin for Ptd6 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 6 }
}

impl AfCmp0In7 for Ptd6 {
   fn af_cmp0_in7(&self) -> usize { 0 }
}

impl AfPtd6 for Ptd6 {
   fn af_ptd6(&self) -> usize { 1 }
}

impl AfLpuart2Rx for Ptd6 {
   fn af_lpuart2_rx(&self) -> usize { 2 }
}

impl AfFtm2Flt2 for Ptd6 {
   fn af_ftm2_flt2(&self) -> usize { 4 }
}

pub const PTD7: Ptd7 = Ptd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd7 {}

impl Pin for Ptd7 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 7 }
}

impl AfCmp0In6 for Ptd7 {
   fn af_cmp0_in6(&self) -> usize { 0 }
}

impl AfPtd7 for Ptd7 {
   fn af_ptd7(&self) -> usize { 1 }
}

impl AfLpuart2Tx for Ptd7 {
   fn af_lpuart2_tx(&self) -> usize { 2 }
}

impl AfFtm2Flt3 for Ptd7 {
   fn af_ftm2_flt3(&self) -> usize { 4 }
}

pub const PTD8: Ptd8 = Ptd8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd8 {}

impl Pin for Ptd8 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 8 }
}

impl AfPtd8 for Ptd8 {
   fn af_ptd8(&self) -> usize { 1 }
}

impl AfFtm2Flt2 for Ptd8 {
   fn af_ftm2_flt2(&self) -> usize { 4 }
}

impl AfFxioD1 for Ptd8 {
   fn af_fxio_d1(&self) -> usize { 5 }
}

impl AfFtm1Ch4 for Ptd8 {
   fn af_ftm1_ch4(&self) -> usize { 6 }
}

pub const PTD9: Ptd9 = Ptd9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd9 {}

impl Pin for Ptd9 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 9 }
}

impl AfPtd9 for Ptd9 {
   fn af_ptd9(&self) -> usize { 1 }
}

impl AfFxioD0 for Ptd9 {
   fn af_fxio_d0(&self) -> usize { 3 }
}

impl AfFtm2Flt3 for Ptd9 {
   fn af_ftm2_flt3(&self) -> usize { 4 }
}

impl AfFtm1Ch5 for Ptd9 {
   fn af_ftm1_ch5(&self) -> usize { 6 }
}

pub const PTD10: Ptd10 = Ptd10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd10 {}

impl Pin for Ptd10 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 10 }
}

impl AfPtd10 for Ptd10 {
   fn af_ptd10(&self) -> usize { 1 }
}

impl AfFtm2Ch0 for Ptd10 {
   fn af_ftm2_ch0(&self) -> usize { 2 }
}

impl AfFtm2QdPhb for Ptd10 {
   fn af_ftm2_qd_phb(&self) -> usize { 3 }
}

pub const PTD11: Ptd11 = Ptd11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd11 {}

impl Pin for Ptd11 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 11 }
}

impl AfPtd11 for Ptd11 {
   fn af_ptd11(&self) -> usize { 1 }
}

impl AfFtm2Ch1 for Ptd11 {
   fn af_ftm2_ch1(&self) -> usize { 2 }
}

impl AfFtm2QdPha for Ptd11 {
   fn af_ftm2_qd_pha(&self) -> usize { 3 }
}

impl AfLpuart2Cts for Ptd11 {
   fn af_lpuart2_cts(&self) -> usize { 6 }
}

pub const PTD12: Ptd12 = Ptd12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd12 {}

impl Pin for Ptd12 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 12 }
}

impl AfPtd12 for Ptd12 {
   fn af_ptd12(&self) -> usize { 1 }
}

impl AfFtm2Ch2 for Ptd12 {
   fn af_ftm2_ch2(&self) -> usize { 2 }
}

impl AfLpuart2Rts for Ptd12 {
   fn af_lpuart2_rts(&self) -> usize { 6 }
}

pub const PTD13: Ptd13 = Ptd13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd13 {}

impl Pin for Ptd13 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 13 }
}

impl AfPtd13 for Ptd13 {
   fn af_ptd13(&self) -> usize { 1 }
}

impl AfFtm2Ch4 for Ptd13 {
   fn af_ftm2_ch4(&self) -> usize { 2 }
}

impl AfLpuart1Rx for Ptd13 {
   fn af_lpuart1_rx(&self) -> usize { 3 }
}

impl AfRtcClkout for Ptd13 {
   fn af_rtc_clkout(&self) -> usize { 7 }
}

pub const PTD14: Ptd14 = Ptd14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd14 {}

impl Pin for Ptd14 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 14 }
}

impl AfPtd14 for Ptd14 {
   fn af_ptd14(&self) -> usize { 1 }
}

impl AfFtm2Ch5 for Ptd14 {
   fn af_ftm2_ch5(&self) -> usize { 2 }
}

impl AfLpuart1Tx for Ptd14 {
   fn af_lpuart1_tx(&self) -> usize { 3 }
}

impl AfClkout for Ptd14 {
   fn af_clkout(&self) -> usize { 7 }
}

pub const PTD15: Ptd15 = Ptd15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd15 {}

impl Pin for Ptd15 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 15 }
}

impl AfPtd15 for Ptd15 {
   fn af_ptd15(&self) -> usize { 1 }
}

impl AfFtm0Ch0 for Ptd15 {
   fn af_ftm0_ch0(&self) -> usize { 2 }
}

impl AfLpspi0Sck for Ptd15 {
   fn af_lpspi0_sck(&self) -> usize { 4 }
}

pub const PTD16: Ptd16 = Ptd16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd16 {}

impl Pin for Ptd16 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 16 }
}

impl AfPtd16 for Ptd16 {
   fn af_ptd16(&self) -> usize { 1 }
}

impl AfFtm0Ch1 for Ptd16 {
   fn af_ftm0_ch1(&self) -> usize { 2 }
}

impl AfLpspi0Sin for Ptd16 {
   fn af_lpspi0_sin(&self) -> usize { 4 }
}

impl AfCmp0Rrt for Ptd16 {
   fn af_cmp0_rrt(&self) -> usize { 5 }
}

pub const PTD17: Ptd17 = Ptd17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd17 {}

impl Pin for Ptd17 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 17 }
}

impl AfPtd17 for Ptd17 {
   fn af_ptd17(&self) -> usize { 1 }
}

impl AfFtm0Flt2 for Ptd17 {
   fn af_ftm0_flt2(&self) -> usize { 2 }
}

impl AfLpuart2Rx for Ptd17 {
   fn af_lpuart2_rx(&self) -> usize { 3 }
}

pub const PTE0: Pte0 = Pte0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte0 {}

impl Pin for Pte0 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 0 }
}

impl AfPte0 for Pte0 {
   fn af_pte0(&self) -> usize { 1 }
}

impl AfLpspi0Sck for Pte0 {
   fn af_lpspi0_sck(&self) -> usize { 2 }
}

impl AfTclk1 for Pte0 {
   fn af_tclk1(&self) -> usize { 3 }
}

impl AfLpspi1Sout for Pte0 {
   fn af_lpspi1_sout(&self) -> usize { 5 }
}

impl AfFtm1Flt2 for Pte0 {
   fn af_ftm1_flt2(&self) -> usize { 6 }
}

pub const PTE1: Pte1 = Pte1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte1 {}

impl Pin for Pte1 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 1 }
}

impl AfPte1 for Pte1 {
   fn af_pte1(&self) -> usize { 1 }
}

impl AfLpspi0Sin for Pte1 {
   fn af_lpspi0_sin(&self) -> usize { 2 }
}

impl AfLpi2c0Hreq for Pte1 {
   fn af_lpi2c0_hreq(&self) -> usize { 3 }
}

impl AfLpspi1Pcs0 for Pte1 {
   fn af_lpspi1_pcs0(&self) -> usize { 5 }
}

impl AfFtm1Flt1 for Pte1 {
   fn af_ftm1_flt1(&self) -> usize { 6 }
}

pub const PTE2: Pte2 = Pte2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte2 {}

impl Pin for Pte2 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 2 }
}

impl AfAdc1Se10 for Pte2 {
   fn af_adc1_se10(&self) -> usize { 0 }
}

impl AfPte2 for Pte2 {
   fn af_pte2(&self) -> usize { 1 }
}

impl AfLpspi0Sout for Pte2 {
   fn af_lpspi0_sout(&self) -> usize { 2 }
}

impl AfLptmr0Alt3 for Pte2 {
   fn af_lptmr0_alt3(&self) -> usize { 3 }
}

impl AfFtm3Ch6 for Pte2 {
   fn af_ftm3_ch6(&self) -> usize { 4 }
}

impl AfLpuart1Cts for Pte2 {
   fn af_lpuart1_cts(&self) -> usize { 6 }
}

pub const PTE3: Pte3 = Pte3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte3 {}

impl Pin for Pte3 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 3 }
}

impl AfPte3 for Pte3 {
   fn af_pte3(&self) -> usize { 1 }
}

impl AfFtm0Flt0 for Pte3 {
   fn af_ftm0_flt0(&self) -> usize { 2 }
}

impl AfLpuart2Rts for Pte3 {
   fn af_lpuart2_rts(&self) -> usize { 3 }
}

impl AfFtm2Flt0 for Pte3 {
   fn af_ftm2_flt0(&self) -> usize { 4 }
}

impl AfTrgmuxIn6 for Pte3 {
   fn af_trgmux_in6(&self) -> usize { 6 }
}

impl AfCmp0Out for Pte3 {
   fn af_cmp0_out(&self) -> usize { 7 }
}

pub const PTE4: Pte4 = Pte4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte4 {}

impl Pin for Pte4 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 4 }
}

impl AfPte4 for Pte4 {
   fn af_pte4(&self) -> usize { 1 }
}

impl AfFtm2QdPhb for Pte4 {
   fn af_ftm2_qd_phb(&self) -> usize { 3 }
}

impl AfFtm2Ch2 for Pte4 {
   fn af_ftm2_ch2(&self) -> usize { 4 }
}

impl AfCan0Rx for Pte4 {
   fn af_can0_rx(&self) -> usize { 5 }
}

impl AfFxioD6 for Pte4 {
   fn af_fxio_d6(&self) -> usize { 6 }
}

impl AfEwmOutB for Pte4 {
   fn af_ewm_out_b(&self) -> usize { 7 }
}

pub const PTE5: Pte5 = Pte5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte5 {}

impl Pin for Pte5 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 5 }
}

impl AfPte5 for Pte5 {
   fn af_pte5(&self) -> usize { 1 }
}

impl AfTclk2 for Pte5 {
   fn af_tclk2(&self) -> usize { 2 }
}

impl AfFtm2QdPha for Pte5 {
   fn af_ftm2_qd_pha(&self) -> usize { 3 }
}

impl AfFtm2Ch3 for Pte5 {
   fn af_ftm2_ch3(&self) -> usize { 4 }
}

impl AfCan0Tx for Pte5 {
   fn af_can0_tx(&self) -> usize { 5 }
}

impl AfFxioD7 for Pte5 {
   fn af_fxio_d7(&self) -> usize { 6 }
}

impl AfEwmIn for Pte5 {
   fn af_ewm_in(&self) -> usize { 7 }
}

pub const PTE6: Pte6 = Pte6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte6 {}

impl Pin for Pte6 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 6 }
}

impl AfAdc1Se11 for Pte6 {
   fn af_adc1_se11(&self) -> usize { 0 }
}

impl AfPte6 for Pte6 {
   fn af_pte6(&self) -> usize { 1 }
}

impl AfLpspi0Pcs2 for Pte6 {
   fn af_lpspi0_pcs2(&self) -> usize { 2 }
}

impl AfFtm3Ch7 for Pte6 {
   fn af_ftm3_ch7(&self) -> usize { 4 }
}

impl AfLpuart1Rts for Pte6 {
   fn af_lpuart1_rts(&self) -> usize { 6 }
}

pub const PTE7: Pte7 = Pte7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte7 {}

impl Pin for Pte7 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 7 }
}

impl AfPte7 for Pte7 {
   fn af_pte7(&self) -> usize { 1 }
}

impl AfFtm0Ch7 for Pte7 {
   fn af_ftm0_ch7(&self) -> usize { 2 }
}

impl AfFtm3Flt0 for Pte7 {
   fn af_ftm3_flt0(&self) -> usize { 3 }
}

pub const PTE8: Pte8 = Pte8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte8 {}

impl Pin for Pte8 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 8 }
}

impl AfCmp0In3 for Pte8 {
   fn af_cmp0_in3(&self) -> usize { 0 }
}

impl AfPte8 for Pte8 {
   fn af_pte8(&self) -> usize { 1 }
}

impl AfFtm0Ch6 for Pte8 {
   fn af_ftm0_ch6(&self) -> usize { 2 }
}

pub const PTE9: Pte9 = Pte9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte9 {}

impl Pin for Pte9 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 9 }
}

impl AfPte9 for Pte9 {
   fn af_pte9(&self) -> usize { 1 }
}

impl AfFtm0Ch7 for Pte9 {
   fn af_ftm0_ch7(&self) -> usize { 2 }
}

impl AfLpuart2Cts for Pte9 {
   fn af_lpuart2_cts(&self) -> usize { 3 }
}

pub const PTE10: Pte10 = Pte10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte10 {}

impl Pin for Pte10 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 10 }
}

impl AfPte10 for Pte10 {
   fn af_pte10(&self) -> usize { 1 }
}

impl AfClkout for Pte10 {
   fn af_clkout(&self) -> usize { 2 }
}

impl AfLpspi2Pcs1 for Pte10 {
   fn af_lpspi2_pcs1(&self) -> usize { 3 }
}

impl AfFtm2Ch4 for Pte10 {
   fn af_ftm2_ch4(&self) -> usize { 4 }
}

impl AfFxioD4 for Pte10 {
   fn af_fxio_d4(&self) -> usize { 6 }
}

impl AfTrgmuxOut4 for Pte10 {
   fn af_trgmux_out4(&self) -> usize { 7 }
}

pub const PTE11: Pte11 = Pte11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte11 {}

impl Pin for Pte11 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 11 }
}

impl AfPte11 for Pte11 {
   fn af_pte11(&self) -> usize { 1 }
}

impl AfLpspi2Pcs0 for Pte11 {
   fn af_lpspi2_pcs0(&self) -> usize { 2 }
}

impl AfLptmr0Alt1 for Pte11 {
   fn af_lptmr0_alt1(&self) -> usize { 3 }
}

impl AfFtm2Ch5 for Pte11 {
   fn af_ftm2_ch5(&self) -> usize { 4 }
}

impl AfFxioD5 for Pte11 {
   fn af_fxio_d5(&self) -> usize { 6 }
}

impl AfTrgmuxOut5 for Pte11 {
   fn af_trgmux_out5(&self) -> usize { 7 }
}

pub const PTE12: Pte12 = Pte12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte12 {}

impl Pin for Pte12 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 12 }
}

impl AfPte12 for Pte12 {
   fn af_pte12(&self) -> usize { 1 }
}

impl AfFtm0Flt3 for Pte12 {
   fn af_ftm0_flt3(&self) -> usize { 2 }
}

impl AfLpuart2Tx for Pte12 {
   fn af_lpuart2_tx(&self) -> usize { 3 }
}

pub const PTE13: Pte13 = Pte13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte13 {}

impl Pin for Pte13 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 13 }
}

impl AfPte13 for Pte13 {
   fn af_pte13(&self) -> usize { 1 }
}

impl AfLpspi2Pcs2 for Pte13 {
   fn af_lpspi2_pcs2(&self) -> usize { 3 }
}

impl AfFtm2Flt0 for Pte13 {
   fn af_ftm2_flt0(&self) -> usize { 4 }
}

pub const PTE14: Pte14 = Pte14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte14 {}

impl Pin for Pte14 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 14 }
}

impl AfPte14 for Pte14 {
   fn af_pte14(&self) -> usize { 1 }
}

impl AfFtm0Flt1 for Pte14 {
   fn af_ftm0_flt1(&self) -> usize { 2 }
}

impl AfFtm2Flt1 for Pte14 {
   fn af_ftm2_flt1(&self) -> usize { 4 }
}

pub const PTE15: Pte15 = Pte15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte15 {}

impl Pin for Pte15 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 15 }
}

impl AfPte15 for Pte15 {
   fn af_pte15(&self) -> usize { 1 }
}

impl AfLpuart1Cts for Pte15 {
   fn af_lpuart1_cts(&self) -> usize { 2 }
}

impl AfLpspi2Sck for Pte15 {
   fn af_lpspi2_sck(&self) -> usize { 3 }
}

impl AfFtm2Ch6 for Pte15 {
   fn af_ftm2_ch6(&self) -> usize { 4 }
}

impl AfFxioD2 for Pte15 {
   fn af_fxio_d2(&self) -> usize { 6 }
}

impl AfTrgmuxOut6 for Pte15 {
   fn af_trgmux_out6(&self) -> usize { 7 }
}

pub const PTE16: Pte16 = Pte16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte16 {}

impl Pin for Pte16 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 16 }
}

impl AfPte16 for Pte16 {
   fn af_pte16(&self) -> usize { 1 }
}

impl AfLpuart1Rts for Pte16 {
   fn af_lpuart1_rts(&self) -> usize { 2 }
}

impl AfLpspi2Sin for Pte16 {
   fn af_lpspi2_sin(&self) -> usize { 3 }
}

impl AfFtm2Ch7 for Pte16 {
   fn af_ftm2_ch7(&self) -> usize { 4 }
}

impl AfFxioD3 for Pte16 {
   fn af_fxio_d3(&self) -> usize { 6 }
}

impl AfTrgmuxOut7 for Pte16 {
   fn af_trgmux_out7(&self) -> usize { 7 }
}

