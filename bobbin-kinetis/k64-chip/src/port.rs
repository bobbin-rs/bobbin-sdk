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

pub trait AfPta0 {
   fn af_pta0(&self) -> usize;
}

pub trait AfUart0CtsB {
   fn af_uart0_cts_b(&self) -> usize;
}

pub trait AfUart0ColB {
   fn af_uart0_col_b(&self) -> usize;
}

pub trait AfFtm0Ch5 {
   fn af_ftm0_ch5(&self) -> usize;
}

pub trait AfJtagTclk {
   fn af_jtag_tclk(&self) -> usize;
}

pub trait AfSwdClk {
   fn af_swd_clk(&self) -> usize;
}

pub trait AfPta1 {
   fn af_pta1(&self) -> usize;
}

pub trait AfUart0Rx {
   fn af_uart0_rx(&self) -> usize;
}

pub trait AfFtm0Ch6 {
   fn af_ftm0_ch6(&self) -> usize;
}

pub trait AfJtagTdi {
   fn af_jtag_tdi(&self) -> usize;
}

pub trait AfPta2 {
   fn af_pta2(&self) -> usize;
}

pub trait AfUart0Tx {
   fn af_uart0_tx(&self) -> usize;
}

pub trait AfFtm0Ch7 {
   fn af_ftm0_ch7(&self) -> usize;
}

pub trait AfJtagTdo {
   fn af_jtag_tdo(&self) -> usize;
}

pub trait AfTraceSwo {
   fn af_trace_swo(&self) -> usize;
}

pub trait AfPta3 {
   fn af_pta3(&self) -> usize;
}

pub trait AfUart0RtsB {
   fn af_uart0_rts_b(&self) -> usize;
}

pub trait AfFtm0Ch0 {
   fn af_ftm0_ch0(&self) -> usize;
}

pub trait AfJtagTms {
   fn af_jtag_tms(&self) -> usize;
}

pub trait AfSwdDio {
   fn af_swd_dio(&self) -> usize;
}

pub trait AfPta4 {
   fn af_pta4(&self) -> usize;
}

pub trait AfFtm0Ch1 {
   fn af_ftm0_ch1(&self) -> usize;
}

pub trait AfNmiB {
   fn af_nmi_b(&self) -> usize;
}

pub trait AfPta5 {
   fn af_pta5(&self) -> usize;
}

pub trait AfUsbClkin {
   fn af_usb_clkin(&self) -> usize;
}

pub trait AfFtm0Ch2 {
   fn af_ftm0_ch2(&self) -> usize;
}

pub trait AfRmii0Rxer {
   fn af_rmii0_rxer(&self) -> usize;
}

pub trait AfMii0Rxer {
   fn af_mii0_rxer(&self) -> usize;
}

pub trait AfCmp2Out {
   fn af_cmp2_out(&self) -> usize;
}

pub trait AfI2s0TxBclk {
   fn af_i2s0_tx_bclk(&self) -> usize;
}

pub trait AfJtagTrstB {
   fn af_jtag_trst_b(&self) -> usize;
}

pub trait AfPta6 {
   fn af_pta6(&self) -> usize;
}

pub trait AfFtm0Ch3 {
   fn af_ftm0_ch3(&self) -> usize;
}

pub trait AfClkout {
   fn af_clkout(&self) -> usize;
}

pub trait AfTraceClkout {
   fn af_trace_clkout(&self) -> usize;
}

pub trait AfAdc0Se10 {
   fn af_adc0_se10(&self) -> usize;
}

pub trait AfPta7 {
   fn af_pta7(&self) -> usize;
}

pub trait AfFtm0Ch4 {
   fn af_ftm0_ch4(&self) -> usize;
}

pub trait AfTraceD3 {
   fn af_trace_d3(&self) -> usize;
}

pub trait AfAdc0Se11 {
   fn af_adc0_se11(&self) -> usize;
}

pub trait AfPta8 {
   fn af_pta8(&self) -> usize;
}

pub trait AfFtm1Ch0 {
   fn af_ftm1_ch0(&self) -> usize;
}

pub trait AfFtm1QdPha {
   fn af_ftm1_qd_pha(&self) -> usize;
}

pub trait AfTraceD2 {
   fn af_trace_d2(&self) -> usize;
}

pub trait AfPta9 {
   fn af_pta9(&self) -> usize;
}

pub trait AfFtm1Ch1 {
   fn af_ftm1_ch1(&self) -> usize;
}

pub trait AfMii0Rxd3 {
   fn af_mii0_rxd3(&self) -> usize;
}

pub trait AfFtm1QdPhb {
   fn af_ftm1_qd_phb(&self) -> usize;
}

pub trait AfTraceD1 {
   fn af_trace_d1(&self) -> usize;
}

pub trait AfPta10 {
   fn af_pta10(&self) -> usize;
}

pub trait AfFtm2Ch0 {
   fn af_ftm2_ch0(&self) -> usize;
}

pub trait AfMii0Rxd2 {
   fn af_mii0_rxd2(&self) -> usize;
}

pub trait AfFtm2QdPha {
   fn af_ftm2_qd_pha(&self) -> usize;
}

pub trait AfTraceD0 {
   fn af_trace_d0(&self) -> usize;
}

pub trait AfPta11 {
   fn af_pta11(&self) -> usize;
}

pub trait AfFtm2Ch1 {
   fn af_ftm2_ch1(&self) -> usize;
}

pub trait AfMii0Rxclk {
   fn af_mii0_rxclk(&self) -> usize;
}

pub trait AfI2c2Sda {
   fn af_i2c2_sda(&self) -> usize;
}

pub trait AfFtm2QdPhb {
   fn af_ftm2_qd_phb(&self) -> usize;
}

pub trait AfCmp2In0 {
   fn af_cmp2_in0(&self) -> usize;
}

pub trait AfPta12 {
   fn af_pta12(&self) -> usize;
}

pub trait AfCan0Tx {
   fn af_can0_tx(&self) -> usize;
}

pub trait AfRmii0Rxd1 {
   fn af_rmii0_rxd1(&self) -> usize;
}

pub trait AfMii0Rxd1 {
   fn af_mii0_rxd1(&self) -> usize;
}

pub trait AfI2c2Scl {
   fn af_i2c2_scl(&self) -> usize;
}

pub trait AfI2s0Txd0 {
   fn af_i2s0_txd0(&self) -> usize;
}

pub trait AfCmp2In1 {
   fn af_cmp2_in1(&self) -> usize;
}

pub trait AfPta13 {
   fn af_pta13(&self) -> usize;
}

pub trait AfCan0Rx {
   fn af_can0_rx(&self) -> usize;
}

pub trait AfRmii0Rxd0 {
   fn af_rmii0_rxd0(&self) -> usize;
}

pub trait AfMii0Rxd0 {
   fn af_mii0_rxd0(&self) -> usize;
}

pub trait AfI2s0TxFs {
   fn af_i2s0_tx_fs(&self) -> usize;
}

pub trait AfPta14 {
   fn af_pta14(&self) -> usize;
}

pub trait AfSpi0Pcs0 {
   fn af_spi0_pcs0(&self) -> usize;
}

pub trait AfRmii0CrsDv {
   fn af_rmii0_crs_dv(&self) -> usize;
}

pub trait AfMii0Rxdv {
   fn af_mii0_rxdv(&self) -> usize;
}

pub trait AfI2s0RxBclk {
   fn af_i2s0_rx_bclk(&self) -> usize;
}

pub trait AfI2s0Txd1 {
   fn af_i2s0_txd1(&self) -> usize;
}

pub trait AfPta15 {
   fn af_pta15(&self) -> usize;
}

pub trait AfSpi0Sck {
   fn af_spi0_sck(&self) -> usize;
}

pub trait AfRmii0Txen {
   fn af_rmii0_txen(&self) -> usize;
}

pub trait AfMii0Txen {
   fn af_mii0_txen(&self) -> usize;
}

pub trait AfI2s0Rxd0 {
   fn af_i2s0_rxd0(&self) -> usize;
}

pub trait AfPta16 {
   fn af_pta16(&self) -> usize;
}

pub trait AfSpi0Sout {
   fn af_spi0_sout(&self) -> usize;
}

pub trait AfRmii0Txd0 {
   fn af_rmii0_txd0(&self) -> usize;
}

pub trait AfMii0Txd0 {
   fn af_mii0_txd0(&self) -> usize;
}

pub trait AfI2s0RxFs {
   fn af_i2s0_rx_fs(&self) -> usize;
}

pub trait AfI2s0Rxd1 {
   fn af_i2s0_rxd1(&self) -> usize;
}

pub trait AfAdc1Se17 {
   fn af_adc1_se17(&self) -> usize;
}

pub trait AfPta17 {
   fn af_pta17(&self) -> usize;
}

pub trait AfSpi0Sin {
   fn af_spi0_sin(&self) -> usize;
}

pub trait AfRmii0Txd1 {
   fn af_rmii0_txd1(&self) -> usize;
}

pub trait AfMii0Txd1 {
   fn af_mii0_txd1(&self) -> usize;
}

pub trait AfI2s0Mclk {
   fn af_i2s0_mclk(&self) -> usize;
}

pub trait AfExtal0 {
   fn af_extal0(&self) -> usize;
}

pub trait AfPta18 {
   fn af_pta18(&self) -> usize;
}

pub trait AfFtm0Flt2 {
   fn af_ftm0_flt2(&self) -> usize;
}

pub trait AfFtmClkin0 {
   fn af_ftm_clkin0(&self) -> usize;
}

pub trait AfXtal0 {
   fn af_xtal0(&self) -> usize;
}

pub trait AfPta19 {
   fn af_pta19(&self) -> usize;
}

pub trait AfFtm1Flt0 {
   fn af_ftm1_flt0(&self) -> usize;
}

pub trait AfFtmClkin1 {
   fn af_ftm_clkin1(&self) -> usize;
}

pub trait AfLptmr0Alt1 {
   fn af_lptmr0_alt1(&self) -> usize;
}

pub trait AfPta24 {
   fn af_pta24(&self) -> usize;
}

pub trait AfMii0Txd2 {
   fn af_mii0_txd2(&self) -> usize;
}

pub trait AfFbA29 {
   fn af_fb_a29(&self) -> usize;
}

pub trait AfPta25 {
   fn af_pta25(&self) -> usize;
}

pub trait AfMii0Txclk {
   fn af_mii0_txclk(&self) -> usize;
}

pub trait AfFbA28 {
   fn af_fb_a28(&self) -> usize;
}

pub trait AfPta26 {
   fn af_pta26(&self) -> usize;
}

pub trait AfMii0Txd3 {
   fn af_mii0_txd3(&self) -> usize;
}

pub trait AfFbA27 {
   fn af_fb_a27(&self) -> usize;
}

pub trait AfPta27 {
   fn af_pta27(&self) -> usize;
}

pub trait AfMii0Crs {
   fn af_mii0_crs(&self) -> usize;
}

pub trait AfFbA26 {
   fn af_fb_a26(&self) -> usize;
}

pub trait AfPta28 {
   fn af_pta28(&self) -> usize;
}

pub trait AfMii0Txer {
   fn af_mii0_txer(&self) -> usize;
}

pub trait AfFbA25 {
   fn af_fb_a25(&self) -> usize;
}

pub trait AfPta29 {
   fn af_pta29(&self) -> usize;
}

pub trait AfMii0Col {
   fn af_mii0_col(&self) -> usize;
}

pub trait AfFbA24 {
   fn af_fb_a24(&self) -> usize;
}

pub trait AfAdc0Se8 {
   fn af_adc0_se8(&self) -> usize;
}

pub trait AfAdc1Se8 {
   fn af_adc1_se8(&self) -> usize;
}

pub trait AfPtb0 {
   fn af_ptb0(&self) -> usize;
}

pub trait AfI2c0Scl {
   fn af_i2c0_scl(&self) -> usize;
}

pub trait AfRmii0Mdio {
   fn af_rmii0_mdio(&self) -> usize;
}

pub trait AfMii0Mdio {
   fn af_mii0_mdio(&self) -> usize;
}

pub trait AfAdc0Se9 {
   fn af_adc0_se9(&self) -> usize;
}

pub trait AfAdc1Se9 {
   fn af_adc1_se9(&self) -> usize;
}

pub trait AfPtb1 {
   fn af_ptb1(&self) -> usize;
}

pub trait AfI2c0Sda {
   fn af_i2c0_sda(&self) -> usize;
}

pub trait AfRmii0Mdc {
   fn af_rmii0_mdc(&self) -> usize;
}

pub trait AfMii0Mdc {
   fn af_mii0_mdc(&self) -> usize;
}

pub trait AfAdc0Se12 {
   fn af_adc0_se12(&self) -> usize;
}

pub trait AfPtb2 {
   fn af_ptb2(&self) -> usize;
}

pub trait AfEnet01588Tmr0 {
   fn af_enet0_1588_tmr0(&self) -> usize;
}

pub trait AfFtm0Flt3 {
   fn af_ftm0_flt3(&self) -> usize;
}

pub trait AfAdc0Se13 {
   fn af_adc0_se13(&self) -> usize;
}

pub trait AfPtb3 {
   fn af_ptb3(&self) -> usize;
}

pub trait AfEnet01588Tmr1 {
   fn af_enet0_1588_tmr1(&self) -> usize;
}

pub trait AfFtm0Flt0 {
   fn af_ftm0_flt0(&self) -> usize;
}

pub trait AfAdc1Se10 {
   fn af_adc1_se10(&self) -> usize;
}

pub trait AfPtb4 {
   fn af_ptb4(&self) -> usize;
}

pub trait AfEnet01588Tmr2 {
   fn af_enet0_1588_tmr2(&self) -> usize;
}

pub trait AfAdc1Se11 {
   fn af_adc1_se11(&self) -> usize;
}

pub trait AfPtb5 {
   fn af_ptb5(&self) -> usize;
}

pub trait AfEnet01588Tmr3 {
   fn af_enet0_1588_tmr3(&self) -> usize;
}

pub trait AfFtm2Flt0 {
   fn af_ftm2_flt0(&self) -> usize;
}

pub trait AfAdc1Se12 {
   fn af_adc1_se12(&self) -> usize;
}

pub trait AfPtb6 {
   fn af_ptb6(&self) -> usize;
}

pub trait AfFbAd23 {
   fn af_fb_ad23(&self) -> usize;
}

pub trait AfAdc1Se13 {
   fn af_adc1_se13(&self) -> usize;
}

pub trait AfPtb7 {
   fn af_ptb7(&self) -> usize;
}

pub trait AfFbAd22 {
   fn af_fb_ad22(&self) -> usize;
}

pub trait AfPtb8 {
   fn af_ptb8(&self) -> usize;
}

pub trait AfUart3RtsB {
   fn af_uart3_rts_b(&self) -> usize;
}

pub trait AfFbAd21 {
   fn af_fb_ad21(&self) -> usize;
}

pub trait AfPtb9 {
   fn af_ptb9(&self) -> usize;
}

pub trait AfSpi1Pcs1 {
   fn af_spi1_pcs1(&self) -> usize;
}

pub trait AfUart3CtsB {
   fn af_uart3_cts_b(&self) -> usize;
}

pub trait AfFbAd20 {
   fn af_fb_ad20(&self) -> usize;
}

pub trait AfAdc1Se14 {
   fn af_adc1_se14(&self) -> usize;
}

pub trait AfPtb10 {
   fn af_ptb10(&self) -> usize;
}

pub trait AfSpi1Pcs0 {
   fn af_spi1_pcs0(&self) -> usize;
}

pub trait AfUart3Rx {
   fn af_uart3_rx(&self) -> usize;
}

pub trait AfFbAd19 {
   fn af_fb_ad19(&self) -> usize;
}

pub trait AfFtm0Flt1 {
   fn af_ftm0_flt1(&self) -> usize;
}

pub trait AfAdc1Se15 {
   fn af_adc1_se15(&self) -> usize;
}

pub trait AfPtb11 {
   fn af_ptb11(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfUart3Tx {
   fn af_uart3_tx(&self) -> usize;
}

pub trait AfFbAd18 {
   fn af_fb_ad18(&self) -> usize;
}

pub trait AfPtb12 {
   fn af_ptb12(&self) -> usize;
}

pub trait AfPtb13 {
   fn af_ptb13(&self) -> usize;
}

pub trait AfPtb16 {
   fn af_ptb16(&self) -> usize;
}

pub trait AfSpi1Sout {
   fn af_spi1_sout(&self) -> usize;
}

pub trait AfFbAd17 {
   fn af_fb_ad17(&self) -> usize;
}

pub trait AfEwmIn {
   fn af_ewm_in(&self) -> usize;
}

pub trait AfPtb17 {
   fn af_ptb17(&self) -> usize;
}

pub trait AfSpi1Sin {
   fn af_spi1_sin(&self) -> usize;
}

pub trait AfFbAd16 {
   fn af_fb_ad16(&self) -> usize;
}

pub trait AfEwmOutB {
   fn af_ewm_out_b(&self) -> usize;
}

pub trait AfPtb18 {
   fn af_ptb18(&self) -> usize;
}

pub trait AfFbAd15 {
   fn af_fb_ad15(&self) -> usize;
}

pub trait AfPtb19 {
   fn af_ptb19(&self) -> usize;
}

pub trait AfFbOeB {
   fn af_fb_oe_b(&self) -> usize;
}

pub trait AfPtb20 {
   fn af_ptb20(&self) -> usize;
}

pub trait AfSpi2Pcs0 {
   fn af_spi2_pcs0(&self) -> usize;
}

pub trait AfFbAd31 {
   fn af_fb_ad31(&self) -> usize;
}

pub trait AfCmp0Out {
   fn af_cmp0_out(&self) -> usize;
}

pub trait AfPtb21 {
   fn af_ptb21(&self) -> usize;
}

pub trait AfSpi2Sck {
   fn af_spi2_sck(&self) -> usize;
}

pub trait AfFbAd30 {
   fn af_fb_ad30(&self) -> usize;
}

pub trait AfCmp1Out {
   fn af_cmp1_out(&self) -> usize;
}

pub trait AfPtb22 {
   fn af_ptb22(&self) -> usize;
}

pub trait AfSpi2Sout {
   fn af_spi2_sout(&self) -> usize;
}

pub trait AfFbAd29 {
   fn af_fb_ad29(&self) -> usize;
}

pub trait AfPtb23 {
   fn af_ptb23(&self) -> usize;
}

pub trait AfSpi2Sin {
   fn af_spi2_sin(&self) -> usize;
}

pub trait AfSpi0Pcs5 {
   fn af_spi0_pcs5(&self) -> usize;
}

pub trait AfFbAd28 {
   fn af_fb_ad28(&self) -> usize;
}

pub trait AfAdc0Se14 {
   fn af_adc0_se14(&self) -> usize;
}

pub trait AfPtc0 {
   fn af_ptc0(&self) -> usize;
}

pub trait AfSpi0Pcs4 {
   fn af_spi0_pcs4(&self) -> usize;
}

pub trait AfPdb0Extrg {
   fn af_pdb0_extrg(&self) -> usize;
}

pub trait AfUsbSofOut {
   fn af_usb_sof_out(&self) -> usize;
}

pub trait AfFbAd14 {
   fn af_fb_ad14(&self) -> usize;
}

pub trait AfAdc0Se15 {
   fn af_adc0_se15(&self) -> usize;
}

pub trait AfPtc1 {
   fn af_ptc1(&self) -> usize;
}

pub trait AfSpi0Pcs3 {
   fn af_spi0_pcs3(&self) -> usize;
}

pub trait AfUart1RtsB {
   fn af_uart1_rts_b(&self) -> usize;
}

pub trait AfFbAd13 {
   fn af_fb_ad13(&self) -> usize;
}

pub trait AfAdc0Se4b {
   fn af_adc0_se4b(&self) -> usize;
}

pub trait AfCmp1In0 {
   fn af_cmp1_in0(&self) -> usize;
}

pub trait AfPtc2 {
   fn af_ptc2(&self) -> usize;
}

pub trait AfSpi0Pcs2 {
   fn af_spi0_pcs2(&self) -> usize;
}

pub trait AfUart1CtsB {
   fn af_uart1_cts_b(&self) -> usize;
}

pub trait AfFbAd12 {
   fn af_fb_ad12(&self) -> usize;
}

pub trait AfCmp1In1 {
   fn af_cmp1_in1(&self) -> usize;
}

pub trait AfPtc3 {
   fn af_ptc3(&self) -> usize;
}

pub trait AfSpi0Pcs1 {
   fn af_spi0_pcs1(&self) -> usize;
}

pub trait AfUart1Rx {
   fn af_uart1_rx(&self) -> usize;
}

pub trait AfPtc4 {
   fn af_ptc4(&self) -> usize;
}

pub trait AfUart1Tx {
   fn af_uart1_tx(&self) -> usize;
}

pub trait AfFbAd11 {
   fn af_fb_ad11(&self) -> usize;
}

pub trait AfPtc5 {
   fn af_ptc5(&self) -> usize;
}

pub trait AfLptmr0Alt2 {
   fn af_lptmr0_alt2(&self) -> usize;
}

pub trait AfFbAd10 {
   fn af_fb_ad10(&self) -> usize;
}

pub trait AfCmp0In0 {
   fn af_cmp0_in0(&self) -> usize;
}

pub trait AfPtc6 {
   fn af_ptc6(&self) -> usize;
}

pub trait AfFbAd9 {
   fn af_fb_ad9(&self) -> usize;
}

pub trait AfCmp0In1 {
   fn af_cmp0_in1(&self) -> usize;
}

pub trait AfPtc7 {
   fn af_ptc7(&self) -> usize;
}

pub trait AfFbAd8 {
   fn af_fb_ad8(&self) -> usize;
}

pub trait AfAdc1Se4b {
   fn af_adc1_se4b(&self) -> usize;
}

pub trait AfCmp0In2 {
   fn af_cmp0_in2(&self) -> usize;
}

pub trait AfPtc8 {
   fn af_ptc8(&self) -> usize;
}

pub trait AfFtm3Ch4 {
   fn af_ftm3_ch4(&self) -> usize;
}

pub trait AfFbAd7 {
   fn af_fb_ad7(&self) -> usize;
}

pub trait AfAdc1Se5b {
   fn af_adc1_se5b(&self) -> usize;
}

pub trait AfCmp0In3 {
   fn af_cmp0_in3(&self) -> usize;
}

pub trait AfPtc9 {
   fn af_ptc9(&self) -> usize;
}

pub trait AfFtm3Ch5 {
   fn af_ftm3_ch5(&self) -> usize;
}

pub trait AfFbAd6 {
   fn af_fb_ad6(&self) -> usize;
}

pub trait AfAdc1Se6b {
   fn af_adc1_se6b(&self) -> usize;
}

pub trait AfPtc10 {
   fn af_ptc10(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfFtm3Ch6 {
   fn af_ftm3_ch6(&self) -> usize;
}

pub trait AfFbAd5 {
   fn af_fb_ad5(&self) -> usize;
}

pub trait AfAdc1Se7b {
   fn af_adc1_se7b(&self) -> usize;
}

pub trait AfPtc11 {
   fn af_ptc11(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfFtm3Ch7 {
   fn af_ftm3_ch7(&self) -> usize;
}

pub trait AfFbRwB {
   fn af_fb_rw_b(&self) -> usize;
}

pub trait AfPtc12 {
   fn af_ptc12(&self) -> usize;
}

pub trait AfUart4RtsB {
   fn af_uart4_rts_b(&self) -> usize;
}

pub trait AfFbAd27 {
   fn af_fb_ad27(&self) -> usize;
}

pub trait AfFtm3Flt0 {
   fn af_ftm3_flt0(&self) -> usize;
}

pub trait AfPtc13 {
   fn af_ptc13(&self) -> usize;
}

pub trait AfUart4CtsB {
   fn af_uart4_cts_b(&self) -> usize;
}

pub trait AfFbAd26 {
   fn af_fb_ad26(&self) -> usize;
}

pub trait AfPtc14 {
   fn af_ptc14(&self) -> usize;
}

pub trait AfUart4Rx {
   fn af_uart4_rx(&self) -> usize;
}

pub trait AfFbAd25 {
   fn af_fb_ad25(&self) -> usize;
}

pub trait AfPtc15 {
   fn af_ptc15(&self) -> usize;
}

pub trait AfUart4Tx {
   fn af_uart4_tx(&self) -> usize;
}

pub trait AfFbAd24 {
   fn af_fb_ad24(&self) -> usize;
}

pub trait AfPtc16 {
   fn af_ptc16(&self) -> usize;
}

pub trait AfFbCs5B {
   fn af_fb_cs5_b(&self) -> usize;
}

pub trait AfFbTsiz1 {
   fn af_fb_tsiz1(&self) -> usize;
}

pub trait AfFbBe2316Bls158B {
   fn af_fb_be23_16_bls15_8_b(&self) -> usize;
}

pub trait AfPtc17 {
   fn af_ptc17(&self) -> usize;
}

pub trait AfFbCs4B {
   fn af_fb_cs4_b(&self) -> usize;
}

pub trait AfFbTsiz0 {
   fn af_fb_tsiz0(&self) -> usize;
}

pub trait AfFbBe3124Bls70B {
   fn af_fb_be31_24_bls7_0_b(&self) -> usize;
}

pub trait AfPtc18 {
   fn af_ptc18(&self) -> usize;
}

pub trait AfFbTbstB {
   fn af_fb_tbst_b(&self) -> usize;
}

pub trait AfFbCs2B {
   fn af_fb_cs2_b(&self) -> usize;
}

pub trait AfFbBe158Bls2316B {
   fn af_fb_be15_8_bls23_16_b(&self) -> usize;
}

pub trait AfPtc19 {
   fn af_ptc19(&self) -> usize;
}

pub trait AfFbCs3B {
   fn af_fb_cs3_b(&self) -> usize;
}

pub trait AfFbBe70Bls3124B {
   fn af_fb_be7_0_bls31_24_b(&self) -> usize;
}

pub trait AfFbTaB {
   fn af_fb_ta_b(&self) -> usize;
}

pub trait AfPtd0 {
   fn af_ptd0(&self) -> usize;
}

pub trait AfUart2RtsB {
   fn af_uart2_rts_b(&self) -> usize;
}

pub trait AfFtm3Ch0 {
   fn af_ftm3_ch0(&self) -> usize;
}

pub trait AfFbAle {
   fn af_fb_ale(&self) -> usize;
}

pub trait AfFbCs1B {
   fn af_fb_cs1_b(&self) -> usize;
}

pub trait AfFbTsB {
   fn af_fb_ts_b(&self) -> usize;
}

pub trait AfAdc0Se5b {
   fn af_adc0_se5b(&self) -> usize;
}

pub trait AfPtd1 {
   fn af_ptd1(&self) -> usize;
}

pub trait AfUart2CtsB {
   fn af_uart2_cts_b(&self) -> usize;
}

pub trait AfFtm3Ch1 {
   fn af_ftm3_ch1(&self) -> usize;
}

pub trait AfFbCs0B {
   fn af_fb_cs0_b(&self) -> usize;
}

pub trait AfPtd2 {
   fn af_ptd2(&self) -> usize;
}

pub trait AfUart2Rx {
   fn af_uart2_rx(&self) -> usize;
}

pub trait AfFtm3Ch2 {
   fn af_ftm3_ch2(&self) -> usize;
}

pub trait AfFbAd4 {
   fn af_fb_ad4(&self) -> usize;
}

pub trait AfPtd3 {
   fn af_ptd3(&self) -> usize;
}

pub trait AfUart2Tx {
   fn af_uart2_tx(&self) -> usize;
}

pub trait AfFtm3Ch3 {
   fn af_ftm3_ch3(&self) -> usize;
}

pub trait AfFbAd3 {
   fn af_fb_ad3(&self) -> usize;
}

pub trait AfPtd4 {
   fn af_ptd4(&self) -> usize;
}

pub trait AfFbAd2 {
   fn af_fb_ad2(&self) -> usize;
}

pub trait AfAdc0Se6b {
   fn af_adc0_se6b(&self) -> usize;
}

pub trait AfPtd5 {
   fn af_ptd5(&self) -> usize;
}

pub trait AfFbAd1 {
   fn af_fb_ad1(&self) -> usize;
}

pub trait AfAdc0Se7b {
   fn af_adc0_se7b(&self) -> usize;
}

pub trait AfPtd6 {
   fn af_ptd6(&self) -> usize;
}

pub trait AfFbAd0 {
   fn af_fb_ad0(&self) -> usize;
}

pub trait AfPtd7 {
   fn af_ptd7(&self) -> usize;
}

pub trait AfCmtIro {
   fn af_cmt_iro(&self) -> usize;
}

pub trait AfPtd8 {
   fn af_ptd8(&self) -> usize;
}

pub trait AfUart5Rx {
   fn af_uart5_rx(&self) -> usize;
}

pub trait AfFbA16 {
   fn af_fb_a16(&self) -> usize;
}

pub trait AfPtd9 {
   fn af_ptd9(&self) -> usize;
}

pub trait AfUart5Tx {
   fn af_uart5_tx(&self) -> usize;
}

pub trait AfFbA17 {
   fn af_fb_a17(&self) -> usize;
}

pub trait AfPtd10 {
   fn af_ptd10(&self) -> usize;
}

pub trait AfUart5RtsB {
   fn af_uart5_rts_b(&self) -> usize;
}

pub trait AfFbA18 {
   fn af_fb_a18(&self) -> usize;
}

pub trait AfPtd11 {
   fn af_ptd11(&self) -> usize;
}

pub trait AfUart5CtsB {
   fn af_uart5_cts_b(&self) -> usize;
}

pub trait AfSdhc0Clkin {
   fn af_sdhc0_clkin(&self) -> usize;
}

pub trait AfFbA19 {
   fn af_fb_a19(&self) -> usize;
}

pub trait AfPtd12 {
   fn af_ptd12(&self) -> usize;
}

pub trait AfSdhc0D4 {
   fn af_sdhc0_d4(&self) -> usize;
}

pub trait AfFbA20 {
   fn af_fb_a20(&self) -> usize;
}

pub trait AfPtd13 {
   fn af_ptd13(&self) -> usize;
}

pub trait AfSdhc0D5 {
   fn af_sdhc0_d5(&self) -> usize;
}

pub trait AfFbA21 {
   fn af_fb_a21(&self) -> usize;
}

pub trait AfPtd14 {
   fn af_ptd14(&self) -> usize;
}

pub trait AfSdhc0D6 {
   fn af_sdhc0_d6(&self) -> usize;
}

pub trait AfFbA22 {
   fn af_fb_a22(&self) -> usize;
}

pub trait AfPtd15 {
   fn af_ptd15(&self) -> usize;
}

pub trait AfSpi2Pcs1 {
   fn af_spi2_pcs1(&self) -> usize;
}

pub trait AfSdhc0D7 {
   fn af_sdhc0_d7(&self) -> usize;
}

pub trait AfFbA23 {
   fn af_fb_a23(&self) -> usize;
}

pub trait AfAdc1Se4a {
   fn af_adc1_se4a(&self) -> usize;
}

pub trait AfPte0 {
   fn af_pte0(&self) -> usize;
}

pub trait AfSdhc0D1 {
   fn af_sdhc0_d1(&self) -> usize;
}

pub trait AfRtcClkout {
   fn af_rtc_clkout(&self) -> usize;
}

pub trait AfAdc1Se5a {
   fn af_adc1_se5a(&self) -> usize;
}

pub trait AfPte1 {
   fn af_pte1(&self) -> usize;
}

pub trait AfSdhc0D0 {
   fn af_sdhc0_d0(&self) -> usize;
}

pub trait AfAdc0Dp2 {
   fn af_adc0_dp2(&self) -> usize;
}

pub trait AfAdc1Se6a {
   fn af_adc1_se6a(&self) -> usize;
}

pub trait AfPte2 {
   fn af_pte2(&self) -> usize;
}

pub trait AfSdhc0Dclk {
   fn af_sdhc0_dclk(&self) -> usize;
}

pub trait AfAdc0Dm2 {
   fn af_adc0_dm2(&self) -> usize;
}

pub trait AfAdc1Se7a {
   fn af_adc1_se7a(&self) -> usize;
}

pub trait AfPte3 {
   fn af_pte3(&self) -> usize;
}

pub trait AfSdhc0Cmd {
   fn af_sdhc0_cmd(&self) -> usize;
}

pub trait AfPte4 {
   fn af_pte4(&self) -> usize;
}

pub trait AfSdhc0D3 {
   fn af_sdhc0_d3(&self) -> usize;
}

pub trait AfPte5 {
   fn af_pte5(&self) -> usize;
}

pub trait AfSpi1Pcs2 {
   fn af_spi1_pcs2(&self) -> usize;
}

pub trait AfSdhc0D2 {
   fn af_sdhc0_d2(&self) -> usize;
}

pub trait AfPte6 {
   fn af_pte6(&self) -> usize;
}

pub trait AfSpi1Pcs3 {
   fn af_spi1_pcs3(&self) -> usize;
}

pub trait AfPte7 {
   fn af_pte7(&self) -> usize;
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

pub trait AfPte11 {
   fn af_pte11(&self) -> usize;
}

pub trait AfPte12 {
   fn af_pte12(&self) -> usize;
}

pub trait AfAdc0Se17 {
   fn af_adc0_se17(&self) -> usize;
}

pub trait AfPte24 {
   fn af_pte24(&self) -> usize;
}

pub trait AfAdc0Se18 {
   fn af_adc0_se18(&self) -> usize;
}

pub trait AfPte25 {
   fn af_pte25(&self) -> usize;
}

pub trait AfPte26 {
   fn af_pte26(&self) -> usize;
}

pub trait AfEnet1588Clkin {
   fn af_enet_1588_clkin(&self) -> usize;
}

pub trait AfPte27 {
   fn af_pte27(&self) -> usize;
}

pub trait AfPte28 {
   fn af_pte28(&self) -> usize;
}

pub const PTA0: Pta0 = Pta0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta0 {}

impl Pin for Pta0 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 0 }
}

impl AfPta0 for Pta0 {
   fn af_pta0(&self) -> usize { 1 }
}

impl AfUart0CtsB for Pta0 {
   fn af_uart0_cts_b(&self) -> usize { 2 }
}

impl AfUart0ColB for Pta0 {
   fn af_uart0_col_b(&self) -> usize { 2 }
}

impl AfFtm0Ch5 for Pta0 {
   fn af_ftm0_ch5(&self) -> usize { 3 }
}

impl AfJtagTclk for Pta0 {
   fn af_jtag_tclk(&self) -> usize { 7 }
}

impl AfSwdClk for Pta0 {
   fn af_swd_clk(&self) -> usize { 7 }
}

pub const PTA1: Pta1 = Pta1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta1 {}

impl Pin for Pta1 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 1 }
}

impl AfPta1 for Pta1 {
   fn af_pta1(&self) -> usize { 1 }
}

impl AfUart0Rx for Pta1 {
   fn af_uart0_rx(&self) -> usize { 2 }
}

impl AfFtm0Ch6 for Pta1 {
   fn af_ftm0_ch6(&self) -> usize { 3 }
}

impl AfJtagTdi for Pta1 {
   fn af_jtag_tdi(&self) -> usize { 7 }
}

pub const PTA2: Pta2 = Pta2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta2 {}

impl Pin for Pta2 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 2 }
}

impl AfPta2 for Pta2 {
   fn af_pta2(&self) -> usize { 1 }
}

impl AfUart0Tx for Pta2 {
   fn af_uart0_tx(&self) -> usize { 2 }
}

impl AfFtm0Ch7 for Pta2 {
   fn af_ftm0_ch7(&self) -> usize { 3 }
}

impl AfJtagTdo for Pta2 {
   fn af_jtag_tdo(&self) -> usize { 7 }
}

impl AfTraceSwo for Pta2 {
   fn af_trace_swo(&self) -> usize { 7 }
}

pub const PTA3: Pta3 = Pta3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta3 {}

impl Pin for Pta3 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 3 }
}

impl AfPta3 for Pta3 {
   fn af_pta3(&self) -> usize { 1 }
}

impl AfUart0RtsB for Pta3 {
   fn af_uart0_rts_b(&self) -> usize { 2 }
}

impl AfFtm0Ch0 for Pta3 {
   fn af_ftm0_ch0(&self) -> usize { 3 }
}

impl AfJtagTms for Pta3 {
   fn af_jtag_tms(&self) -> usize { 7 }
}

impl AfSwdDio for Pta3 {
   fn af_swd_dio(&self) -> usize { 7 }
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

impl AfFtm0Ch1 for Pta4 {
   fn af_ftm0_ch1(&self) -> usize { 3 }
}

impl AfNmiB for Pta4 {
   fn af_nmi_b(&self) -> usize { 7 }
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

impl AfUsbClkin for Pta5 {
   fn af_usb_clkin(&self) -> usize { 2 }
}

impl AfFtm0Ch2 for Pta5 {
   fn af_ftm0_ch2(&self) -> usize { 3 }
}

impl AfRmii0Rxer for Pta5 {
   fn af_rmii0_rxer(&self) -> usize { 4 }
}

impl AfMii0Rxer for Pta5 {
   fn af_mii0_rxer(&self) -> usize { 4 }
}

impl AfCmp2Out for Pta5 {
   fn af_cmp2_out(&self) -> usize { 5 }
}

impl AfI2s0TxBclk for Pta5 {
   fn af_i2s0_tx_bclk(&self) -> usize { 6 }
}

impl AfJtagTrstB for Pta5 {
   fn af_jtag_trst_b(&self) -> usize { 7 }
}

pub const PTA6: Pta6 = Pta6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta6 {}

impl Pin for Pta6 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 6 }
}

impl AfPta6 for Pta6 {
   fn af_pta6(&self) -> usize { 1 }
}

impl AfFtm0Ch3 for Pta6 {
   fn af_ftm0_ch3(&self) -> usize { 3 }
}

impl AfClkout for Pta6 {
   fn af_clkout(&self) -> usize { 5 }
}

impl AfTraceClkout for Pta6 {
   fn af_trace_clkout(&self) -> usize { 7 }
}

pub const PTA7: Pta7 = Pta7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta7 {}

impl Pin for Pta7 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 7 }
}

impl AfAdc0Se10 for Pta7 {
   fn af_adc0_se10(&self) -> usize { 0 }
}

impl AfPta7 for Pta7 {
   fn af_pta7(&self) -> usize { 1 }
}

impl AfFtm0Ch4 for Pta7 {
   fn af_ftm0_ch4(&self) -> usize { 3 }
}

impl AfTraceD3 for Pta7 {
   fn af_trace_d3(&self) -> usize { 7 }
}

pub const PTA8: Pta8 = Pta8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta8 {}

impl Pin for Pta8 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 8 }
}

impl AfAdc0Se11 for Pta8 {
   fn af_adc0_se11(&self) -> usize { 0 }
}

impl AfPta8 for Pta8 {
   fn af_pta8(&self) -> usize { 1 }
}

impl AfFtm1Ch0 for Pta8 {
   fn af_ftm1_ch0(&self) -> usize { 3 }
}

impl AfFtm1QdPha for Pta8 {
   fn af_ftm1_qd_pha(&self) -> usize { 6 }
}

impl AfTraceD2 for Pta8 {
   fn af_trace_d2(&self) -> usize { 7 }
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

impl AfFtm1Ch1 for Pta9 {
   fn af_ftm1_ch1(&self) -> usize { 3 }
}

impl AfMii0Rxd3 for Pta9 {
   fn af_mii0_rxd3(&self) -> usize { 4 }
}

impl AfFtm1QdPhb for Pta9 {
   fn af_ftm1_qd_phb(&self) -> usize { 6 }
}

impl AfTraceD1 for Pta9 {
   fn af_trace_d1(&self) -> usize { 7 }
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

impl AfFtm2Ch0 for Pta10 {
   fn af_ftm2_ch0(&self) -> usize { 3 }
}

impl AfMii0Rxd2 for Pta10 {
   fn af_mii0_rxd2(&self) -> usize { 4 }
}

impl AfFtm2QdPha for Pta10 {
   fn af_ftm2_qd_pha(&self) -> usize { 6 }
}

impl AfTraceD0 for Pta10 {
   fn af_trace_d0(&self) -> usize { 7 }
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

impl AfFtm2Ch1 for Pta11 {
   fn af_ftm2_ch1(&self) -> usize { 3 }
}

impl AfMii0Rxclk for Pta11 {
   fn af_mii0_rxclk(&self) -> usize { 4 }
}

impl AfI2c2Sda for Pta11 {
   fn af_i2c2_sda(&self) -> usize { 5 }
}

impl AfFtm2QdPhb for Pta11 {
   fn af_ftm2_qd_phb(&self) -> usize { 6 }
}

pub const PTA12: Pta12 = Pta12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta12 {}

impl Pin for Pta12 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 12 }
}

impl AfCmp2In0 for Pta12 {
   fn af_cmp2_in0(&self) -> usize { 0 }
}

impl AfPta12 for Pta12 {
   fn af_pta12(&self) -> usize { 1 }
}

impl AfCan0Tx for Pta12 {
   fn af_can0_tx(&self) -> usize { 2 }
}

impl AfFtm1Ch0 for Pta12 {
   fn af_ftm1_ch0(&self) -> usize { 3 }
}

impl AfRmii0Rxd1 for Pta12 {
   fn af_rmii0_rxd1(&self) -> usize { 4 }
}

impl AfMii0Rxd1 for Pta12 {
   fn af_mii0_rxd1(&self) -> usize { 4 }
}

impl AfI2c2Scl for Pta12 {
   fn af_i2c2_scl(&self) -> usize { 5 }
}

impl AfI2s0Txd0 for Pta12 {
   fn af_i2s0_txd0(&self) -> usize { 6 }
}

impl AfFtm1QdPha for Pta12 {
   fn af_ftm1_qd_pha(&self) -> usize { 7 }
}

pub const PTA13: Pta13 = Pta13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta13 {}

impl Pin for Pta13 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 13 }
}

impl AfCmp2In1 for Pta13 {
   fn af_cmp2_in1(&self) -> usize { 0 }
}

impl AfPta13 for Pta13 {
   fn af_pta13(&self) -> usize { 1 }
}

impl AfCan0Rx for Pta13 {
   fn af_can0_rx(&self) -> usize { 2 }
}

impl AfFtm1Ch1 for Pta13 {
   fn af_ftm1_ch1(&self) -> usize { 3 }
}

impl AfRmii0Rxd0 for Pta13 {
   fn af_rmii0_rxd0(&self) -> usize { 4 }
}

impl AfMii0Rxd0 for Pta13 {
   fn af_mii0_rxd0(&self) -> usize { 4 }
}

impl AfI2c2Sda for Pta13 {
   fn af_i2c2_sda(&self) -> usize { 5 }
}

impl AfI2s0TxFs for Pta13 {
   fn af_i2s0_tx_fs(&self) -> usize { 6 }
}

impl AfFtm1QdPhb for Pta13 {
   fn af_ftm1_qd_phb(&self) -> usize { 7 }
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

impl AfSpi0Pcs0 for Pta14 {
   fn af_spi0_pcs0(&self) -> usize { 2 }
}

impl AfUart0Tx for Pta14 {
   fn af_uart0_tx(&self) -> usize { 3 }
}

impl AfRmii0CrsDv for Pta14 {
   fn af_rmii0_crs_dv(&self) -> usize { 4 }
}

impl AfMii0Rxdv for Pta14 {
   fn af_mii0_rxdv(&self) -> usize { 4 }
}

impl AfI2c2Scl for Pta14 {
   fn af_i2c2_scl(&self) -> usize { 5 }
}

impl AfI2s0RxBclk for Pta14 {
   fn af_i2s0_rx_bclk(&self) -> usize { 6 }
}

impl AfI2s0Txd1 for Pta14 {
   fn af_i2s0_txd1(&self) -> usize { 7 }
}

pub const PTA15: Pta15 = Pta15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta15 {}

impl Pin for Pta15 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 15 }
}

impl AfPta15 for Pta15 {
   fn af_pta15(&self) -> usize { 1 }
}

impl AfSpi0Sck for Pta15 {
   fn af_spi0_sck(&self) -> usize { 2 }
}

impl AfUart0Rx for Pta15 {
   fn af_uart0_rx(&self) -> usize { 3 }
}

impl AfRmii0Txen for Pta15 {
   fn af_rmii0_txen(&self) -> usize { 4 }
}

impl AfMii0Txen for Pta15 {
   fn af_mii0_txen(&self) -> usize { 4 }
}

impl AfI2s0Rxd0 for Pta15 {
   fn af_i2s0_rxd0(&self) -> usize { 6 }
}

pub const PTA16: Pta16 = Pta16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta16 {}

impl Pin for Pta16 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 16 }
}

impl AfPta16 for Pta16 {
   fn af_pta16(&self) -> usize { 1 }
}

impl AfSpi0Sout for Pta16 {
   fn af_spi0_sout(&self) -> usize { 2 }
}

impl AfUart0CtsB for Pta16 {
   fn af_uart0_cts_b(&self) -> usize { 3 }
}

impl AfUart0ColB for Pta16 {
   fn af_uart0_col_b(&self) -> usize { 3 }
}

impl AfRmii0Txd0 for Pta16 {
   fn af_rmii0_txd0(&self) -> usize { 4 }
}

impl AfMii0Txd0 for Pta16 {
   fn af_mii0_txd0(&self) -> usize { 4 }
}

impl AfI2s0RxFs for Pta16 {
   fn af_i2s0_rx_fs(&self) -> usize { 6 }
}

impl AfI2s0Rxd1 for Pta16 {
   fn af_i2s0_rxd1(&self) -> usize { 7 }
}

pub const PTA17: Pta17 = Pta17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta17 {}

impl Pin for Pta17 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 17 }
}

impl AfAdc1Se17 for Pta17 {
   fn af_adc1_se17(&self) -> usize { 0 }
}

impl AfPta17 for Pta17 {
   fn af_pta17(&self) -> usize { 1 }
}

impl AfSpi0Sin for Pta17 {
   fn af_spi0_sin(&self) -> usize { 2 }
}

impl AfUart0RtsB for Pta17 {
   fn af_uart0_rts_b(&self) -> usize { 3 }
}

impl AfRmii0Txd1 for Pta17 {
   fn af_rmii0_txd1(&self) -> usize { 4 }
}

impl AfMii0Txd1 for Pta17 {
   fn af_mii0_txd1(&self) -> usize { 4 }
}

impl AfI2s0Mclk for Pta17 {
   fn af_i2s0_mclk(&self) -> usize { 6 }
}

pub const PTA18: Pta18 = Pta18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta18 {}

impl Pin for Pta18 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 18 }
}

impl AfExtal0 for Pta18 {
   fn af_extal0(&self) -> usize { 0 }
}

impl AfPta18 for Pta18 {
   fn af_pta18(&self) -> usize { 1 }
}

impl AfFtm0Flt2 for Pta18 {
   fn af_ftm0_flt2(&self) -> usize { 3 }
}

impl AfFtmClkin0 for Pta18 {
   fn af_ftm_clkin0(&self) -> usize { 4 }
}

pub const PTA19: Pta19 = Pta19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta19 {}

impl Pin for Pta19 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 19 }
}

impl AfXtal0 for Pta19 {
   fn af_xtal0(&self) -> usize { 0 }
}

impl AfPta19 for Pta19 {
   fn af_pta19(&self) -> usize { 1 }
}

impl AfFtm1Flt0 for Pta19 {
   fn af_ftm1_flt0(&self) -> usize { 3 }
}

impl AfFtmClkin1 for Pta19 {
   fn af_ftm_clkin1(&self) -> usize { 4 }
}

impl AfLptmr0Alt1 for Pta19 {
   fn af_lptmr0_alt1(&self) -> usize { 6 }
}

pub const PTA24: Pta24 = Pta24 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta24 {}

impl Pin for Pta24 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 24 }
}

impl AfPta24 for Pta24 {
   fn af_pta24(&self) -> usize { 1 }
}

impl AfMii0Txd2 for Pta24 {
   fn af_mii0_txd2(&self) -> usize { 4 }
}

impl AfFbA29 for Pta24 {
   fn af_fb_a29(&self) -> usize { 6 }
}

pub const PTA25: Pta25 = Pta25 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta25 {}

impl Pin for Pta25 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 25 }
}

impl AfPta25 for Pta25 {
   fn af_pta25(&self) -> usize { 1 }
}

impl AfMii0Txclk for Pta25 {
   fn af_mii0_txclk(&self) -> usize { 4 }
}

impl AfFbA28 for Pta25 {
   fn af_fb_a28(&self) -> usize { 6 }
}

pub const PTA26: Pta26 = Pta26 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta26 {}

impl Pin for Pta26 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 26 }
}

impl AfPta26 for Pta26 {
   fn af_pta26(&self) -> usize { 1 }
}

impl AfMii0Txd3 for Pta26 {
   fn af_mii0_txd3(&self) -> usize { 4 }
}

impl AfFbA27 for Pta26 {
   fn af_fb_a27(&self) -> usize { 6 }
}

pub const PTA27: Pta27 = Pta27 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta27 {}

impl Pin for Pta27 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 27 }
}

impl AfPta27 for Pta27 {
   fn af_pta27(&self) -> usize { 1 }
}

impl AfMii0Crs for Pta27 {
   fn af_mii0_crs(&self) -> usize { 4 }
}

impl AfFbA26 for Pta27 {
   fn af_fb_a26(&self) -> usize { 6 }
}

pub const PTA28: Pta28 = Pta28 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta28 {}

impl Pin for Pta28 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 28 }
}

impl AfPta28 for Pta28 {
   fn af_pta28(&self) -> usize { 1 }
}

impl AfMii0Txer for Pta28 {
   fn af_mii0_txer(&self) -> usize { 4 }
}

impl AfFbA25 for Pta28 {
   fn af_fb_a25(&self) -> usize { 6 }
}

pub const PTA29: Pta29 = Pta29 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pta29 {}

impl Pin for Pta29 {
   fn port(&self) -> Port { PORTA }
   fn index(&self) -> usize { 29 }
}

impl AfPta29 for Pta29 {
   fn af_pta29(&self) -> usize { 1 }
}

impl AfMii0Col for Pta29 {
   fn af_mii0_col(&self) -> usize { 4 }
}

impl AfFbA24 for Pta29 {
   fn af_fb_a24(&self) -> usize { 6 }
}

pub const PTB0: Ptb0 = Ptb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb0 {}

impl Pin for Ptb0 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se8 for Ptb0 {
   fn af_adc0_se8(&self) -> usize { 0 }
}

impl AfAdc1Se8 for Ptb0 {
   fn af_adc1_se8(&self) -> usize { 0 }
}

impl AfPtb0 for Ptb0 {
   fn af_ptb0(&self) -> usize { 1 }
}

impl AfI2c0Scl for Ptb0 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfFtm1Ch0 for Ptb0 {
   fn af_ftm1_ch0(&self) -> usize { 3 }
}

impl AfRmii0Mdio for Ptb0 {
   fn af_rmii0_mdio(&self) -> usize { 4 }
}

impl AfMii0Mdio for Ptb0 {
   fn af_mii0_mdio(&self) -> usize { 4 }
}

impl AfFtm1QdPha for Ptb0 {
   fn af_ftm1_qd_pha(&self) -> usize { 6 }
}

pub const PTB1: Ptb1 = Ptb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb1 {}

impl Pin for Ptb1 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se9 for Ptb1 {
   fn af_adc0_se9(&self) -> usize { 0 }
}

impl AfAdc1Se9 for Ptb1 {
   fn af_adc1_se9(&self) -> usize { 0 }
}

impl AfPtb1 for Ptb1 {
   fn af_ptb1(&self) -> usize { 1 }
}

impl AfI2c0Sda for Ptb1 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfFtm1Ch1 for Ptb1 {
   fn af_ftm1_ch1(&self) -> usize { 3 }
}

impl AfRmii0Mdc for Ptb1 {
   fn af_rmii0_mdc(&self) -> usize { 4 }
}

impl AfMii0Mdc for Ptb1 {
   fn af_mii0_mdc(&self) -> usize { 4 }
}

impl AfFtm1QdPhb for Ptb1 {
   fn af_ftm1_qd_phb(&self) -> usize { 6 }
}

pub const PTB2: Ptb2 = Ptb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb2 {}

impl Pin for Ptb2 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se12 for Ptb2 {
   fn af_adc0_se12(&self) -> usize { 0 }
}

impl AfPtb2 for Ptb2 {
   fn af_ptb2(&self) -> usize { 1 }
}

impl AfI2c0Scl for Ptb2 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfUart0RtsB for Ptb2 {
   fn af_uart0_rts_b(&self) -> usize { 3 }
}

impl AfEnet01588Tmr0 for Ptb2 {
   fn af_enet0_1588_tmr0(&self) -> usize { 4 }
}

impl AfFtm0Flt3 for Ptb2 {
   fn af_ftm0_flt3(&self) -> usize { 6 }
}

pub const PTB3: Ptb3 = Ptb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb3 {}

impl Pin for Ptb3 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 3 }
}

impl AfAdc0Se13 for Ptb3 {
   fn af_adc0_se13(&self) -> usize { 0 }
}

impl AfPtb3 for Ptb3 {
   fn af_ptb3(&self) -> usize { 1 }
}

impl AfI2c0Sda for Ptb3 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfUart0CtsB for Ptb3 {
   fn af_uart0_cts_b(&self) -> usize { 3 }
}

impl AfUart0ColB for Ptb3 {
   fn af_uart0_col_b(&self) -> usize { 3 }
}

impl AfEnet01588Tmr1 for Ptb3 {
   fn af_enet0_1588_tmr1(&self) -> usize { 4 }
}

impl AfFtm0Flt0 for Ptb3 {
   fn af_ftm0_flt0(&self) -> usize { 6 }
}

pub const PTB4: Ptb4 = Ptb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb4 {}

impl Pin for Ptb4 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 4 }
}

impl AfAdc1Se10 for Ptb4 {
   fn af_adc1_se10(&self) -> usize { 0 }
}

impl AfPtb4 for Ptb4 {
   fn af_ptb4(&self) -> usize { 1 }
}

impl AfEnet01588Tmr2 for Ptb4 {
   fn af_enet0_1588_tmr2(&self) -> usize { 4 }
}

impl AfFtm1Flt0 for Ptb4 {
   fn af_ftm1_flt0(&self) -> usize { 6 }
}

pub const PTB5: Ptb5 = Ptb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb5 {}

impl Pin for Ptb5 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 5 }
}

impl AfAdc1Se11 for Ptb5 {
   fn af_adc1_se11(&self) -> usize { 0 }
}

impl AfPtb5 for Ptb5 {
   fn af_ptb5(&self) -> usize { 1 }
}

impl AfEnet01588Tmr3 for Ptb5 {
   fn af_enet0_1588_tmr3(&self) -> usize { 4 }
}

impl AfFtm2Flt0 for Ptb5 {
   fn af_ftm2_flt0(&self) -> usize { 6 }
}

pub const PTB6: Ptb6 = Ptb6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb6 {}

impl Pin for Ptb6 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 6 }
}

impl AfAdc1Se12 for Ptb6 {
   fn af_adc1_se12(&self) -> usize { 0 }
}

impl AfPtb6 for Ptb6 {
   fn af_ptb6(&self) -> usize { 1 }
}

impl AfFbAd23 for Ptb6 {
   fn af_fb_ad23(&self) -> usize { 5 }
}

pub const PTB7: Ptb7 = Ptb7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb7 {}

impl Pin for Ptb7 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 7 }
}

impl AfAdc1Se13 for Ptb7 {
   fn af_adc1_se13(&self) -> usize { 0 }
}

impl AfPtb7 for Ptb7 {
   fn af_ptb7(&self) -> usize { 1 }
}

impl AfFbAd22 for Ptb7 {
   fn af_fb_ad22(&self) -> usize { 5 }
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

impl AfUart3RtsB for Ptb8 {
   fn af_uart3_rts_b(&self) -> usize { 3 }
}

impl AfFbAd21 for Ptb8 {
   fn af_fb_ad21(&self) -> usize { 5 }
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

impl AfSpi1Pcs1 for Ptb9 {
   fn af_spi1_pcs1(&self) -> usize { 2 }
}

impl AfUart3CtsB for Ptb9 {
   fn af_uart3_cts_b(&self) -> usize { 3 }
}

impl AfFbAd20 for Ptb9 {
   fn af_fb_ad20(&self) -> usize { 5 }
}

pub const PTB10: Ptb10 = Ptb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb10 {}

impl Pin for Ptb10 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 10 }
}

impl AfAdc1Se14 for Ptb10 {
   fn af_adc1_se14(&self) -> usize { 0 }
}

impl AfPtb10 for Ptb10 {
   fn af_ptb10(&self) -> usize { 1 }
}

impl AfSpi1Pcs0 for Ptb10 {
   fn af_spi1_pcs0(&self) -> usize { 2 }
}

impl AfUart3Rx for Ptb10 {
   fn af_uart3_rx(&self) -> usize { 3 }
}

impl AfFbAd19 for Ptb10 {
   fn af_fb_ad19(&self) -> usize { 5 }
}

impl AfFtm0Flt1 for Ptb10 {
   fn af_ftm0_flt1(&self) -> usize { 6 }
}

pub const PTB11: Ptb11 = Ptb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb11 {}

impl Pin for Ptb11 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 11 }
}

impl AfAdc1Se15 for Ptb11 {
   fn af_adc1_se15(&self) -> usize { 0 }
}

impl AfPtb11 for Ptb11 {
   fn af_ptb11(&self) -> usize { 1 }
}

impl AfSpi1Sck for Ptb11 {
   fn af_spi1_sck(&self) -> usize { 2 }
}

impl AfUart3Tx for Ptb11 {
   fn af_uart3_tx(&self) -> usize { 3 }
}

impl AfFbAd18 for Ptb11 {
   fn af_fb_ad18(&self) -> usize { 5 }
}

impl AfFtm0Flt2 for Ptb11 {
   fn af_ftm0_flt2(&self) -> usize { 6 }
}

pub const PTB12: Ptb12 = Ptb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb12 {}

impl Pin for Ptb12 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 12 }
}

impl AfPtb12 for Ptb12 {
   fn af_ptb12(&self) -> usize { 1 }
}

impl AfUart3RtsB for Ptb12 {
   fn af_uart3_rts_b(&self) -> usize { 2 }
}

impl AfFtm1Ch0 for Ptb12 {
   fn af_ftm1_ch0(&self) -> usize { 3 }
}

impl AfFtm0Ch4 for Ptb12 {
   fn af_ftm0_ch4(&self) -> usize { 4 }
}

impl AfFtm1QdPha for Ptb12 {
   fn af_ftm1_qd_pha(&self) -> usize { 6 }
}

pub const PTB13: Ptb13 = Ptb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb13 {}

impl Pin for Ptb13 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 13 }
}

impl AfPtb13 for Ptb13 {
   fn af_ptb13(&self) -> usize { 1 }
}

impl AfUart3CtsB for Ptb13 {
   fn af_uart3_cts_b(&self) -> usize { 2 }
}

impl AfFtm1Ch1 for Ptb13 {
   fn af_ftm1_ch1(&self) -> usize { 3 }
}

impl AfFtm0Ch5 for Ptb13 {
   fn af_ftm0_ch5(&self) -> usize { 4 }
}

impl AfFtm1QdPhb for Ptb13 {
   fn af_ftm1_qd_phb(&self) -> usize { 6 }
}

pub const PTB16: Ptb16 = Ptb16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb16 {}

impl Pin for Ptb16 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 16 }
}

impl AfPtb16 for Ptb16 {
   fn af_ptb16(&self) -> usize { 1 }
}

impl AfSpi1Sout for Ptb16 {
   fn af_spi1_sout(&self) -> usize { 2 }
}

impl AfUart0Rx for Ptb16 {
   fn af_uart0_rx(&self) -> usize { 3 }
}

impl AfFtmClkin0 for Ptb16 {
   fn af_ftm_clkin0(&self) -> usize { 4 }
}

impl AfFbAd17 for Ptb16 {
   fn af_fb_ad17(&self) -> usize { 5 }
}

impl AfEwmIn for Ptb16 {
   fn af_ewm_in(&self) -> usize { 6 }
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

impl AfSpi1Sin for Ptb17 {
   fn af_spi1_sin(&self) -> usize { 2 }
}

impl AfUart0Tx for Ptb17 {
   fn af_uart0_tx(&self) -> usize { 3 }
}

impl AfFtmClkin1 for Ptb17 {
   fn af_ftm_clkin1(&self) -> usize { 4 }
}

impl AfFbAd16 for Ptb17 {
   fn af_fb_ad16(&self) -> usize { 5 }
}

impl AfEwmOutB for Ptb17 {
   fn af_ewm_out_b(&self) -> usize { 6 }
}

pub const PTB18: Ptb18 = Ptb18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb18 {}

impl Pin for Ptb18 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 18 }
}

impl AfPtb18 for Ptb18 {
   fn af_ptb18(&self) -> usize { 1 }
}

impl AfCan0Tx for Ptb18 {
   fn af_can0_tx(&self) -> usize { 2 }
}

impl AfFtm2Ch0 for Ptb18 {
   fn af_ftm2_ch0(&self) -> usize { 3 }
}

impl AfI2s0TxBclk for Ptb18 {
   fn af_i2s0_tx_bclk(&self) -> usize { 4 }
}

impl AfFbAd15 for Ptb18 {
   fn af_fb_ad15(&self) -> usize { 5 }
}

impl AfFtm2QdPha for Ptb18 {
   fn af_ftm2_qd_pha(&self) -> usize { 6 }
}

pub const PTB19: Ptb19 = Ptb19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb19 {}

impl Pin for Ptb19 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 19 }
}

impl AfPtb19 for Ptb19 {
   fn af_ptb19(&self) -> usize { 1 }
}

impl AfCan0Rx for Ptb19 {
   fn af_can0_rx(&self) -> usize { 2 }
}

impl AfFtm2Ch1 for Ptb19 {
   fn af_ftm2_ch1(&self) -> usize { 3 }
}

impl AfI2s0TxFs for Ptb19 {
   fn af_i2s0_tx_fs(&self) -> usize { 4 }
}

impl AfFbOeB for Ptb19 {
   fn af_fb_oe_b(&self) -> usize { 5 }
}

impl AfFtm2QdPhb for Ptb19 {
   fn af_ftm2_qd_phb(&self) -> usize { 6 }
}

pub const PTB20: Ptb20 = Ptb20 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb20 {}

impl Pin for Ptb20 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 20 }
}

impl AfPtb20 for Ptb20 {
   fn af_ptb20(&self) -> usize { 1 }
}

impl AfSpi2Pcs0 for Ptb20 {
   fn af_spi2_pcs0(&self) -> usize { 2 }
}

impl AfFbAd31 for Ptb20 {
   fn af_fb_ad31(&self) -> usize { 5 }
}

impl AfCmp0Out for Ptb20 {
   fn af_cmp0_out(&self) -> usize { 6 }
}

pub const PTB21: Ptb21 = Ptb21 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb21 {}

impl Pin for Ptb21 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 21 }
}

impl AfPtb21 for Ptb21 {
   fn af_ptb21(&self) -> usize { 1 }
}

impl AfSpi2Sck for Ptb21 {
   fn af_spi2_sck(&self) -> usize { 2 }
}

impl AfFbAd30 for Ptb21 {
   fn af_fb_ad30(&self) -> usize { 5 }
}

impl AfCmp1Out for Ptb21 {
   fn af_cmp1_out(&self) -> usize { 6 }
}

pub const PTB22: Ptb22 = Ptb22 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb22 {}

impl Pin for Ptb22 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 22 }
}

impl AfPtb22 for Ptb22 {
   fn af_ptb22(&self) -> usize { 1 }
}

impl AfSpi2Sout for Ptb22 {
   fn af_spi2_sout(&self) -> usize { 2 }
}

impl AfFbAd29 for Ptb22 {
   fn af_fb_ad29(&self) -> usize { 5 }
}

impl AfCmp2Out for Ptb22 {
   fn af_cmp2_out(&self) -> usize { 6 }
}

pub const PTB23: Ptb23 = Ptb23 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptb23 {}

impl Pin for Ptb23 {
   fn port(&self) -> Port { PORTB }
   fn index(&self) -> usize { 23 }
}

impl AfPtb23 for Ptb23 {
   fn af_ptb23(&self) -> usize { 1 }
}

impl AfSpi2Sin for Ptb23 {
   fn af_spi2_sin(&self) -> usize { 2 }
}

impl AfSpi0Pcs5 for Ptb23 {
   fn af_spi0_pcs5(&self) -> usize { 3 }
}

impl AfFbAd28 for Ptb23 {
   fn af_fb_ad28(&self) -> usize { 5 }
}

pub const PTC0: Ptc0 = Ptc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc0 {}

impl Pin for Ptc0 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 0 }
}

impl AfAdc0Se14 for Ptc0 {
   fn af_adc0_se14(&self) -> usize { 0 }
}

impl AfPtc0 for Ptc0 {
   fn af_ptc0(&self) -> usize { 1 }
}

impl AfSpi0Pcs4 for Ptc0 {
   fn af_spi0_pcs4(&self) -> usize { 2 }
}

impl AfPdb0Extrg for Ptc0 {
   fn af_pdb0_extrg(&self) -> usize { 3 }
}

impl AfUsbSofOut for Ptc0 {
   fn af_usb_sof_out(&self) -> usize { 4 }
}

impl AfFbAd14 for Ptc0 {
   fn af_fb_ad14(&self) -> usize { 5 }
}

impl AfI2s0Txd1 for Ptc0 {
   fn af_i2s0_txd1(&self) -> usize { 6 }
}

pub const PTC1: Ptc1 = Ptc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc1 {}

impl Pin for Ptc1 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se15 for Ptc1 {
   fn af_adc0_se15(&self) -> usize { 0 }
}

impl AfPtc1 for Ptc1 {
   fn af_ptc1(&self) -> usize { 1 }
}

impl AfSpi0Pcs3 for Ptc1 {
   fn af_spi0_pcs3(&self) -> usize { 2 }
}

impl AfUart1RtsB for Ptc1 {
   fn af_uart1_rts_b(&self) -> usize { 3 }
}

impl AfFtm0Ch0 for Ptc1 {
   fn af_ftm0_ch0(&self) -> usize { 4 }
}

impl AfFbAd13 for Ptc1 {
   fn af_fb_ad13(&self) -> usize { 5 }
}

impl AfI2s0Txd0 for Ptc1 {
   fn af_i2s0_txd0(&self) -> usize { 6 }
}

pub const PTC2: Ptc2 = Ptc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc2 {}

impl Pin for Ptc2 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Se4b for Ptc2 {
   fn af_adc0_se4b(&self) -> usize { 0 }
}

impl AfCmp1In0 for Ptc2 {
   fn af_cmp1_in0(&self) -> usize { 0 }
}

impl AfPtc2 for Ptc2 {
   fn af_ptc2(&self) -> usize { 1 }
}

impl AfSpi0Pcs2 for Ptc2 {
   fn af_spi0_pcs2(&self) -> usize { 2 }
}

impl AfUart1CtsB for Ptc2 {
   fn af_uart1_cts_b(&self) -> usize { 3 }
}

impl AfFtm0Ch1 for Ptc2 {
   fn af_ftm0_ch1(&self) -> usize { 4 }
}

impl AfFbAd12 for Ptc2 {
   fn af_fb_ad12(&self) -> usize { 5 }
}

impl AfI2s0TxFs for Ptc2 {
   fn af_i2s0_tx_fs(&self) -> usize { 6 }
}

pub const PTC3: Ptc3 = Ptc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc3 {}

impl Pin for Ptc3 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 3 }
}

impl AfCmp1In1 for Ptc3 {
   fn af_cmp1_in1(&self) -> usize { 0 }
}

impl AfPtc3 for Ptc3 {
   fn af_ptc3(&self) -> usize { 1 }
}

impl AfSpi0Pcs1 for Ptc3 {
   fn af_spi0_pcs1(&self) -> usize { 2 }
}

impl AfUart1Rx for Ptc3 {
   fn af_uart1_rx(&self) -> usize { 3 }
}

impl AfFtm0Ch2 for Ptc3 {
   fn af_ftm0_ch2(&self) -> usize { 4 }
}

impl AfClkout for Ptc3 {
   fn af_clkout(&self) -> usize { 5 }
}

impl AfI2s0TxBclk for Ptc3 {
   fn af_i2s0_tx_bclk(&self) -> usize { 6 }
}

pub const PTC4: Ptc4 = Ptc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc4 {}

impl Pin for Ptc4 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 4 }
}

impl AfPtc4 for Ptc4 {
   fn af_ptc4(&self) -> usize { 1 }
}

impl AfSpi0Pcs0 for Ptc4 {
   fn af_spi0_pcs0(&self) -> usize { 2 }
}

impl AfUart1Tx for Ptc4 {
   fn af_uart1_tx(&self) -> usize { 3 }
}

impl AfFtm0Ch3 for Ptc4 {
   fn af_ftm0_ch3(&self) -> usize { 4 }
}

impl AfFbAd11 for Ptc4 {
   fn af_fb_ad11(&self) -> usize { 5 }
}

impl AfCmp1Out for Ptc4 {
   fn af_cmp1_out(&self) -> usize { 6 }
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

impl AfSpi0Sck for Ptc5 {
   fn af_spi0_sck(&self) -> usize { 2 }
}

impl AfLptmr0Alt2 for Ptc5 {
   fn af_lptmr0_alt2(&self) -> usize { 3 }
}

impl AfI2s0Rxd0 for Ptc5 {
   fn af_i2s0_rxd0(&self) -> usize { 4 }
}

impl AfFbAd10 for Ptc5 {
   fn af_fb_ad10(&self) -> usize { 5 }
}

impl AfCmp0Out for Ptc5 {
   fn af_cmp0_out(&self) -> usize { 6 }
}

impl AfFtm0Ch2 for Ptc5 {
   fn af_ftm0_ch2(&self) -> usize { 7 }
}

pub const PTC6: Ptc6 = Ptc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc6 {}

impl Pin for Ptc6 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 6 }
}

impl AfCmp0In0 for Ptc6 {
   fn af_cmp0_in0(&self) -> usize { 0 }
}

impl AfPtc6 for Ptc6 {
   fn af_ptc6(&self) -> usize { 1 }
}

impl AfSpi0Sout for Ptc6 {
   fn af_spi0_sout(&self) -> usize { 2 }
}

impl AfPdb0Extrg for Ptc6 {
   fn af_pdb0_extrg(&self) -> usize { 3 }
}

impl AfI2s0RxBclk for Ptc6 {
   fn af_i2s0_rx_bclk(&self) -> usize { 4 }
}

impl AfFbAd9 for Ptc6 {
   fn af_fb_ad9(&self) -> usize { 5 }
}

impl AfI2s0Mclk for Ptc6 {
   fn af_i2s0_mclk(&self) -> usize { 6 }
}

pub const PTC7: Ptc7 = Ptc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc7 {}

impl Pin for Ptc7 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 7 }
}

impl AfCmp0In1 for Ptc7 {
   fn af_cmp0_in1(&self) -> usize { 0 }
}

impl AfPtc7 for Ptc7 {
   fn af_ptc7(&self) -> usize { 1 }
}

impl AfSpi0Sin for Ptc7 {
   fn af_spi0_sin(&self) -> usize { 2 }
}

impl AfUsbSofOut for Ptc7 {
   fn af_usb_sof_out(&self) -> usize { 3 }
}

impl AfI2s0RxFs for Ptc7 {
   fn af_i2s0_rx_fs(&self) -> usize { 4 }
}

impl AfFbAd8 for Ptc7 {
   fn af_fb_ad8(&self) -> usize { 5 }
}

pub const PTC8: Ptc8 = Ptc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc8 {}

impl Pin for Ptc8 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 8 }
}

impl AfAdc1Se4b for Ptc8 {
   fn af_adc1_se4b(&self) -> usize { 0 }
}

impl AfCmp0In2 for Ptc8 {
   fn af_cmp0_in2(&self) -> usize { 0 }
}

impl AfPtc8 for Ptc8 {
   fn af_ptc8(&self) -> usize { 1 }
}

impl AfFtm3Ch4 for Ptc8 {
   fn af_ftm3_ch4(&self) -> usize { 3 }
}

impl AfI2s0Mclk for Ptc8 {
   fn af_i2s0_mclk(&self) -> usize { 4 }
}

impl AfFbAd7 for Ptc8 {
   fn af_fb_ad7(&self) -> usize { 5 }
}

pub const PTC9: Ptc9 = Ptc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc9 {}

impl Pin for Ptc9 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 9 }
}

impl AfAdc1Se5b for Ptc9 {
   fn af_adc1_se5b(&self) -> usize { 0 }
}

impl AfCmp0In3 for Ptc9 {
   fn af_cmp0_in3(&self) -> usize { 0 }
}

impl AfPtc9 for Ptc9 {
   fn af_ptc9(&self) -> usize { 1 }
}

impl AfFtm3Ch5 for Ptc9 {
   fn af_ftm3_ch5(&self) -> usize { 3 }
}

impl AfI2s0RxBclk for Ptc9 {
   fn af_i2s0_rx_bclk(&self) -> usize { 4 }
}

impl AfFbAd6 for Ptc9 {
   fn af_fb_ad6(&self) -> usize { 5 }
}

impl AfFtm2Flt0 for Ptc9 {
   fn af_ftm2_flt0(&self) -> usize { 6 }
}

pub const PTC10: Ptc10 = Ptc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc10 {}

impl Pin for Ptc10 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 10 }
}

impl AfAdc1Se6b for Ptc10 {
   fn af_adc1_se6b(&self) -> usize { 0 }
}

impl AfPtc10 for Ptc10 {
   fn af_ptc10(&self) -> usize { 1 }
}

impl AfI2c1Scl for Ptc10 {
   fn af_i2c1_scl(&self) -> usize { 2 }
}

impl AfFtm3Ch6 for Ptc10 {
   fn af_ftm3_ch6(&self) -> usize { 3 }
}

impl AfI2s0RxFs for Ptc10 {
   fn af_i2s0_rx_fs(&self) -> usize { 4 }
}

impl AfFbAd5 for Ptc10 {
   fn af_fb_ad5(&self) -> usize { 5 }
}

pub const PTC11: Ptc11 = Ptc11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc11 {}

impl Pin for Ptc11 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 11 }
}

impl AfAdc1Se7b for Ptc11 {
   fn af_adc1_se7b(&self) -> usize { 0 }
}

impl AfPtc11 for Ptc11 {
   fn af_ptc11(&self) -> usize { 1 }
}

impl AfI2c1Sda for Ptc11 {
   fn af_i2c1_sda(&self) -> usize { 2 }
}

impl AfFtm3Ch7 for Ptc11 {
   fn af_ftm3_ch7(&self) -> usize { 3 }
}

impl AfI2s0Rxd1 for Ptc11 {
   fn af_i2s0_rxd1(&self) -> usize { 4 }
}

impl AfFbRwB for Ptc11 {
   fn af_fb_rw_b(&self) -> usize { 5 }
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

impl AfUart4RtsB for Ptc12 {
   fn af_uart4_rts_b(&self) -> usize { 3 }
}

impl AfFbAd27 for Ptc12 {
   fn af_fb_ad27(&self) -> usize { 5 }
}

impl AfFtm3Flt0 for Ptc12 {
   fn af_ftm3_flt0(&self) -> usize { 6 }
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

impl AfUart4CtsB for Ptc13 {
   fn af_uart4_cts_b(&self) -> usize { 3 }
}

impl AfFbAd26 for Ptc13 {
   fn af_fb_ad26(&self) -> usize { 5 }
}

pub const PTC14: Ptc14 = Ptc14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc14 {}

impl Pin for Ptc14 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 14 }
}

impl AfPtc14 for Ptc14 {
   fn af_ptc14(&self) -> usize { 1 }
}

impl AfUart4Rx for Ptc14 {
   fn af_uart4_rx(&self) -> usize { 3 }
}

impl AfFbAd25 for Ptc14 {
   fn af_fb_ad25(&self) -> usize { 5 }
}

pub const PTC15: Ptc15 = Ptc15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc15 {}

impl Pin for Ptc15 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 15 }
}

impl AfPtc15 for Ptc15 {
   fn af_ptc15(&self) -> usize { 1 }
}

impl AfUart4Tx for Ptc15 {
   fn af_uart4_tx(&self) -> usize { 3 }
}

impl AfFbAd24 for Ptc15 {
   fn af_fb_ad24(&self) -> usize { 5 }
}

pub const PTC16: Ptc16 = Ptc16 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc16 {}

impl Pin for Ptc16 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 16 }
}

impl AfPtc16 for Ptc16 {
   fn af_ptc16(&self) -> usize { 1 }
}

impl AfUart3Rx for Ptc16 {
   fn af_uart3_rx(&self) -> usize { 3 }
}

impl AfEnet01588Tmr0 for Ptc16 {
   fn af_enet0_1588_tmr0(&self) -> usize { 4 }
}

impl AfFbCs5B for Ptc16 {
   fn af_fb_cs5_b(&self) -> usize { 5 }
}

impl AfFbTsiz1 for Ptc16 {
   fn af_fb_tsiz1(&self) -> usize { 5 }
}

impl AfFbBe2316Bls158B for Ptc16 {
   fn af_fb_be23_16_bls15_8_b(&self) -> usize { 5 }
}

pub const PTC17: Ptc17 = Ptc17 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc17 {}

impl Pin for Ptc17 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 17 }
}

impl AfPtc17 for Ptc17 {
   fn af_ptc17(&self) -> usize { 1 }
}

impl AfUart3Tx for Ptc17 {
   fn af_uart3_tx(&self) -> usize { 3 }
}

impl AfEnet01588Tmr1 for Ptc17 {
   fn af_enet0_1588_tmr1(&self) -> usize { 4 }
}

impl AfFbCs4B for Ptc17 {
   fn af_fb_cs4_b(&self) -> usize { 5 }
}

impl AfFbTsiz0 for Ptc17 {
   fn af_fb_tsiz0(&self) -> usize { 5 }
}

impl AfFbBe3124Bls70B for Ptc17 {
   fn af_fb_be31_24_bls7_0_b(&self) -> usize { 5 }
}

pub const PTC18: Ptc18 = Ptc18 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc18 {}

impl Pin for Ptc18 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 18 }
}

impl AfPtc18 for Ptc18 {
   fn af_ptc18(&self) -> usize { 1 }
}

impl AfUart3RtsB for Ptc18 {
   fn af_uart3_rts_b(&self) -> usize { 3 }
}

impl AfEnet01588Tmr2 for Ptc18 {
   fn af_enet0_1588_tmr2(&self) -> usize { 4 }
}

impl AfFbTbstB for Ptc18 {
   fn af_fb_tbst_b(&self) -> usize { 5 }
}

impl AfFbCs2B for Ptc18 {
   fn af_fb_cs2_b(&self) -> usize { 5 }
}

impl AfFbBe158Bls2316B for Ptc18 {
   fn af_fb_be15_8_bls23_16_b(&self) -> usize { 5 }
}

pub const PTC19: Ptc19 = Ptc19 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptc19 {}

impl Pin for Ptc19 {
   fn port(&self) -> Port { PORTC }
   fn index(&self) -> usize { 19 }
}

impl AfPtc19 for Ptc19 {
   fn af_ptc19(&self) -> usize { 1 }
}

impl AfUart3CtsB for Ptc19 {
   fn af_uart3_cts_b(&self) -> usize { 3 }
}

impl AfEnet01588Tmr3 for Ptc19 {
   fn af_enet0_1588_tmr3(&self) -> usize { 4 }
}

impl AfFbCs3B for Ptc19 {
   fn af_fb_cs3_b(&self) -> usize { 5 }
}

impl AfFbBe70Bls3124B for Ptc19 {
   fn af_fb_be7_0_bls31_24_b(&self) -> usize { 5 }
}

impl AfFbTaB for Ptc19 {
   fn af_fb_ta_b(&self) -> usize { 6 }
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

impl AfSpi0Pcs0 for Ptd0 {
   fn af_spi0_pcs0(&self) -> usize { 2 }
}

impl AfUart2RtsB for Ptd0 {
   fn af_uart2_rts_b(&self) -> usize { 3 }
}

impl AfFtm3Ch0 for Ptd0 {
   fn af_ftm3_ch0(&self) -> usize { 4 }
}

impl AfFbAle for Ptd0 {
   fn af_fb_ale(&self) -> usize { 5 }
}

impl AfFbCs1B for Ptd0 {
   fn af_fb_cs1_b(&self) -> usize { 5 }
}

impl AfFbTsB for Ptd0 {
   fn af_fb_ts_b(&self) -> usize { 5 }
}

pub const PTD1: Ptd1 = Ptd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd1 {}

impl Pin for Ptd1 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 1 }
}

impl AfAdc0Se5b for Ptd1 {
   fn af_adc0_se5b(&self) -> usize { 0 }
}

impl AfPtd1 for Ptd1 {
   fn af_ptd1(&self) -> usize { 1 }
}

impl AfSpi0Sck for Ptd1 {
   fn af_spi0_sck(&self) -> usize { 2 }
}

impl AfUart2CtsB for Ptd1 {
   fn af_uart2_cts_b(&self) -> usize { 3 }
}

impl AfFtm3Ch1 for Ptd1 {
   fn af_ftm3_ch1(&self) -> usize { 4 }
}

impl AfFbCs0B for Ptd1 {
   fn af_fb_cs0_b(&self) -> usize { 5 }
}

pub const PTD2: Ptd2 = Ptd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd2 {}

impl Pin for Ptd2 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 2 }
}

impl AfPtd2 for Ptd2 {
   fn af_ptd2(&self) -> usize { 1 }
}

impl AfSpi0Sout for Ptd2 {
   fn af_spi0_sout(&self) -> usize { 2 }
}

impl AfUart2Rx for Ptd2 {
   fn af_uart2_rx(&self) -> usize { 3 }
}

impl AfFtm3Ch2 for Ptd2 {
   fn af_ftm3_ch2(&self) -> usize { 4 }
}

impl AfFbAd4 for Ptd2 {
   fn af_fb_ad4(&self) -> usize { 5 }
}

impl AfI2c0Scl for Ptd2 {
   fn af_i2c0_scl(&self) -> usize { 7 }
}

pub const PTD3: Ptd3 = Ptd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd3 {}

impl Pin for Ptd3 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 3 }
}

impl AfPtd3 for Ptd3 {
   fn af_ptd3(&self) -> usize { 1 }
}

impl AfSpi0Sin for Ptd3 {
   fn af_spi0_sin(&self) -> usize { 2 }
}

impl AfUart2Tx for Ptd3 {
   fn af_uart2_tx(&self) -> usize { 3 }
}

impl AfFtm3Ch3 for Ptd3 {
   fn af_ftm3_ch3(&self) -> usize { 4 }
}

impl AfFbAd3 for Ptd3 {
   fn af_fb_ad3(&self) -> usize { 5 }
}

impl AfI2c0Sda for Ptd3 {
   fn af_i2c0_sda(&self) -> usize { 7 }
}

pub const PTD4: Ptd4 = Ptd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd4 {}

impl Pin for Ptd4 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 4 }
}

impl AfPtd4 for Ptd4 {
   fn af_ptd4(&self) -> usize { 1 }
}

impl AfSpi0Pcs1 for Ptd4 {
   fn af_spi0_pcs1(&self) -> usize { 2 }
}

impl AfUart0RtsB for Ptd4 {
   fn af_uart0_rts_b(&self) -> usize { 3 }
}

impl AfFtm0Ch4 for Ptd4 {
   fn af_ftm0_ch4(&self) -> usize { 4 }
}

impl AfFbAd2 for Ptd4 {
   fn af_fb_ad2(&self) -> usize { 5 }
}

impl AfEwmIn for Ptd4 {
   fn af_ewm_in(&self) -> usize { 6 }
}

impl AfSpi1Pcs0 for Ptd4 {
   fn af_spi1_pcs0(&self) -> usize { 7 }
}

pub const PTD5: Ptd5 = Ptd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd5 {}

impl Pin for Ptd5 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 5 }
}

impl AfAdc0Se6b for Ptd5 {
   fn af_adc0_se6b(&self) -> usize { 0 }
}

impl AfPtd5 for Ptd5 {
   fn af_ptd5(&self) -> usize { 1 }
}

impl AfSpi0Pcs2 for Ptd5 {
   fn af_spi0_pcs2(&self) -> usize { 2 }
}

impl AfUart0CtsB for Ptd5 {
   fn af_uart0_cts_b(&self) -> usize { 3 }
}

impl AfUart0ColB for Ptd5 {
   fn af_uart0_col_b(&self) -> usize { 3 }
}

impl AfFtm0Ch5 for Ptd5 {
   fn af_ftm0_ch5(&self) -> usize { 4 }
}

impl AfFbAd1 for Ptd5 {
   fn af_fb_ad1(&self) -> usize { 5 }
}

impl AfEwmOutB for Ptd5 {
   fn af_ewm_out_b(&self) -> usize { 6 }
}

impl AfSpi1Sck for Ptd5 {
   fn af_spi1_sck(&self) -> usize { 7 }
}

pub const PTD6: Ptd6 = Ptd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd6 {}

impl Pin for Ptd6 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 6 }
}

impl AfAdc0Se7b for Ptd6 {
   fn af_adc0_se7b(&self) -> usize { 0 }
}

impl AfPtd6 for Ptd6 {
   fn af_ptd6(&self) -> usize { 1 }
}

impl AfSpi0Pcs3 for Ptd6 {
   fn af_spi0_pcs3(&self) -> usize { 2 }
}

impl AfUart0Rx for Ptd6 {
   fn af_uart0_rx(&self) -> usize { 3 }
}

impl AfFtm0Ch6 for Ptd6 {
   fn af_ftm0_ch6(&self) -> usize { 4 }
}

impl AfFbAd0 for Ptd6 {
   fn af_fb_ad0(&self) -> usize { 5 }
}

impl AfFtm0Flt0 for Ptd6 {
   fn af_ftm0_flt0(&self) -> usize { 6 }
}

impl AfSpi1Sout for Ptd6 {
   fn af_spi1_sout(&self) -> usize { 7 }
}

pub const PTD7: Ptd7 = Ptd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ptd7 {}

impl Pin for Ptd7 {
   fn port(&self) -> Port { PORTD }
   fn index(&self) -> usize { 7 }
}

impl AfPtd7 for Ptd7 {
   fn af_ptd7(&self) -> usize { 1 }
}

impl AfCmtIro for Ptd7 {
   fn af_cmt_iro(&self) -> usize { 2 }
}

impl AfUart0Tx for Ptd7 {
   fn af_uart0_tx(&self) -> usize { 3 }
}

impl AfFtm0Ch7 for Ptd7 {
   fn af_ftm0_ch7(&self) -> usize { 4 }
}

impl AfFtm0Flt1 for Ptd7 {
   fn af_ftm0_flt1(&self) -> usize { 6 }
}

impl AfSpi1Sin for Ptd7 {
   fn af_spi1_sin(&self) -> usize { 7 }
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

impl AfI2c0Scl for Ptd8 {
   fn af_i2c0_scl(&self) -> usize { 2 }
}

impl AfUart5Rx for Ptd8 {
   fn af_uart5_rx(&self) -> usize { 3 }
}

impl AfFbA16 for Ptd8 {
   fn af_fb_a16(&self) -> usize { 6 }
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

impl AfI2c0Sda for Ptd9 {
   fn af_i2c0_sda(&self) -> usize { 2 }
}

impl AfUart5Tx for Ptd9 {
   fn af_uart5_tx(&self) -> usize { 3 }
}

impl AfFbA17 for Ptd9 {
   fn af_fb_a17(&self) -> usize { 6 }
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

impl AfUart5RtsB for Ptd10 {
   fn af_uart5_rts_b(&self) -> usize { 3 }
}

impl AfFbA18 for Ptd10 {
   fn af_fb_a18(&self) -> usize { 6 }
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

impl AfSpi2Pcs0 for Ptd11 {
   fn af_spi2_pcs0(&self) -> usize { 2 }
}

impl AfUart5CtsB for Ptd11 {
   fn af_uart5_cts_b(&self) -> usize { 3 }
}

impl AfSdhc0Clkin for Ptd11 {
   fn af_sdhc0_clkin(&self) -> usize { 4 }
}

impl AfFbA19 for Ptd11 {
   fn af_fb_a19(&self) -> usize { 6 }
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

impl AfSpi2Sck for Ptd12 {
   fn af_spi2_sck(&self) -> usize { 2 }
}

impl AfFtm3Flt0 for Ptd12 {
   fn af_ftm3_flt0(&self) -> usize { 3 }
}

impl AfSdhc0D4 for Ptd12 {
   fn af_sdhc0_d4(&self) -> usize { 4 }
}

impl AfFbA20 for Ptd12 {
   fn af_fb_a20(&self) -> usize { 6 }
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

impl AfSpi2Sout for Ptd13 {
   fn af_spi2_sout(&self) -> usize { 2 }
}

impl AfSdhc0D5 for Ptd13 {
   fn af_sdhc0_d5(&self) -> usize { 4 }
}

impl AfFbA21 for Ptd13 {
   fn af_fb_a21(&self) -> usize { 6 }
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

impl AfSpi2Sin for Ptd14 {
   fn af_spi2_sin(&self) -> usize { 2 }
}

impl AfSdhc0D6 for Ptd14 {
   fn af_sdhc0_d6(&self) -> usize { 4 }
}

impl AfFbA22 for Ptd14 {
   fn af_fb_a22(&self) -> usize { 6 }
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

impl AfSpi2Pcs1 for Ptd15 {
   fn af_spi2_pcs1(&self) -> usize { 2 }
}

impl AfSdhc0D7 for Ptd15 {
   fn af_sdhc0_d7(&self) -> usize { 4 }
}

impl AfFbA23 for Ptd15 {
   fn af_fb_a23(&self) -> usize { 6 }
}

pub const PTE0: Pte0 = Pte0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte0 {}

impl Pin for Pte0 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 0 }
}

impl AfAdc1Se4a for Pte0 {
   fn af_adc1_se4a(&self) -> usize { 0 }
}

impl AfPte0 for Pte0 {
   fn af_pte0(&self) -> usize { 1 }
}

impl AfSpi1Pcs1 for Pte0 {
   fn af_spi1_pcs1(&self) -> usize { 2 }
}

impl AfUart1Tx for Pte0 {
   fn af_uart1_tx(&self) -> usize { 3 }
}

impl AfSdhc0D1 for Pte0 {
   fn af_sdhc0_d1(&self) -> usize { 4 }
}

impl AfTraceClkout for Pte0 {
   fn af_trace_clkout(&self) -> usize { 5 }
}

impl AfI2c1Sda for Pte0 {
   fn af_i2c1_sda(&self) -> usize { 6 }
}

impl AfRtcClkout for Pte0 {
   fn af_rtc_clkout(&self) -> usize { 7 }
}

pub const PTE1: Pte1 = Pte1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte1 {}

impl Pin for Pte1 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 1 }
}

impl AfAdc1Se5a for Pte1 {
   fn af_adc1_se5a(&self) -> usize { 0 }
}

impl AfPte1 for Pte1 {
   fn af_pte1(&self) -> usize { 1 }
}

impl AfSpi1Sout for Pte1 {
   fn af_spi1_sout(&self) -> usize { 2 }
}

impl AfUart1Rx for Pte1 {
   fn af_uart1_rx(&self) -> usize { 3 }
}

impl AfSdhc0D0 for Pte1 {
   fn af_sdhc0_d0(&self) -> usize { 4 }
}

impl AfTraceD3 for Pte1 {
   fn af_trace_d3(&self) -> usize { 5 }
}

impl AfI2c1Scl for Pte1 {
   fn af_i2c1_scl(&self) -> usize { 6 }
}

impl AfSpi1Sin for Pte1 {
   fn af_spi1_sin(&self) -> usize { 7 }
}

pub const PTE2: Pte2 = Pte2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte2 {}

impl Pin for Pte2 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 2 }
}

impl AfAdc0Dp2 for Pte2 {
   fn af_adc0_dp2(&self) -> usize { 0 }
}

impl AfAdc1Se6a for Pte2 {
   fn af_adc1_se6a(&self) -> usize { 0 }
}

impl AfPte2 for Pte2 {
   fn af_pte2(&self) -> usize { 1 }
}

impl AfSpi1Sck for Pte2 {
   fn af_spi1_sck(&self) -> usize { 2 }
}

impl AfUart1CtsB for Pte2 {
   fn af_uart1_cts_b(&self) -> usize { 3 }
}

impl AfSdhc0Dclk for Pte2 {
   fn af_sdhc0_dclk(&self) -> usize { 4 }
}

impl AfTraceD2 for Pte2 {
   fn af_trace_d2(&self) -> usize { 5 }
}

pub const PTE3: Pte3 = Pte3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte3 {}

impl Pin for Pte3 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 3 }
}

impl AfAdc0Dm2 for Pte3 {
   fn af_adc0_dm2(&self) -> usize { 0 }
}

impl AfAdc1Se7a for Pte3 {
   fn af_adc1_se7a(&self) -> usize { 0 }
}

impl AfPte3 for Pte3 {
   fn af_pte3(&self) -> usize { 1 }
}

impl AfSpi1Sin for Pte3 {
   fn af_spi1_sin(&self) -> usize { 2 }
}

impl AfUart1RtsB for Pte3 {
   fn af_uart1_rts_b(&self) -> usize { 3 }
}

impl AfSdhc0Cmd for Pte3 {
   fn af_sdhc0_cmd(&self) -> usize { 4 }
}

impl AfTraceD1 for Pte3 {
   fn af_trace_d1(&self) -> usize { 5 }
}

impl AfSpi1Sout for Pte3 {
   fn af_spi1_sout(&self) -> usize { 7 }
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

impl AfSpi1Pcs0 for Pte4 {
   fn af_spi1_pcs0(&self) -> usize { 2 }
}

impl AfUart3Tx for Pte4 {
   fn af_uart3_tx(&self) -> usize { 3 }
}

impl AfSdhc0D3 for Pte4 {
   fn af_sdhc0_d3(&self) -> usize { 4 }
}

impl AfTraceD0 for Pte4 {
   fn af_trace_d0(&self) -> usize { 5 }
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

impl AfSpi1Pcs2 for Pte5 {
   fn af_spi1_pcs2(&self) -> usize { 2 }
}

impl AfUart3Rx for Pte5 {
   fn af_uart3_rx(&self) -> usize { 3 }
}

impl AfSdhc0D2 for Pte5 {
   fn af_sdhc0_d2(&self) -> usize { 4 }
}

impl AfFtm3Ch0 for Pte5 {
   fn af_ftm3_ch0(&self) -> usize { 6 }
}

pub const PTE6: Pte6 = Pte6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte6 {}

impl Pin for Pte6 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 6 }
}

impl AfPte6 for Pte6 {
   fn af_pte6(&self) -> usize { 1 }
}

impl AfSpi1Pcs3 for Pte6 {
   fn af_spi1_pcs3(&self) -> usize { 2 }
}

impl AfUart3CtsB for Pte6 {
   fn af_uart3_cts_b(&self) -> usize { 3 }
}

impl AfI2s0Mclk for Pte6 {
   fn af_i2s0_mclk(&self) -> usize { 4 }
}

impl AfFtm3Ch1 for Pte6 {
   fn af_ftm3_ch1(&self) -> usize { 6 }
}

impl AfUsbSofOut for Pte6 {
   fn af_usb_sof_out(&self) -> usize { 7 }
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

impl AfUart3RtsB for Pte7 {
   fn af_uart3_rts_b(&self) -> usize { 3 }
}

impl AfI2s0Rxd0 for Pte7 {
   fn af_i2s0_rxd0(&self) -> usize { 4 }
}

impl AfFtm3Ch2 for Pte7 {
   fn af_ftm3_ch2(&self) -> usize { 6 }
}

pub const PTE8: Pte8 = Pte8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte8 {}

impl Pin for Pte8 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 8 }
}

impl AfPte8 for Pte8 {
   fn af_pte8(&self) -> usize { 1 }
}

impl AfI2s0Rxd1 for Pte8 {
   fn af_i2s0_rxd1(&self) -> usize { 2 }
}

impl AfUart5Tx for Pte8 {
   fn af_uart5_tx(&self) -> usize { 3 }
}

impl AfI2s0RxFs for Pte8 {
   fn af_i2s0_rx_fs(&self) -> usize { 4 }
}

impl AfFtm3Ch3 for Pte8 {
   fn af_ftm3_ch3(&self) -> usize { 6 }
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

impl AfI2s0Txd1 for Pte9 {
   fn af_i2s0_txd1(&self) -> usize { 2 }
}

impl AfUart5Rx for Pte9 {
   fn af_uart5_rx(&self) -> usize { 3 }
}

impl AfI2s0RxBclk for Pte9 {
   fn af_i2s0_rx_bclk(&self) -> usize { 4 }
}

impl AfFtm3Ch4 for Pte9 {
   fn af_ftm3_ch4(&self) -> usize { 6 }
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

impl AfUart5CtsB for Pte10 {
   fn af_uart5_cts_b(&self) -> usize { 3 }
}

impl AfI2s0Txd0 for Pte10 {
   fn af_i2s0_txd0(&self) -> usize { 4 }
}

impl AfFtm3Ch5 for Pte10 {
   fn af_ftm3_ch5(&self) -> usize { 6 }
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

impl AfUart5RtsB for Pte11 {
   fn af_uart5_rts_b(&self) -> usize { 3 }
}

impl AfI2s0TxFs for Pte11 {
   fn af_i2s0_tx_fs(&self) -> usize { 4 }
}

impl AfFtm3Ch6 for Pte11 {
   fn af_ftm3_ch6(&self) -> usize { 6 }
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

impl AfI2s0TxBclk for Pte12 {
   fn af_i2s0_tx_bclk(&self) -> usize { 4 }
}

impl AfFtm3Ch7 for Pte12 {
   fn af_ftm3_ch7(&self) -> usize { 6 }
}

pub const PTE24: Pte24 = Pte24 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte24 {}

impl Pin for Pte24 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 24 }
}

impl AfAdc0Se17 for Pte24 {
   fn af_adc0_se17(&self) -> usize { 0 }
}

impl AfPte24 for Pte24 {
   fn af_pte24(&self) -> usize { 1 }
}

impl AfUart4Tx for Pte24 {
   fn af_uart4_tx(&self) -> usize { 3 }
}

impl AfI2c0Scl for Pte24 {
   fn af_i2c0_scl(&self) -> usize { 5 }
}

impl AfEwmOutB for Pte24 {
   fn af_ewm_out_b(&self) -> usize { 6 }
}

pub const PTE25: Pte25 = Pte25 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte25 {}

impl Pin for Pte25 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 25 }
}

impl AfAdc0Se18 for Pte25 {
   fn af_adc0_se18(&self) -> usize { 0 }
}

impl AfPte25 for Pte25 {
   fn af_pte25(&self) -> usize { 1 }
}

impl AfUart4Rx for Pte25 {
   fn af_uart4_rx(&self) -> usize { 3 }
}

impl AfI2c0Sda for Pte25 {
   fn af_i2c0_sda(&self) -> usize { 5 }
}

impl AfEwmIn for Pte25 {
   fn af_ewm_in(&self) -> usize { 6 }
}

pub const PTE26: Pte26 = Pte26 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte26 {}

impl Pin for Pte26 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 26 }
}

impl AfPte26 for Pte26 {
   fn af_pte26(&self) -> usize { 1 }
}

impl AfEnet1588Clkin for Pte26 {
   fn af_enet_1588_clkin(&self) -> usize { 2 }
}

impl AfUart4CtsB for Pte26 {
   fn af_uart4_cts_b(&self) -> usize { 3 }
}

impl AfRtcClkout for Pte26 {
   fn af_rtc_clkout(&self) -> usize { 6 }
}

impl AfUsbClkin for Pte26 {
   fn af_usb_clkin(&self) -> usize { 7 }
}

pub const PTE27: Pte27 = Pte27 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte27 {}

impl Pin for Pte27 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 27 }
}

impl AfPte27 for Pte27 {
   fn af_pte27(&self) -> usize { 1 }
}

impl AfUart4RtsB for Pte27 {
   fn af_uart4_rts_b(&self) -> usize { 3 }
}

pub const PTE28: Pte28 = Pte28 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pte28 {}

impl Pin for Pte28 {
   fn port(&self) -> Port { PORTE }
   fn index(&self) -> usize { 28 }
}

impl AfPte28 for Pte28 {
   fn af_pte28(&self) -> usize { 1 }
}

