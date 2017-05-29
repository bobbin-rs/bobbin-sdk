pub use stm32_common::chip::gpio::*;

pub const GPIOA: Gpio = Gpio(0x40020000);
pub const GPIOB: Gpio = Gpio(0x40020400);
pub const GPIOC: Gpio = Gpio(0x40020800);
pub const GPIOD: Gpio = Gpio(0x40020c00);
pub const GPIOE: Gpio = Gpio(0x40021000);
pub const GPIOF: Gpio = Gpio(0x40021400);
pub const GPIOG: Gpio = Gpio(0x40021800);
pub const GPIOH: Gpio = Gpio(0x40021c00);
pub const GPIOI: Gpio = Gpio(0x40022000);
pub const GPIOJ: Gpio = Gpio(0x40022400);
pub const GPIOK: Gpio = Gpio(0x40022800);

pub trait Pin {
   fn port(&self) -> Gpio;
   fn index(&self) -> usize;
}

pub trait AfTim2Ch1 {
   fn af_tim2_ch1(&self) -> usize;
}

pub trait AfTim2Etr {
   fn af_tim2_etr(&self) -> usize;
}

pub trait AfTim5Ch1 {
   fn af_tim5_ch1(&self) -> usize;
}

pub trait AfTim8Etr {
   fn af_tim8_etr(&self) -> usize;
}

pub trait AfUsart2Cts {
   fn af_usart2_cts(&self) -> usize;
}

pub trait AfUart4Tx {
   fn af_uart4_tx(&self) -> usize;
}

pub trait AfEthMiiCrs {
   fn af_eth_mii_crs(&self) -> usize;
}

pub trait AfEventout {
   fn af_eventout(&self) -> usize;
}

pub trait AfTim2Ch2 {
   fn af_tim2_ch2(&self) -> usize;
}

pub trait AfTim5Ch2 {
   fn af_tim5_ch2(&self) -> usize;
}

pub trait AfUsart2Rts {
   fn af_usart2_rts(&self) -> usize;
}

pub trait AfUart4Rx {
   fn af_uart4_rx(&self) -> usize;
}

pub trait AfEthMiiRxClk {
   fn af_eth_mii_rx_clk(&self) -> usize;
}

pub trait AfEthRmiiRefClk {
   fn af_eth_rmii_ref_clk(&self) -> usize;
}

pub trait AfTim2Ch3 {
   fn af_tim2_ch3(&self) -> usize;
}

pub trait AfTim5Ch3 {
   fn af_tim5_ch3(&self) -> usize;
}

pub trait AfTim9Ch1 {
   fn af_tim9_ch1(&self) -> usize;
}

pub trait AfUsart2Tx {
   fn af_usart2_tx(&self) -> usize;
}

pub trait AfEthMdio {
   fn af_eth_mdio(&self) -> usize;
}

pub trait AfTim2Ch4 {
   fn af_tim2_ch4(&self) -> usize;
}

pub trait AfTim5Ch4 {
   fn af_tim5_ch4(&self) -> usize;
}

pub trait AfTim9Ch2 {
   fn af_tim9_ch2(&self) -> usize;
}

pub trait AfUsart2Rx {
   fn af_usart2_rx(&self) -> usize;
}

pub trait AfOtgHsUlpiD0 {
   fn af_otg_hs_ulpi_d0(&self) -> usize;
}

pub trait AfEthMiiCol {
   fn af_eth_mii_col(&self) -> usize;
}

pub trait AfLcdB5 {
   fn af_lcd_b5(&self) -> usize;
}

pub trait AfSpi1Nss {
   fn af_spi1_nss(&self) -> usize;
}

pub trait AfSpi3Nss {
   fn af_spi3_nss(&self) -> usize;
}

pub trait AfI2s3Ws {
   fn af_i2s3_ws(&self) -> usize;
}

pub trait AfUsart2Ck {
   fn af_usart2_ck(&self) -> usize;
}

pub trait AfOtgHsSof {
   fn af_otg_hs_sof(&self) -> usize;
}

pub trait AfDcmiHsync {
   fn af_dcmi_hsync(&self) -> usize;
}

pub trait AfLcdVsync {
   fn af_lcd_vsync(&self) -> usize;
}

pub trait AfTim8Ch1n {
   fn af_tim8_ch1n(&self) -> usize;
}

pub trait AfSpi1Sck {
   fn af_spi1_sck(&self) -> usize;
}

pub trait AfOtgHsUlpiCk {
   fn af_otg_hs_ulpi_ck(&self) -> usize;
}

pub trait AfTim1Bkin {
   fn af_tim1_bkin(&self) -> usize;
}

pub trait AfTim3Ch1 {
   fn af_tim3_ch1(&self) -> usize;
}

pub trait AfTim8Bkin {
   fn af_tim8_bkin(&self) -> usize;
}

pub trait AfSpi1Miso {
   fn af_spi1_miso(&self) -> usize;
}

pub trait AfTim13Ch1 {
   fn af_tim13_ch1(&self) -> usize;
}

pub trait AfDcmiPxclk {
   fn af_dcmi_pxclk(&self) -> usize;
}

pub trait AfLcdG2 {
   fn af_lcd_g2(&self) -> usize;
}

pub trait AfTim1Ch1n {
   fn af_tim1_ch1n(&self) -> usize;
}

pub trait AfTim3Ch2 {
   fn af_tim3_ch2(&self) -> usize;
}

pub trait AfSpi1Mosi {
   fn af_spi1_mosi(&self) -> usize;
}

pub trait AfTim14Ch1 {
   fn af_tim14_ch1(&self) -> usize;
}

pub trait AfEthMiiRxDv {
   fn af_eth_mii_rx_dv(&self) -> usize;
}

pub trait AfEthRmiiCrsDv {
   fn af_eth_rmii_crs_dv(&self) -> usize;
}

pub trait AfMco1 {
   fn af_mco1(&self) -> usize;
}

pub trait AfTim1Ch1 {
   fn af_tim1_ch1(&self) -> usize;
}

pub trait AfI2c3Scl {
   fn af_i2c3_scl(&self) -> usize;
}

pub trait AfUsart1Ck {
   fn af_usart1_ck(&self) -> usize;
}

pub trait AfOtgFsSof {
   fn af_otg_fs_sof(&self) -> usize;
}

pub trait AfLcdR6 {
   fn af_lcd_r6(&self) -> usize;
}

pub trait AfTim1Ch2 {
   fn af_tim1_ch2(&self) -> usize;
}

pub trait AfI2c3Smba {
   fn af_i2c3_smba(&self) -> usize;
}

pub trait AfUasrt1Tx {
   fn af_uasrt1_tx(&self) -> usize;
}

pub trait AfDcmiD0 {
   fn af_dcmi_d0(&self) -> usize;
}

pub trait AfTim1Ch3 {
   fn af_tim1_ch3(&self) -> usize;
}

pub trait AfUsart1Rx {
   fn af_usart1_rx(&self) -> usize;
}

pub trait AfOtgFsId {
   fn af_otg_fs_id(&self) -> usize;
}

pub trait AfDcmiD1 {
   fn af_dcmi_d1(&self) -> usize;
}

pub trait AfTim1Ch4 {
   fn af_tim1_ch4(&self) -> usize;
}

pub trait AfUsart1Cts {
   fn af_usart1_cts(&self) -> usize;
}

pub trait AfCan1Rx {
   fn af_can1_rx(&self) -> usize;
}

pub trait AfOtgFsDm {
   fn af_otg_fs_dm(&self) -> usize;
}

pub trait AfLcdR4 {
   fn af_lcd_r4(&self) -> usize;
}

pub trait AfTim1Etr {
   fn af_tim1_etr(&self) -> usize;
}

pub trait AfUsart1Rts {
   fn af_usart1_rts(&self) -> usize;
}

pub trait AfCan1Tx {
   fn af_can1_tx(&self) -> usize;
}

pub trait AfOtgFsDp {
   fn af_otg_fs_dp(&self) -> usize;
}

pub trait AfLcdR5 {
   fn af_lcd_r5(&self) -> usize;
}

pub trait AfJtms {
   fn af_jtms(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfJtck {
   fn af_jtck(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfJtdi {
   fn af_jtdi(&self) -> usize;
}

pub trait AfTim1Ch2n {
   fn af_tim1_ch2n(&self) -> usize;
}

pub trait AfTim3Ch3 {
   fn af_tim3_ch3(&self) -> usize;
}

pub trait AfTim8Ch2n {
   fn af_tim8_ch2n(&self) -> usize;
}

pub trait AfLcdR3 {
   fn af_lcd_r3(&self) -> usize;
}

pub trait AfOtgHsUlpiD1 {
   fn af_otg_hs_ulpi_d1(&self) -> usize;
}

pub trait AfEthMiiRxd2 {
   fn af_eth_mii_rxd2(&self) -> usize;
}

pub trait AfTim1Ch3n {
   fn af_tim1_ch3n(&self) -> usize;
}

pub trait AfTim3Ch4 {
   fn af_tim3_ch4(&self) -> usize;
}

pub trait AfTim8Ch3n {
   fn af_tim8_ch3n(&self) -> usize;
}

pub trait AfOtgHsUlpiD2 {
   fn af_otg_hs_ulpi_d2(&self) -> usize;
}

pub trait AfEthMiiRxd3 {
   fn af_eth_mii_rxd3(&self) -> usize;
}

pub trait AfJtdo {
   fn af_jtdo(&self) -> usize;
}

pub trait AfTraceswo {
   fn af_traceswo(&self) -> usize;
}

pub trait AfSpi3Sck {
   fn af_spi3_sck(&self) -> usize;
}

pub trait AfIs2cCk {
   fn af_is2c_ck(&self) -> usize;
}

pub trait AfNjtrst {
   fn af_njtrst(&self) -> usize;
}

pub trait AfSpi3Miso {
   fn af_spi3_miso(&self) -> usize;
}

pub trait AfI2s3extSd {
   fn af_i2s3ext_sd(&self) -> usize;
}

pub trait AfI2c1Smba {
   fn af_i2c1_smba(&self) -> usize;
}

pub trait AfSpi3Mosi {
   fn af_spi3_mosi(&self) -> usize;
}

pub trait AfI2s3Sd {
   fn af_i2s3_sd(&self) -> usize;
}

pub trait AfCan2Rx {
   fn af_can2_rx(&self) -> usize;
}

pub trait AfOtgHsUlpiD7 {
   fn af_otg_hs_ulpi_d7(&self) -> usize;
}

pub trait AfEthPpsOut {
   fn af_eth_pps_out(&self) -> usize;
}

pub trait AfFmcSdcke1 {
   fn af_fmc_sdcke1(&self) -> usize;
}

pub trait AfDcmiD10 {
   fn af_dcmi_d10(&self) -> usize;
}

pub trait AfTim4Ch1 {
   fn af_tim4_ch1(&self) -> usize;
}

pub trait AfI2c1Scl {
   fn af_i2c1_scl(&self) -> usize;
}

pub trait AfUsart1Tx {
   fn af_usart1_tx(&self) -> usize;
}

pub trait AfCan2Tx {
   fn af_can2_tx(&self) -> usize;
}

pub trait AfFmcSdne1 {
   fn af_fmc_sdne1(&self) -> usize;
}

pub trait AfDcmiD5 {
   fn af_dcmi_d5(&self) -> usize;
}

pub trait AfTim4Ch2 {
   fn af_tim4_ch2(&self) -> usize;
}

pub trait AfI2c1Sda {
   fn af_i2c1_sda(&self) -> usize;
}

pub trait AfFmcNl {
   fn af_fmc_nl(&self) -> usize;
}

pub trait AfDcmiVsync {
   fn af_dcmi_vsync(&self) -> usize;
}

pub trait AfTim4Ch3 {
   fn af_tim4_ch3(&self) -> usize;
}

pub trait AfTim10Ch1 {
   fn af_tim10_ch1(&self) -> usize;
}

pub trait AfEthMiiTxd3 {
   fn af_eth_mii_txd3(&self) -> usize;
}

pub trait AfSdioD4 {
   fn af_sdio_d4(&self) -> usize;
}

pub trait AfDcmiD6 {
   fn af_dcmi_d6(&self) -> usize;
}

pub trait AfLcdB6 {
   fn af_lcd_b6(&self) -> usize;
}

pub trait AfTim4Ch4 {
   fn af_tim4_ch4(&self) -> usize;
}

pub trait AfTim11Ch1 {
   fn af_tim11_ch1(&self) -> usize;
}

pub trait AfSpi2Nss {
   fn af_spi2_nss(&self) -> usize;
}

pub trait AfI2s2Ws {
   fn af_i2s2_ws(&self) -> usize;
}

pub trait AfSdioD5 {
   fn af_sdio_d5(&self) -> usize;
}

pub trait AfDcmiD7 {
   fn af_dcmi_d7(&self) -> usize;
}

pub trait AfLcdB7 {
   fn af_lcd_b7(&self) -> usize;
}

pub trait AfI2c2Scl {
   fn af_i2c2_scl(&self) -> usize;
}

pub trait AfSpi2Sck {
   fn af_spi2_sck(&self) -> usize;
}

pub trait AfI2s2Ck {
   fn af_i2s2_ck(&self) -> usize;
}

pub trait AfUsart3Tx {
   fn af_usart3_tx(&self) -> usize;
}

pub trait AfOtgHsUlpiD3 {
   fn af_otg_hs_ulpi_d3(&self) -> usize;
}

pub trait AfEthMiiRxEr {
   fn af_eth_mii_rx_er(&self) -> usize;
}

pub trait AfLcdG4 {
   fn af_lcd_g4(&self) -> usize;
}

pub trait AfSim2Ch4 {
   fn af_sim2_ch4(&self) -> usize;
}

pub trait AfI2c2Sda {
   fn af_i2c2_sda(&self) -> usize;
}

pub trait AfUsart3Rx {
   fn af_usart3_rx(&self) -> usize;
}

pub trait AfOtgHsUlpiD4 {
   fn af_otg_hs_ulpi_d4(&self) -> usize;
}

pub trait AfEthMiiTxEn {
   fn af_eth_mii_tx_en(&self) -> usize;
}

pub trait AfEthRmiiTxEn {
   fn af_eth_rmii_tx_en(&self) -> usize;
}

pub trait AfLcdG5 {
   fn af_lcd_g5(&self) -> usize;
}

pub trait AfI2c2Smba {
   fn af_i2c2_smba(&self) -> usize;
}

pub trait AfUsart3Ck {
   fn af_usart3_ck(&self) -> usize;
}

pub trait AfOtgHsUlpiD5 {
   fn af_otg_hs_ulpi_d5(&self) -> usize;
}

pub trait AfEthMiiTxd0 {
   fn af_eth_mii_txd0(&self) -> usize;
}

pub trait AfEthRmiiTxd0 {
   fn af_eth_rmii_txd0(&self) -> usize;
}

pub trait AfOtgHsId {
   fn af_otg_hs_id(&self) -> usize;
}

pub trait AfUsart3Cts {
   fn af_usart3_cts(&self) -> usize;
}

pub trait AfOtgHsUlpiD6 {
   fn af_otg_hs_ulpi_d6(&self) -> usize;
}

pub trait AfEthMiiTxd1 {
   fn af_eth_mii_txd1(&self) -> usize;
}

pub trait AfEthRmiiTxd1 {
   fn af_eth_rmii_txd1(&self) -> usize;
}

pub trait AfTim2Ch2n {
   fn af_tim2_ch2n(&self) -> usize;
}

pub trait AfSpi2Miso {
   fn af_spi2_miso(&self) -> usize;
}

pub trait AfI2s2extSd {
   fn af_i2s2ext_sd(&self) -> usize;
}

pub trait AfUsart3Rts {
   fn af_usart3_rts(&self) -> usize;
}

pub trait AfTim12Ch1 {
   fn af_tim12_ch1(&self) -> usize;
}

pub trait AfOtgHsDm {
   fn af_otg_hs_dm(&self) -> usize;
}

pub trait AfRtcRefin {
   fn af_rtc_refin(&self) -> usize;
}

pub trait AfSpi2Mosi {
   fn af_spi2_mosi(&self) -> usize;
}

pub trait AfI2s2Sd {
   fn af_i2s2_sd(&self) -> usize;
}

pub trait AfTim12Ch2 {
   fn af_tim12_ch2(&self) -> usize;
}

pub trait AfOtgHsDp {
   fn af_otg_hs_dp(&self) -> usize;
}

pub trait AfOtgHsUlpiStp {
   fn af_otg_hs_ulpi_stp(&self) -> usize;
}

pub trait AfFmcSdnwe {
   fn af_fmc_sdnwe(&self) -> usize;
}

pub trait AfEthMdc {
   fn af_eth_mdc(&self) -> usize;
}

pub trait AfOtgHsUlpiDir {
   fn af_otg_hs_ulpi_dir(&self) -> usize;
}

pub trait AfEthMiiTxd2 {
   fn af_eth_mii_txd2(&self) -> usize;
}

pub trait AfFmcSdne0 {
   fn af_fmc_sdne0(&self) -> usize;
}

pub trait AfOtgHsUlpiNxt {
   fn af_otg_hs_ulpi_nxt(&self) -> usize;
}

pub trait AfEthMiiTxClk {
   fn af_eth_mii_tx_clk(&self) -> usize;
}

pub trait AfFmcSdcke0 {
   fn af_fmc_sdcke0(&self) -> usize;
}

pub trait AfEthMiiRxd0 {
   fn af_eth_mii_rxd0(&self) -> usize;
}

pub trait AfEthRmiiRxd0 {
   fn af_eth_rmii_rxd0(&self) -> usize;
}

pub trait AfEthMiiRxd1 {
   fn af_eth_mii_rxd1(&self) -> usize;
}

pub trait AfEthRmiiRxd1 {
   fn af_eth_rmii_rxd1(&self) -> usize;
}

pub trait AfTim8Ch1 {
   fn af_tim8_ch1(&self) -> usize;
}

pub trait AfI2s2Mck {
   fn af_i2s2_mck(&self) -> usize;
}

pub trait AfUsart6Tx {
   fn af_usart6_tx(&self) -> usize;
}

pub trait AfSdioD6 {
   fn af_sdio_d6(&self) -> usize;
}

pub trait AfLcdHsync {
   fn af_lcd_hsync(&self) -> usize;
}

pub trait AfTim8Ch2 {
   fn af_tim8_ch2(&self) -> usize;
}

pub trait AfI2s3Mck {
   fn af_i2s3_mck(&self) -> usize;
}

pub trait AfUsart6Rx {
   fn af_usart6_rx(&self) -> usize;
}

pub trait AfSdioD7 {
   fn af_sdio_d7(&self) -> usize;
}

pub trait AfLcdG6 {
   fn af_lcd_g6(&self) -> usize;
}

pub trait AfTim8Ch3 {
   fn af_tim8_ch3(&self) -> usize;
}

pub trait AfUsart6Ck {
   fn af_usart6_ck(&self) -> usize;
}

pub trait AfSdioD0 {
   fn af_sdio_d0(&self) -> usize;
}

pub trait AfDcmiD2 {
   fn af_dcmi_d2(&self) -> usize;
}

pub trait AfMco2 {
   fn af_mco2(&self) -> usize;
}

pub trait AfTim8Ch4 {
   fn af_tim8_ch4(&self) -> usize;
}

pub trait AfI2c3Sda {
   fn af_i2c3_sda(&self) -> usize;
}

pub trait AfI2sCkin {
   fn af_i2s_ckin(&self) -> usize;
}

pub trait AfSdioD1 {
   fn af_sdio_d1(&self) -> usize;
}

pub trait AfDcmiD3 {
   fn af_dcmi_d3(&self) -> usize;
}

pub trait AfI2s3Ck {
   fn af_i2s3_ck(&self) -> usize;
}

pub trait AfSdioD2 {
   fn af_sdio_d2(&self) -> usize;
}

pub trait AfDcmiD8 {
   fn af_dcmi_d8(&self) -> usize;
}

pub trait AfLcdR2 {
   fn af_lcd_r2(&self) -> usize;
}

pub trait AfSdioD3 {
   fn af_sdio_d3(&self) -> usize;
}

pub trait AfDcmiD4 {
   fn af_dcmi_d4(&self) -> usize;
}

pub trait AfSdioCk {
   fn af_sdio_ck(&self) -> usize;
}

pub trait AfDcmiD9 {
   fn af_dcmi_d9(&self) -> usize;
}

pub trait AfFmcD2 {
   fn af_fmc_d2(&self) -> usize;
}

pub trait AfFmcD3 {
   fn af_fmc_d3(&self) -> usize;
}

pub trait AfTim3Etr {
   fn af_tim3_etr(&self) -> usize;
}

pub trait AfUart5Rx {
   fn af_uart5_rx(&self) -> usize;
}

pub trait AfSdioCmd {
   fn af_sdio_cmd(&self) -> usize;
}

pub trait AfDcmiD11 {
   fn af_dcmi_d11(&self) -> usize;
}

pub trait AfFmcClk {
   fn af_fmc_clk(&self) -> usize;
}

pub trait AfLcdG7 {
   fn af_lcd_g7(&self) -> usize;
}

pub trait AfFmcNoe {
   fn af_fmc_noe(&self) -> usize;
}

pub trait AfFmcNwe {
   fn af_fmc_nwe(&self) -> usize;
}

pub trait AfSai1SdA {
   fn af_sai1_sd_a(&self) -> usize;
}

pub trait AfFmcNwait {
   fn af_fmc_nwait(&self) -> usize;
}

pub trait AfLcdB2 {
   fn af_lcd_b2(&self) -> usize;
}

pub trait AfFmcNe1 {
   fn af_fmc_ne1(&self) -> usize;
}

pub trait AfFmcNce2 {
   fn af_fmc_nce2(&self) -> usize;
}

pub trait AfFmcD13 {
   fn af_fmc_d13(&self) -> usize;
}

pub trait AfFmcD14 {
   fn af_fmc_d14(&self) -> usize;
}

pub trait AfFmcD15 {
   fn af_fmc_d15(&self) -> usize;
}

pub trait AfFmcA16 {
   fn af_fmc_a16(&self) -> usize;
}

pub trait AfFmcA17 {
   fn af_fmc_a17(&self) -> usize;
}

pub trait AfFmcA18 {
   fn af_fmc_a18(&self) -> usize;
}

pub trait AfFmcD0 {
   fn af_fmc_d0(&self) -> usize;
}

pub trait AfFmcD1 {
   fn af_fmc_d1(&self) -> usize;
}

pub trait AfTim4Etr {
   fn af_tim4_etr(&self) -> usize;
}

pub trait AfUart8Rx {
   fn af_uart8_rx(&self) -> usize;
}

pub trait AfFmcNbl0 {
   fn af_fmc_nbl0(&self) -> usize;
}

pub trait AfUart8Tx {
   fn af_uart8_tx(&self) -> usize;
}

pub trait AfFmcNbl1 {
   fn af_fmc_nbl1(&self) -> usize;
}

pub trait AfTraceclk {
   fn af_traceclk(&self) -> usize;
}

pub trait AfSpi4Sck {
   fn af_spi4_sck(&self) -> usize;
}

pub trait AfSai1MclkA {
   fn af_sai1_mclk_a(&self) -> usize;
}

pub trait AfFmcA23 {
   fn af_fmc_a23(&self) -> usize;
}

pub trait AfTraced0 {
   fn af_traced0(&self) -> usize;
}

pub trait AfSai1SdB {
   fn af_sai1_sd_b(&self) -> usize;
}

pub trait AfFmcA19 {
   fn af_fmc_a19(&self) -> usize;
}

pub trait AfTraced1 {
   fn af_traced1(&self) -> usize;
}

pub trait AfSpi4Nss {
   fn af_spi4_nss(&self) -> usize;
}

pub trait AfSai1FsA {
   fn af_sai1_fs_a(&self) -> usize;
}

pub trait AfFmcA20 {
   fn af_fmc_a20(&self) -> usize;
}

pub trait AfTraced2 {
   fn af_traced2(&self) -> usize;
}

pub trait AfSpi4Miso {
   fn af_spi4_miso(&self) -> usize;
}

pub trait AfSai1SckA {
   fn af_sai1_sck_a(&self) -> usize;
}

pub trait AfFmcA21 {
   fn af_fmc_a21(&self) -> usize;
}

pub trait AfTraced3 {
   fn af_traced3(&self) -> usize;
}

pub trait AfSpi4Mosi {
   fn af_spi4_mosi(&self) -> usize;
}

pub trait AfFmcA22 {
   fn af_fmc_a22(&self) -> usize;
}

pub trait AfFmcD4 {
   fn af_fmc_d4(&self) -> usize;
}

pub trait AfFmcD5 {
   fn af_fmc_d5(&self) -> usize;
}

pub trait AfFmcD6 {
   fn af_fmc_d6(&self) -> usize;
}

pub trait AfFmcD7 {
   fn af_fmc_d7(&self) -> usize;
}

pub trait AfFmcD8 {
   fn af_fmc_d8(&self) -> usize;
}

pub trait AfLcdG3 {
   fn af_lcd_g3(&self) -> usize;
}

pub trait AfFmcD9 {
   fn af_fmc_d9(&self) -> usize;
}

pub trait AfLcdB4 {
   fn af_lcd_b4(&self) -> usize;
}

pub trait AfFmcD10 {
   fn af_fmc_d10(&self) -> usize;
}

pub trait AfLcdDe {
   fn af_lcd_de(&self) -> usize;
}

pub trait AfFmcD11 {
   fn af_fmc_d11(&self) -> usize;
}

pub trait AfLcdClk {
   fn af_lcd_clk(&self) -> usize;
}

pub trait AfFmcD12 {
   fn af_fmc_d12(&self) -> usize;
}

pub trait AfLcdR7 {
   fn af_lcd_r7(&self) -> usize;
}

pub trait AfFmcA0 {
   fn af_fmc_a0(&self) -> usize;
}

pub trait AfFmcA1 {
   fn af_fmc_a1(&self) -> usize;
}

pub trait AfFmcA2 {
   fn af_fmc_a2(&self) -> usize;
}

pub trait AfFmcA3 {
   fn af_fmc_a3(&self) -> usize;
}

pub trait AfFmcA4 {
   fn af_fmc_a4(&self) -> usize;
}

pub trait AfFmcA5 {
   fn af_fmc_a5(&self) -> usize;
}

pub trait AfSpi5Nss {
   fn af_spi5_nss(&self) -> usize;
}

pub trait AfUart7Rx {
   fn af_uart7_rx(&self) -> usize;
}

pub trait AfFmcNiord {
   fn af_fmc_niord(&self) -> usize;
}

pub trait AfSpi5Sck {
   fn af_spi5_sck(&self) -> usize;
}

pub trait AfSai1MclkB {
   fn af_sai1_mclk_b(&self) -> usize;
}

pub trait AfUart7Tx {
   fn af_uart7_tx(&self) -> usize;
}

pub trait AfFmcNreg {
   fn af_fmc_nreg(&self) -> usize;
}

pub trait AfSpi5Miso {
   fn af_spi5_miso(&self) -> usize;
}

pub trait AfSaiSckB {
   fn af_sai_sck_b(&self) -> usize;
}

pub trait AfFmcNiowr {
   fn af_fmc_niowr(&self) -> usize;
}

pub trait AfSpi5Mosi {
   fn af_spi5_mosi(&self) -> usize;
}

pub trait AfSaiFsB {
   fn af_sai_fs_b(&self) -> usize;
}

pub trait AfFmcCd {
   fn af_fmc_cd(&self) -> usize;
}

pub trait AfFmcIntr {
   fn af_fmc_intr(&self) -> usize;
}

pub trait AfFmcSdnras {
   fn af_fmc_sdnras(&self) -> usize;
}

pub trait AfDcmiD12 {
   fn af_dcmi_d12(&self) -> usize;
}

pub trait AfFmcA6 {
   fn af_fmc_a6(&self) -> usize;
}

pub trait AfFmcA7 {
   fn af_fmc_a7(&self) -> usize;
}

pub trait AfFmcA8 {
   fn af_fmc_a8(&self) -> usize;
}

pub trait AfFmcA9 {
   fn af_fmc_a9(&self) -> usize;
}

pub trait AfFmcA10 {
   fn af_fmc_a10(&self) -> usize;
}

pub trait AfFmcA11 {
   fn af_fmc_a11(&self) -> usize;
}

pub trait AfFmcA12 {
   fn af_fmc_a12(&self) -> usize;
}

pub trait AfFmcA13 {
   fn af_fmc_a13(&self) -> usize;
}

pub trait AfFmcA14 {
   fn af_fmc_a14(&self) -> usize;
}

pub trait AfFmcBa0 {
   fn af_fmc_ba0(&self) -> usize;
}

pub trait AfFmcA15 {
   fn af_fmc_a15(&self) -> usize;
}

pub trait AfFmcBa1 {
   fn af_fmc_ba1(&self) -> usize;
}

pub trait AfFmcInt2 {
   fn af_fmc_int2(&self) -> usize;
}

pub trait AfFmcInt3 {
   fn af_fmc_int3(&self) -> usize;
}

pub trait AfDcmiD13 {
   fn af_dcmi_d13(&self) -> usize;
}

pub trait AfSpi6Nss {
   fn af_spi6_nss(&self) -> usize;
}

pub trait AfUsart6Rts {
   fn af_usart6_rts(&self) -> usize;
}

pub trait AfEthPsOut {
   fn af_eth_ps_out(&self) -> usize;
}

pub trait AfFmcSdclk {
   fn af_fmc_sdclk(&self) -> usize;
}

pub trait AfFmcNe2 {
   fn af_fmc_ne2(&self) -> usize;
}

pub trait AfFmcNce3 {
   fn af_fmc_nce3(&self) -> usize;
}

pub trait AfFmcNce41 {
   fn af_fmc_nce4_1(&self) -> usize;
}

pub trait AfFmcNe3 {
   fn af_fmc_ne3(&self) -> usize;
}

pub trait AfSpi6Miso {
   fn af_spi6_miso(&self) -> usize;
}

pub trait AfFmcNce42 {
   fn af_fmc_nce4_2(&self) -> usize;
}

pub trait AfLcdB3 {
   fn af_lcd_b3(&self) -> usize;
}

pub trait AfSpi6Sck {
   fn af_spi6_sck(&self) -> usize;
}

pub trait AfUsart6Cts {
   fn af_usart6_cts(&self) -> usize;
}

pub trait AfFmcNe4 {
   fn af_fmc_ne4(&self) -> usize;
}

pub trait AfLcdB1 {
   fn af_lcd_b1(&self) -> usize;
}

pub trait AfSpi6Mosi {
   fn af_spi6_mosi(&self) -> usize;
}

pub trait AfFmcA24 {
   fn af_fmc_a24(&self) -> usize;
}

pub trait AfFmcA25 {
   fn af_fmc_a25(&self) -> usize;
}

pub trait AfFmcSndcas {
   fn af_fmc_sndcas(&self) -> usize;
}

pub trait AfLcdR0 {
   fn af_lcd_r0(&self) -> usize;
}

pub trait AfLcdR1 {
   fn af_lcd_r1(&self) -> usize;
}

pub trait AfFmcD16 {
   fn af_fmc_d16(&self) -> usize;
}

pub trait AfFmcD17 {
   fn af_fmc_d17(&self) -> usize;
}

pub trait AfFmcD18 {
   fn af_fmc_d18(&self) -> usize;
}

pub trait AfFmcD19 {
   fn af_fmc_d19(&self) -> usize;
}

pub trait AfFmcD20 {
   fn af_fmc_d20(&self) -> usize;
}

pub trait AfFmcD21 {
   fn af_fmc_d21(&self) -> usize;
}

pub trait AfFmcD22 {
   fn af_fmc_d22(&self) -> usize;
}

pub trait AfFmcD23 {
   fn af_fmc_d23(&self) -> usize;
}

pub trait AfFmcD24 {
   fn af_fmc_d24(&self) -> usize;
}

pub trait AfFmcD25 {
   fn af_fmc_d25(&self) -> usize;
}

pub trait AfFmcD26 {
   fn af_fmc_d26(&self) -> usize;
}

pub trait AfFmcD27 {
   fn af_fmc_d27(&self) -> usize;
}

pub trait AfFmcNbl2 {
   fn af_fmc_nbl2(&self) -> usize;
}

pub trait AfFmcNbl3 {
   fn af_fmc_nbl3(&self) -> usize;
}

pub trait AfDcmVsync {
   fn af_dcm_vsync(&self) -> usize;
}

pub trait AfFmcD28 {
   fn af_fmc_d28(&self) -> usize;
}

pub trait AfFmcD29 {
   fn af_fmc_d29(&self) -> usize;
}

pub trait AfFmcD30 {
   fn af_fmc_d30(&self) -> usize;
}

pub trait AfFmcD31 {
   fn af_fmc_d31(&self) -> usize;
}

pub trait AfLcdG0 {
   fn af_lcd_g0(&self) -> usize;
}

pub trait AfLcdG1 {
   fn af_lcd_g1(&self) -> usize;
}

pub trait AfLcdB0 {
   fn af_lcd_b0(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin for Pa0 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 0 }
}

impl AfTim2Ch1 for Pa0 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa0 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfTim5Ch1 for Pa0 {
   fn af_tim5_ch1(&self) -> usize { 2 }
}

impl AfTim8Etr for Pa0 {
   fn af_tim8_etr(&self) -> usize { 3 }
}

impl AfUsart2Cts for Pa0 {
   fn af_usart2_cts(&self) -> usize { 7 }
}

impl AfUart4Tx for Pa0 {
   fn af_uart4_tx(&self) -> usize { 8 }
}

impl AfEthMiiCrs for Pa0 {
   fn af_eth_mii_crs(&self) -> usize { 11 }
}

impl AfEventout for Pa0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA1: Pa1 = Pa1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin for Pa1 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 1 }
}

impl AfTim2Ch2 for Pa1 {
   fn af_tim2_ch2(&self) -> usize { 1 }
}

impl AfTim5Ch2 for Pa1 {
   fn af_tim5_ch2(&self) -> usize { 2 }
}

impl AfUsart2Rts for Pa1 {
   fn af_usart2_rts(&self) -> usize { 7 }
}

impl AfUart4Rx for Pa1 {
   fn af_uart4_rx(&self) -> usize { 8 }
}

impl AfEthMiiRxClk for Pa1 {
   fn af_eth_mii_rx_clk(&self) -> usize { 11 }
}

impl AfEthRmiiRefClk for Pa1 {
   fn af_eth_rmii_ref_clk(&self) -> usize { 11 }
}

impl AfEventout for Pa1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA2: Pa2 = Pa2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin for Pa2 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 2 }
}

impl AfTim2Ch3 for Pa2 {
   fn af_tim2_ch3(&self) -> usize { 1 }
}

impl AfTim5Ch3 for Pa2 {
   fn af_tim5_ch3(&self) -> usize { 2 }
}

impl AfTim9Ch1 for Pa2 {
   fn af_tim9_ch1(&self) -> usize { 3 }
}

impl AfUsart2Tx for Pa2 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfEthMdio for Pa2 {
   fn af_eth_mdio(&self) -> usize { 11 }
}

impl AfEventout for Pa2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin for Pa3 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 3 }
}

impl AfTim2Ch4 for Pa3 {
   fn af_tim2_ch4(&self) -> usize { 1 }
}

impl AfTim5Ch4 for Pa3 {
   fn af_tim5_ch4(&self) -> usize { 2 }
}

impl AfTim9Ch2 for Pa3 {
   fn af_tim9_ch2(&self) -> usize { 3 }
}

impl AfUsart2Rx for Pa3 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfOtgHsUlpiD0 for Pa3 {
   fn af_otg_hs_ulpi_d0(&self) -> usize { 10 }
}

impl AfEthMiiCol for Pa3 {
   fn af_eth_mii_col(&self) -> usize { 11 }
}

impl AfLcdB5 for Pa3 {
   fn af_lcd_b5(&self) -> usize { 14 }
}

impl AfEventout for Pa3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin for Pa4 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 4 }
}

impl AfSpi1Nss for Pa4 {
   fn af_spi1_nss(&self) -> usize { 5 }
}

impl AfSpi3Nss for Pa4 {
   fn af_spi3_nss(&self) -> usize { 6 }
}

impl AfI2s3Ws for Pa4 {
   fn af_i2s3_ws(&self) -> usize { 6 }
}

impl AfUsart2Ck for Pa4 {
   fn af_usart2_ck(&self) -> usize { 7 }
}

impl AfOtgHsSof for Pa4 {
   fn af_otg_hs_sof(&self) -> usize { 12 }
}

impl AfDcmiHsync for Pa4 {
   fn af_dcmi_hsync(&self) -> usize { 13 }
}

impl AfLcdVsync for Pa4 {
   fn af_lcd_vsync(&self) -> usize { 14 }
}

impl AfEventout for Pa4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA5: Pa5 = Pa5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin for Pa5 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 5 }
}

impl AfTim2Ch1 for Pa5 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa5 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfTim8Ch1n for Pa5 {
   fn af_tim8_ch1n(&self) -> usize { 3 }
}

impl AfSpi1Sck for Pa5 {
   fn af_spi1_sck(&self) -> usize { 5 }
}

impl AfOtgHsUlpiCk for Pa5 {
   fn af_otg_hs_ulpi_ck(&self) -> usize { 10 }
}

impl AfEventout for Pa5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA6: Pa6 = Pa6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin for Pa6 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 6 }
}

impl AfTim1Bkin for Pa6 {
   fn af_tim1_bkin(&self) -> usize { 1 }
}

impl AfTim3Ch1 for Pa6 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTim8Bkin for Pa6 {
   fn af_tim8_bkin(&self) -> usize { 3 }
}

impl AfSpi1Miso for Pa6 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

impl AfTim13Ch1 for Pa6 {
   fn af_tim13_ch1(&self) -> usize { 9 }
}

impl AfDcmiPxclk for Pa6 {
   fn af_dcmi_pxclk(&self) -> usize { 13 }
}

impl AfLcdG2 for Pa6 {
   fn af_lcd_g2(&self) -> usize { 14 }
}

impl AfEventout for Pa6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA7: Pa7 = Pa7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin for Pa7 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 7 }
}

impl AfTim1Ch1n for Pa7 {
   fn af_tim1_ch1n(&self) -> usize { 1 }
}

impl AfTim3Ch2 for Pa7 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTim8Ch1n for Pa7 {
   fn af_tim8_ch1n(&self) -> usize { 3 }
}

impl AfSpi1Mosi for Pa7 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

impl AfTim14Ch1 for Pa7 {
   fn af_tim14_ch1(&self) -> usize { 9 }
}

impl AfEthMiiRxDv for Pa7 {
   fn af_eth_mii_rx_dv(&self) -> usize { 11 }
}

impl AfEthRmiiCrsDv for Pa7 {
   fn af_eth_rmii_crs_dv(&self) -> usize { 11 }
}

impl AfEventout for Pa7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA8: Pa8 = Pa8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa8 {}

impl Pin for Pa8 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 8 }
}

impl AfMco1 for Pa8 {
   fn af_mco1(&self) -> usize { 0 }
}

impl AfTim1Ch1 for Pa8 {
   fn af_tim1_ch1(&self) -> usize { 1 }
}

impl AfI2c3Scl for Pa8 {
   fn af_i2c3_scl(&self) -> usize { 4 }
}

impl AfUsart1Ck for Pa8 {
   fn af_usart1_ck(&self) -> usize { 7 }
}

impl AfOtgFsSof for Pa8 {
   fn af_otg_fs_sof(&self) -> usize { 10 }
}

impl AfLcdR6 for Pa8 {
   fn af_lcd_r6(&self) -> usize { 14 }
}

impl AfEventout for Pa8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA9: Pa9 = Pa9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa9 {}

impl Pin for Pa9 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 9 }
}

impl AfTim1Ch2 for Pa9 {
   fn af_tim1_ch2(&self) -> usize { 1 }
}

impl AfI2c3Smba for Pa9 {
   fn af_i2c3_smba(&self) -> usize { 4 }
}

impl AfUasrt1Tx for Pa9 {
   fn af_uasrt1_tx(&self) -> usize { 7 }
}

impl AfDcmiD0 for Pa9 {
   fn af_dcmi_d0(&self) -> usize { 13 }
}

impl AfEventout for Pa9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA10: Pa10 = Pa10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa10 {}

impl Pin for Pa10 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 10 }
}

impl AfTim1Ch3 for Pa10 {
   fn af_tim1_ch3(&self) -> usize { 1 }
}

impl AfUsart1Rx for Pa10 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfOtgFsId for Pa10 {
   fn af_otg_fs_id(&self) -> usize { 10 }
}

impl AfDcmiD1 for Pa10 {
   fn af_dcmi_d1(&self) -> usize { 13 }
}

impl AfEventout for Pa10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA11: Pa11 = Pa11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa11 {}

impl Pin for Pa11 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 11 }
}

impl AfTim1Ch4 for Pa11 {
   fn af_tim1_ch4(&self) -> usize { 1 }
}

impl AfUsart1Cts for Pa11 {
   fn af_usart1_cts(&self) -> usize { 7 }
}

impl AfCan1Rx for Pa11 {
   fn af_can1_rx(&self) -> usize { 9 }
}

impl AfOtgFsDm for Pa11 {
   fn af_otg_fs_dm(&self) -> usize { 10 }
}

impl AfLcdR4 for Pa11 {
   fn af_lcd_r4(&self) -> usize { 14 }
}

impl AfEventout for Pa11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA12: Pa12 = Pa12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa12 {}

impl Pin for Pa12 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 12 }
}

impl AfTim1Etr for Pa12 {
   fn af_tim1_etr(&self) -> usize { 1 }
}

impl AfUsart1Rts for Pa12 {
   fn af_usart1_rts(&self) -> usize { 7 }
}

impl AfCan1Tx for Pa12 {
   fn af_can1_tx(&self) -> usize { 9 }
}

impl AfOtgFsDp for Pa12 {
   fn af_otg_fs_dp(&self) -> usize { 10 }
}

impl AfLcdR5 for Pa12 {
   fn af_lcd_r5(&self) -> usize { 14 }
}

impl AfEventout for Pa12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA13: Pa13 = Pa13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa13 {}

impl Pin for Pa13 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 13 }
}

impl AfJtms for Pa13 {
   fn af_jtms(&self) -> usize { 0 }
}

impl AfSwdio for Pa13 {
   fn af_swdio(&self) -> usize { 0 }
}

impl AfEventout for Pa13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA14: Pa14 = Pa14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa14 {}

impl Pin for Pa14 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 14 }
}

impl AfJtck for Pa14 {
   fn af_jtck(&self) -> usize { 0 }
}

impl AfSwclk for Pa14 {
   fn af_swclk(&self) -> usize { 0 }
}

impl AfEventout for Pa14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PA15: Pa15 = Pa15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa15 {}

impl Pin for Pa15 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 15 }
}

impl AfJtdi for Pa15 {
   fn af_jtdi(&self) -> usize { 0 }
}

impl AfTim2Ch1 for Pa15 {
   fn af_tim2_ch1(&self) -> usize { 1 }
}

impl AfTim2Etr for Pa15 {
   fn af_tim2_etr(&self) -> usize { 1 }
}

impl AfSpi1Nss for Pa15 {
   fn af_spi1_nss(&self) -> usize { 5 }
}

impl AfSpi3Nss for Pa15 {
   fn af_spi3_nss(&self) -> usize { 6 }
}

impl AfI2s3Ws for Pa15 {
   fn af_i2s3_ws(&self) -> usize { 6 }
}

impl AfEventout for Pa15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin for Pb0 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 0 }
}

impl AfTim1Ch2n for Pb0 {
   fn af_tim1_ch2n(&self) -> usize { 1 }
}

impl AfTim3Ch3 for Pb0 {
   fn af_tim3_ch3(&self) -> usize { 2 }
}

impl AfTim8Ch2n for Pb0 {
   fn af_tim8_ch2n(&self) -> usize { 3 }
}

impl AfLcdR3 for Pb0 {
   fn af_lcd_r3(&self) -> usize { 9 }
}

impl AfOtgHsUlpiD1 for Pb0 {
   fn af_otg_hs_ulpi_d1(&self) -> usize { 10 }
}

impl AfEthMiiRxd2 for Pb0 {
   fn af_eth_mii_rxd2(&self) -> usize { 11 }
}

impl AfEventout for Pb0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB1: Pb1 = Pb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin for Pb1 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 1 }
}

impl AfTim1Ch3n for Pb1 {
   fn af_tim1_ch3n(&self) -> usize { 1 }
}

impl AfTim3Ch4 for Pb1 {
   fn af_tim3_ch4(&self) -> usize { 2 }
}

impl AfTim8Ch3n for Pb1 {
   fn af_tim8_ch3n(&self) -> usize { 3 }
}

impl AfLcdR6 for Pb1 {
   fn af_lcd_r6(&self) -> usize { 9 }
}

impl AfOtgHsUlpiD2 for Pb1 {
   fn af_otg_hs_ulpi_d2(&self) -> usize { 10 }
}

impl AfEthMiiRxd3 for Pb1 {
   fn af_eth_mii_rxd3(&self) -> usize { 11 }
}

impl AfEventout for Pb1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB2: Pb2 = Pb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin for Pb2 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 2 }
}

impl AfEventout for Pb2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB3: Pb3 = Pb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin for Pb3 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 3 }
}

impl AfJtdo for Pb3 {
   fn af_jtdo(&self) -> usize { 0 }
}

impl AfTraceswo for Pb3 {
   fn af_traceswo(&self) -> usize { 0 }
}

impl AfTim2Ch2 for Pb3 {
   fn af_tim2_ch2(&self) -> usize { 1 }
}

impl AfSpi1Sck for Pb3 {
   fn af_spi1_sck(&self) -> usize { 5 }
}

impl AfSpi3Sck for Pb3 {
   fn af_spi3_sck(&self) -> usize { 6 }
}

impl AfIs2cCk for Pb3 {
   fn af_is2c_ck(&self) -> usize { 6 }
}

impl AfEventout for Pb3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB4: Pb4 = Pb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin for Pb4 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 4 }
}

impl AfNjtrst for Pb4 {
   fn af_njtrst(&self) -> usize { 0 }
}

impl AfTim3Ch1 for Pb4 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfSpi1Miso for Pb4 {
   fn af_spi1_miso(&self) -> usize { 5 }
}

impl AfSpi3Miso for Pb4 {
   fn af_spi3_miso(&self) -> usize { 6 }
}

impl AfI2s3extSd for Pb4 {
   fn af_i2s3ext_sd(&self) -> usize { 7 }
}

impl AfEventout for Pb4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB5: Pb5 = Pb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin for Pb5 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 5 }
}

impl AfTim3Ch2 for Pb5 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfI2c1Smba for Pb5 {
   fn af_i2c1_smba(&self) -> usize { 4 }
}

impl AfSpi1Mosi for Pb5 {
   fn af_spi1_mosi(&self) -> usize { 5 }
}

impl AfSpi3Mosi for Pb5 {
   fn af_spi3_mosi(&self) -> usize { 6 }
}

impl AfI2s3Sd for Pb5 {
   fn af_i2s3_sd(&self) -> usize { 6 }
}

impl AfCan2Rx for Pb5 {
   fn af_can2_rx(&self) -> usize { 9 }
}

impl AfOtgHsUlpiD7 for Pb5 {
   fn af_otg_hs_ulpi_d7(&self) -> usize { 10 }
}

impl AfEthPpsOut for Pb5 {
   fn af_eth_pps_out(&self) -> usize { 11 }
}

impl AfFmcSdcke1 for Pb5 {
   fn af_fmc_sdcke1(&self) -> usize { 12 }
}

impl AfDcmiD10 for Pb5 {
   fn af_dcmi_d10(&self) -> usize { 13 }
}

impl AfEventout for Pb5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB6: Pb6 = Pb6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb6 {}

impl Pin for Pb6 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 6 }
}

impl AfTim4Ch1 for Pb6 {
   fn af_tim4_ch1(&self) -> usize { 2 }
}

impl AfI2c1Scl for Pb6 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfUsart1Tx for Pb6 {
   fn af_usart1_tx(&self) -> usize { 7 }
}

impl AfCan2Tx for Pb6 {
   fn af_can2_tx(&self) -> usize { 9 }
}

impl AfFmcSdne1 for Pb6 {
   fn af_fmc_sdne1(&self) -> usize { 12 }
}

impl AfDcmiD5 for Pb6 {
   fn af_dcmi_d5(&self) -> usize { 13 }
}

impl AfEventout for Pb6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB7: Pb7 = Pb7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb7 {}

impl Pin for Pb7 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 7 }
}

impl AfTim4Ch2 for Pb7 {
   fn af_tim4_ch2(&self) -> usize { 2 }
}

impl AfI2c1Sda for Pb7 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfUsart1Rx for Pb7 {
   fn af_usart1_rx(&self) -> usize { 7 }
}

impl AfFmcNl for Pb7 {
   fn af_fmc_nl(&self) -> usize { 12 }
}

impl AfDcmiVsync for Pb7 {
   fn af_dcmi_vsync(&self) -> usize { 13 }
}

impl AfEventout for Pb7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB8: Pb8 = Pb8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb8 {}

impl Pin for Pb8 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 8 }
}

impl AfTim4Ch3 for Pb8 {
   fn af_tim4_ch3(&self) -> usize { 2 }
}

impl AfTim10Ch1 for Pb8 {
   fn af_tim10_ch1(&self) -> usize { 3 }
}

impl AfI2c1Scl for Pb8 {
   fn af_i2c1_scl(&self) -> usize { 4 }
}

impl AfCan1Rx for Pb8 {
   fn af_can1_rx(&self) -> usize { 9 }
}

impl AfEthMiiTxd3 for Pb8 {
   fn af_eth_mii_txd3(&self) -> usize { 11 }
}

impl AfSdioD4 for Pb8 {
   fn af_sdio_d4(&self) -> usize { 12 }
}

impl AfDcmiD6 for Pb8 {
   fn af_dcmi_d6(&self) -> usize { 13 }
}

impl AfLcdB6 for Pb8 {
   fn af_lcd_b6(&self) -> usize { 14 }
}

impl AfEventout for Pb8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB9: Pb9 = Pb9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb9 {}

impl Pin for Pb9 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 9 }
}

impl AfTim4Ch4 for Pb9 {
   fn af_tim4_ch4(&self) -> usize { 2 }
}

impl AfTim11Ch1 for Pb9 {
   fn af_tim11_ch1(&self) -> usize { 3 }
}

impl AfI2c1Sda for Pb9 {
   fn af_i2c1_sda(&self) -> usize { 4 }
}

impl AfSpi2Nss for Pb9 {
   fn af_spi2_nss(&self) -> usize { 5 }
}

impl AfI2s2Ws for Pb9 {
   fn af_i2s2_ws(&self) -> usize { 5 }
}

impl AfCan1Tx for Pb9 {
   fn af_can1_tx(&self) -> usize { 9 }
}

impl AfSdioD5 for Pb9 {
   fn af_sdio_d5(&self) -> usize { 12 }
}

impl AfDcmiD7 for Pb9 {
   fn af_dcmi_d7(&self) -> usize { 13 }
}

impl AfLcdB7 for Pb9 {
   fn af_lcd_b7(&self) -> usize { 14 }
}

impl AfEventout for Pb9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB10: Pb10 = Pb10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb10 {}

impl Pin for Pb10 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 10 }
}

impl AfTim2Ch3 for Pb10 {
   fn af_tim2_ch3(&self) -> usize { 1 }
}

impl AfI2c2Scl for Pb10 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfSpi2Sck for Pb10 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pb10 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

impl AfUsart3Tx for Pb10 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfOtgHsUlpiD3 for Pb10 {
   fn af_otg_hs_ulpi_d3(&self) -> usize { 10 }
}

impl AfEthMiiRxEr for Pb10 {
   fn af_eth_mii_rx_er(&self) -> usize { 11 }
}

impl AfLcdG4 for Pb10 {
   fn af_lcd_g4(&self) -> usize { 14 }
}

impl AfEventout for Pb10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB11: Pb11 = Pb11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb11 {}

impl Pin for Pb11 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 11 }
}

impl AfSim2Ch4 for Pb11 {
   fn af_sim2_ch4(&self) -> usize { 1 }
}

impl AfI2c2Sda for Pb11 {
   fn af_i2c2_sda(&self) -> usize { 4 }
}

impl AfUsart3Rx for Pb11 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfOtgHsUlpiD4 for Pb11 {
   fn af_otg_hs_ulpi_d4(&self) -> usize { 10 }
}

impl AfEthMiiTxEn for Pb11 {
   fn af_eth_mii_tx_en(&self) -> usize { 11 }
}

impl AfEthRmiiTxEn for Pb11 {
   fn af_eth_rmii_tx_en(&self) -> usize { 11 }
}

impl AfLcdG5 for Pb11 {
   fn af_lcd_g5(&self) -> usize { 13 }
}

impl AfEventout for Pb11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB12: Pb12 = Pb12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb12 {}

impl Pin for Pb12 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 12 }
}

impl AfTim1Bkin for Pb12 {
   fn af_tim1_bkin(&self) -> usize { 1 }
}

impl AfI2c2Smba for Pb12 {
   fn af_i2c2_smba(&self) -> usize { 4 }
}

impl AfSpi2Nss for Pb12 {
   fn af_spi2_nss(&self) -> usize { 5 }
}

impl AfI2s2Ws for Pb12 {
   fn af_i2s2_ws(&self) -> usize { 5 }
}

impl AfUsart3Ck for Pb12 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

impl AfCan2Rx for Pb12 {
   fn af_can2_rx(&self) -> usize { 9 }
}

impl AfOtgHsUlpiD5 for Pb12 {
   fn af_otg_hs_ulpi_d5(&self) -> usize { 10 }
}

impl AfEthMiiTxd0 for Pb12 {
   fn af_eth_mii_txd0(&self) -> usize { 11 }
}

impl AfEthRmiiTxd0 for Pb12 {
   fn af_eth_rmii_txd0(&self) -> usize { 11 }
}

impl AfOtgHsId for Pb12 {
   fn af_otg_hs_id(&self) -> usize { 12 }
}

impl AfEventout for Pb12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB13: Pb13 = Pb13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb13 {}

impl Pin for Pb13 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 13 }
}

impl AfTim1Ch1n for Pb13 {
   fn af_tim1_ch1n(&self) -> usize { 1 }
}

impl AfSpi2Sck for Pb13 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pb13 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

impl AfUsart3Cts for Pb13 {
   fn af_usart3_cts(&self) -> usize { 7 }
}

impl AfCan2Tx for Pb13 {
   fn af_can2_tx(&self) -> usize { 9 }
}

impl AfOtgHsUlpiD6 for Pb13 {
   fn af_otg_hs_ulpi_d6(&self) -> usize { 10 }
}

impl AfEthMiiTxd1 for Pb13 {
   fn af_eth_mii_txd1(&self) -> usize { 11 }
}

impl AfEthRmiiTxd1 for Pb13 {
   fn af_eth_rmii_txd1(&self) -> usize { 11 }
}

impl AfEventout for Pb13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB14: Pb14 = Pb14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb14 {}

impl Pin for Pb14 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 14 }
}

impl AfTim2Ch2n for Pb14 {
   fn af_tim2_ch2n(&self) -> usize { 1 }
}

impl AfTim8Ch2n for Pb14 {
   fn af_tim8_ch2n(&self) -> usize { 3 }
}

impl AfSpi2Miso for Pb14 {
   fn af_spi2_miso(&self) -> usize { 5 }
}

impl AfI2s2extSd for Pb14 {
   fn af_i2s2ext_sd(&self) -> usize { 6 }
}

impl AfUsart3Rts for Pb14 {
   fn af_usart3_rts(&self) -> usize { 7 }
}

impl AfTim12Ch1 for Pb14 {
   fn af_tim12_ch1(&self) -> usize { 9 }
}

impl AfOtgHsDm for Pb14 {
   fn af_otg_hs_dm(&self) -> usize { 12 }
}

impl AfEventout for Pb14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PB15: Pb15 = Pb15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb15 {}

impl Pin for Pb15 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 15 }
}

impl AfRtcRefin for Pb15 {
   fn af_rtc_refin(&self) -> usize { 0 }
}

impl AfTim1Ch3n for Pb15 {
   fn af_tim1_ch3n(&self) -> usize { 1 }
}

impl AfTim8Ch3n for Pb15 {
   fn af_tim8_ch3n(&self) -> usize { 3 }
}

impl AfSpi2Mosi for Pb15 {
   fn af_spi2_mosi(&self) -> usize { 5 }
}

impl AfI2s2Sd for Pb15 {
   fn af_i2s2_sd(&self) -> usize { 5 }
}

impl AfTim12Ch2 for Pb15 {
   fn af_tim12_ch2(&self) -> usize { 9 }
}

impl AfOtgHsDp for Pb15 {
   fn af_otg_hs_dp(&self) -> usize { 12 }
}

impl AfEventout for Pb15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin for Pc0 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 0 }
}

impl AfOtgHsUlpiStp for Pc0 {
   fn af_otg_hs_ulpi_stp(&self) -> usize { 10 }
}

impl AfFmcSdnwe for Pc0 {
   fn af_fmc_sdnwe(&self) -> usize { 12 }
}

impl AfEventout for Pc0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC1: Pc1 = Pc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc1 {}

impl Pin for Pc1 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 1 }
}

impl AfEthMdc for Pc1 {
   fn af_eth_mdc(&self) -> usize { 11 }
}

impl AfEventout for Pc1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC2: Pc2 = Pc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc2 {}

impl Pin for Pc2 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 2 }
}

impl AfOtgHsUlpiDir for Pc2 {
   fn af_otg_hs_ulpi_dir(&self) -> usize { 10 }
}

impl AfEthMiiTxd2 for Pc2 {
   fn af_eth_mii_txd2(&self) -> usize { 11 }
}

impl AfFmcSdne0 for Pc2 {
   fn af_fmc_sdne0(&self) -> usize { 12 }
}

impl AfEventout for Pc2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC3: Pc3 = Pc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin for Pc3 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 3 }
}

impl AfOtgHsUlpiNxt for Pc3 {
   fn af_otg_hs_ulpi_nxt(&self) -> usize { 10 }
}

impl AfEthMiiTxClk for Pc3 {
   fn af_eth_mii_tx_clk(&self) -> usize { 11 }
}

impl AfFmcSdcke0 for Pc3 {
   fn af_fmc_sdcke0(&self) -> usize { 12 }
}

impl AfEventout for Pc3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC4: Pc4 = Pc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc4 {}

impl Pin for Pc4 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 4 }
}

impl AfEthMiiRxd0 for Pc4 {
   fn af_eth_mii_rxd0(&self) -> usize { 11 }
}

impl AfEthRmiiRxd0 for Pc4 {
   fn af_eth_rmii_rxd0(&self) -> usize { 11 }
}

impl AfEventout for Pc4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC5: Pc5 = Pc5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc5 {}

impl Pin for Pc5 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 5 }
}

impl AfEthMiiRxd1 for Pc5 {
   fn af_eth_mii_rxd1(&self) -> usize { 11 }
}

impl AfEthRmiiRxd1 for Pc5 {
   fn af_eth_rmii_rxd1(&self) -> usize { 11 }
}

impl AfEventout for Pc5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC6: Pc6 = Pc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc6 {}

impl Pin for Pc6 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 6 }
}

impl AfTim3Ch1 for Pc6 {
   fn af_tim3_ch1(&self) -> usize { 2 }
}

impl AfTim8Ch1 for Pc6 {
   fn af_tim8_ch1(&self) -> usize { 3 }
}

impl AfI2s2Mck for Pc6 {
   fn af_i2s2_mck(&self) -> usize { 5 }
}

impl AfUsart6Tx for Pc6 {
   fn af_usart6_tx(&self) -> usize { 8 }
}

impl AfSdioD6 for Pc6 {
   fn af_sdio_d6(&self) -> usize { 12 }
}

impl AfDcmiD0 for Pc6 {
   fn af_dcmi_d0(&self) -> usize { 13 }
}

impl AfLcdHsync for Pc6 {
   fn af_lcd_hsync(&self) -> usize { 14 }
}

impl AfEventout for Pc6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC7: Pc7 = Pc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc7 {}

impl Pin for Pc7 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 7 }
}

impl AfTim3Ch2 for Pc7 {
   fn af_tim3_ch2(&self) -> usize { 2 }
}

impl AfTim8Ch2 for Pc7 {
   fn af_tim8_ch2(&self) -> usize { 3 }
}

impl AfI2s3Mck for Pc7 {
   fn af_i2s3_mck(&self) -> usize { 6 }
}

impl AfUsart6Rx for Pc7 {
   fn af_usart6_rx(&self) -> usize { 8 }
}

impl AfSdioD7 for Pc7 {
   fn af_sdio_d7(&self) -> usize { 12 }
}

impl AfDcmiD1 for Pc7 {
   fn af_dcmi_d1(&self) -> usize { 13 }
}

impl AfLcdG6 for Pc7 {
   fn af_lcd_g6(&self) -> usize { 14 }
}

impl AfEventout for Pc7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC8: Pc8 = Pc8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc8 {}

impl Pin for Pc8 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 8 }
}

impl AfTim3Ch3 for Pc8 {
   fn af_tim3_ch3(&self) -> usize { 2 }
}

impl AfTim8Ch3 for Pc8 {
   fn af_tim8_ch3(&self) -> usize { 3 }
}

impl AfUsart6Ck for Pc8 {
   fn af_usart6_ck(&self) -> usize { 8 }
}

impl AfSdioD0 for Pc8 {
   fn af_sdio_d0(&self) -> usize { 12 }
}

impl AfDcmiD2 for Pc8 {
   fn af_dcmi_d2(&self) -> usize { 13 }
}

impl AfEventout for Pc8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC9: Pc9 = Pc9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc9 {}

impl Pin for Pc9 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 9 }
}

impl AfMco2 for Pc9 {
   fn af_mco2(&self) -> usize { 0 }
}

impl AfTim3Ch4 for Pc9 {
   fn af_tim3_ch4(&self) -> usize { 2 }
}

impl AfTim8Ch4 for Pc9 {
   fn af_tim8_ch4(&self) -> usize { 3 }
}

impl AfI2c3Sda for Pc9 {
   fn af_i2c3_sda(&self) -> usize { 4 }
}

impl AfI2sCkin for Pc9 {
   fn af_i2s_ckin(&self) -> usize { 5 }
}

impl AfSdioD1 for Pc9 {
   fn af_sdio_d1(&self) -> usize { 12 }
}

impl AfDcmiD3 for Pc9 {
   fn af_dcmi_d3(&self) -> usize { 13 }
}

impl AfEventout for Pc9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC10: Pc10 = Pc10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc10 {}

impl Pin for Pc10 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 10 }
}

impl AfSpi3Sck for Pc10 {
   fn af_spi3_sck(&self) -> usize { 6 }
}

impl AfI2s3Ck for Pc10 {
   fn af_i2s3_ck(&self) -> usize { 6 }
}

impl AfUsart3Tx for Pc10 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfSdioD2 for Pc10 {
   fn af_sdio_d2(&self) -> usize { 12 }
}

impl AfDcmiD8 for Pc10 {
   fn af_dcmi_d8(&self) -> usize { 13 }
}

impl AfLcdR2 for Pc10 {
   fn af_lcd_r2(&self) -> usize { 14 }
}

impl AfEventout for Pc10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC11: Pc11 = Pc11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc11 {}

impl Pin for Pc11 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 11 }
}

impl AfSpi3Miso for Pc11 {
   fn af_spi3_miso(&self) -> usize { 6 }
}

impl AfUsart3Rx for Pc11 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfSdioD3 for Pc11 {
   fn af_sdio_d3(&self) -> usize { 12 }
}

impl AfDcmiD4 for Pc11 {
   fn af_dcmi_d4(&self) -> usize { 13 }
}

impl AfEventout for Pc11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC12: Pc12 = Pc12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc12 {}

impl Pin for Pc12 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 12 }
}

impl AfSpi3Mosi for Pc12 {
   fn af_spi3_mosi(&self) -> usize { 6 }
}

impl AfI2s3Sd for Pc12 {
   fn af_i2s3_sd(&self) -> usize { 6 }
}

impl AfUsart3Ck for Pc12 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

impl AfSdioCk for Pc12 {
   fn af_sdio_ck(&self) -> usize { 12 }
}

impl AfDcmiD9 for Pc12 {
   fn af_dcmi_d9(&self) -> usize { 13 }
}

impl AfEventout for Pc12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC13: Pc13 = Pc13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc13 {}

impl Pin for Pc13 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 13 }
}

impl AfEventout for Pc13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC14: Pc14 = Pc14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc14 {}

impl Pin for Pc14 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 14 }
}

impl AfEventout for Pc14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PC15: Pc15 = Pc15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc15 {}

impl Pin for Pc15 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 15 }
}

impl AfEventout for Pc15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD0: Pd0 = Pd0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd0 {}

impl Pin for Pd0 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 0 }
}

impl AfCan1Rx for Pd0 {
   fn af_can1_rx(&self) -> usize { 9 }
}

impl AfFmcD2 for Pd0 {
   fn af_fmc_d2(&self) -> usize { 12 }
}

impl AfEventout for Pd0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD1: Pd1 = Pd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd1 {}

impl Pin for Pd1 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 1 }
}

impl AfCan1Tx for Pd1 {
   fn af_can1_tx(&self) -> usize { 9 }
}

impl AfFmcD3 for Pd1 {
   fn af_fmc_d3(&self) -> usize { 12 }
}

impl AfEventout for Pd1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD2: Pd2 = Pd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin for Pd2 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 2 }
}

impl AfTim3Etr for Pd2 {
   fn af_tim3_etr(&self) -> usize { 2 }
}

impl AfUart5Rx for Pd2 {
   fn af_uart5_rx(&self) -> usize { 8 }
}

impl AfSdioCmd for Pd2 {
   fn af_sdio_cmd(&self) -> usize { 12 }
}

impl AfDcmiD11 for Pd2 {
   fn af_dcmi_d11(&self) -> usize { 13 }
}

impl AfEventout for Pd2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD3: Pd3 = Pd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd3 {}

impl Pin for Pd3 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 3 }
}

impl AfSpi2Sck for Pd3 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pd3 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

impl AfUsart2Cts for Pd3 {
   fn af_usart2_cts(&self) -> usize { 7 }
}

impl AfFmcClk for Pd3 {
   fn af_fmc_clk(&self) -> usize { 12 }
}

impl AfDcmiD5 for Pd3 {
   fn af_dcmi_d5(&self) -> usize { 13 }
}

impl AfLcdG7 for Pd3 {
   fn af_lcd_g7(&self) -> usize { 14 }
}

impl AfEventout for Pd3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD4: Pd4 = Pd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd4 {}

impl Pin for Pd4 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 4 }
}

impl AfUsart2Rts for Pd4 {
   fn af_usart2_rts(&self) -> usize { 7 }
}

impl AfFmcNoe for Pd4 {
   fn af_fmc_noe(&self) -> usize { 12 }
}

impl AfEventout for Pd4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD5: Pd5 = Pd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd5 {}

impl Pin for Pd5 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 5 }
}

impl AfUsart2Tx for Pd5 {
   fn af_usart2_tx(&self) -> usize { 7 }
}

impl AfFmcNwe for Pd5 {
   fn af_fmc_nwe(&self) -> usize { 12 }
}

impl AfEventout for Pd5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD6: Pd6 = Pd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd6 {}

impl Pin for Pd6 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 6 }
}

impl AfSpi3Mosi for Pd6 {
   fn af_spi3_mosi(&self) -> usize { 5 }
}

impl AfI2s3Sd for Pd6 {
   fn af_i2s3_sd(&self) -> usize { 5 }
}

impl AfSai1SdA for Pd6 {
   fn af_sai1_sd_a(&self) -> usize { 6 }
}

impl AfUsart2Rx for Pd6 {
   fn af_usart2_rx(&self) -> usize { 7 }
}

impl AfFmcNwait for Pd6 {
   fn af_fmc_nwait(&self) -> usize { 12 }
}

impl AfDcmiD10 for Pd6 {
   fn af_dcmi_d10(&self) -> usize { 13 }
}

impl AfLcdB2 for Pd6 {
   fn af_lcd_b2(&self) -> usize { 14 }
}

impl AfEventout for Pd6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD7: Pd7 = Pd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd7 {}

impl Pin for Pd7 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 7 }
}

impl AfUsart2Ck for Pd7 {
   fn af_usart2_ck(&self) -> usize { 7 }
}

impl AfFmcNe1 for Pd7 {
   fn af_fmc_ne1(&self) -> usize { 12 }
}

impl AfFmcNce2 for Pd7 {
   fn af_fmc_nce2(&self) -> usize { 12 }
}

impl AfEventout for Pd7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD8: Pd8 = Pd8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd8 {}

impl Pin for Pd8 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 8 }
}

impl AfUsart3Tx for Pd8 {
   fn af_usart3_tx(&self) -> usize { 7 }
}

impl AfFmcD13 for Pd8 {
   fn af_fmc_d13(&self) -> usize { 12 }
}

impl AfEventout for Pd8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD9: Pd9 = Pd9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd9 {}

impl Pin for Pd9 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 9 }
}

impl AfUsart3Rx for Pd9 {
   fn af_usart3_rx(&self) -> usize { 7 }
}

impl AfFmcD14 for Pd9 {
   fn af_fmc_d14(&self) -> usize { 12 }
}

impl AfEventout for Pd9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD10: Pd10 = Pd10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd10 {}

impl Pin for Pd10 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 10 }
}

impl AfUsart3Ck for Pd10 {
   fn af_usart3_ck(&self) -> usize { 7 }
}

impl AfFmcD15 for Pd10 {
   fn af_fmc_d15(&self) -> usize { 12 }
}

impl AfEventout for Pd10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD11: Pd11 = Pd11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd11 {}

impl Pin for Pd11 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 11 }
}

impl AfUsart3Cts for Pd11 {
   fn af_usart3_cts(&self) -> usize { 7 }
}

impl AfFmcA16 for Pd11 {
   fn af_fmc_a16(&self) -> usize { 12 }
}

impl AfEventout for Pd11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD12: Pd12 = Pd12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd12 {}

impl Pin for Pd12 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 12 }
}

impl AfTim4Ch1 for Pd12 {
   fn af_tim4_ch1(&self) -> usize { 2 }
}

impl AfUsart3Rts for Pd12 {
   fn af_usart3_rts(&self) -> usize { 7 }
}

impl AfFmcA17 for Pd12 {
   fn af_fmc_a17(&self) -> usize { 12 }
}

impl AfEventout for Pd12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD13: Pd13 = Pd13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd13 {}

impl Pin for Pd13 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 13 }
}

impl AfTim4Ch2 for Pd13 {
   fn af_tim4_ch2(&self) -> usize { 2 }
}

impl AfFmcA18 for Pd13 {
   fn af_fmc_a18(&self) -> usize { 12 }
}

impl AfEventout for Pd13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD14: Pd14 = Pd14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd14 {}

impl Pin for Pd14 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 14 }
}

impl AfTim4Ch3 for Pd14 {
   fn af_tim4_ch3(&self) -> usize { 2 }
}

impl AfFmcD0 for Pd14 {
   fn af_fmc_d0(&self) -> usize { 12 }
}

impl AfEventout for Pd14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PD15: Pd15 = Pd15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd15 {}

impl Pin for Pd15 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 15 }
}

impl AfTim4Ch4 for Pd15 {
   fn af_tim4_ch4(&self) -> usize { 2 }
}

impl AfFmcD1 for Pd15 {
   fn af_fmc_d1(&self) -> usize { 12 }
}

impl AfEventout for Pd15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE0: Pe0 = Pe0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe0 {}

impl Pin for Pe0 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 0 }
}

impl AfTim4Etr for Pe0 {
   fn af_tim4_etr(&self) -> usize { 2 }
}

impl AfUart8Rx for Pe0 {
   fn af_uart8_rx(&self) -> usize { 8 }
}

impl AfFmcNbl0 for Pe0 {
   fn af_fmc_nbl0(&self) -> usize { 12 }
}

impl AfEventout for Pe0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE1: Pe1 = Pe1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe1 {}

impl Pin for Pe1 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 1 }
}

impl AfUart8Tx for Pe1 {
   fn af_uart8_tx(&self) -> usize { 8 }
}

impl AfFmcNbl1 for Pe1 {
   fn af_fmc_nbl1(&self) -> usize { 12 }
}

impl AfEventout for Pe1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE2: Pe2 = Pe2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe2 {}

impl Pin for Pe2 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 2 }
}

impl AfTraceclk for Pe2 {
   fn af_traceclk(&self) -> usize { 0 }
}

impl AfSpi4Sck for Pe2 {
   fn af_spi4_sck(&self) -> usize { 5 }
}

impl AfSai1MclkA for Pe2 {
   fn af_sai1_mclk_a(&self) -> usize { 6 }
}

impl AfEthMiiTxd3 for Pe2 {
   fn af_eth_mii_txd3(&self) -> usize { 11 }
}

impl AfFmcA23 for Pe2 {
   fn af_fmc_a23(&self) -> usize { 12 }
}

impl AfDcmiD2 for Pe2 {
   fn af_dcmi_d2(&self) -> usize { 13 }
}

impl AfEventout for Pe2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE3: Pe3 = Pe3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe3 {}

impl Pin for Pe3 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 3 }
}

impl AfTraced0 for Pe3 {
   fn af_traced0(&self) -> usize { 0 }
}

impl AfSai1SdB for Pe3 {
   fn af_sai1_sd_b(&self) -> usize { 6 }
}

impl AfFmcA19 for Pe3 {
   fn af_fmc_a19(&self) -> usize { 12 }
}

impl AfDcmiD3 for Pe3 {
   fn af_dcmi_d3(&self) -> usize { 13 }
}

impl AfEventout for Pe3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE4: Pe4 = Pe4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe4 {}

impl Pin for Pe4 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 4 }
}

impl AfTraced1 for Pe4 {
   fn af_traced1(&self) -> usize { 0 }
}

impl AfSpi4Nss for Pe4 {
   fn af_spi4_nss(&self) -> usize { 5 }
}

impl AfSai1FsA for Pe4 {
   fn af_sai1_fs_a(&self) -> usize { 6 }
}

impl AfFmcA20 for Pe4 {
   fn af_fmc_a20(&self) -> usize { 12 }
}

impl AfEventout for Pe4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE5: Pe5 = Pe5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe5 {}

impl Pin for Pe5 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 5 }
}

impl AfTraced2 for Pe5 {
   fn af_traced2(&self) -> usize { 0 }
}

impl AfTim9Ch1 for Pe5 {
   fn af_tim9_ch1(&self) -> usize { 3 }
}

impl AfSpi4Miso for Pe5 {
   fn af_spi4_miso(&self) -> usize { 5 }
}

impl AfSai1SckA for Pe5 {
   fn af_sai1_sck_a(&self) -> usize { 6 }
}

impl AfFmcA21 for Pe5 {
   fn af_fmc_a21(&self) -> usize { 12 }
}

impl AfEventout for Pe5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE6: Pe6 = Pe6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe6 {}

impl Pin for Pe6 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 6 }
}

impl AfTraced3 for Pe6 {
   fn af_traced3(&self) -> usize { 0 }
}

impl AfTim9Ch2 for Pe6 {
   fn af_tim9_ch2(&self) -> usize { 3 }
}

impl AfSpi4Mosi for Pe6 {
   fn af_spi4_mosi(&self) -> usize { 5 }
}

impl AfSai1SdA for Pe6 {
   fn af_sai1_sd_a(&self) -> usize { 6 }
}

impl AfFmcA22 for Pe6 {
   fn af_fmc_a22(&self) -> usize { 12 }
}

impl AfDcmiD4 for Pe6 {
   fn af_dcmi_d4(&self) -> usize { 13 }
}

impl AfEventout for Pe6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE7: Pe7 = Pe7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe7 {}

impl Pin for Pe7 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 7 }
}

impl AfTim1Etr for Pe7 {
   fn af_tim1_etr(&self) -> usize { 1 }
}

impl AfFmcD4 for Pe7 {
   fn af_fmc_d4(&self) -> usize { 12 }
}

impl AfDcmiD6 for Pe7 {
   fn af_dcmi_d6(&self) -> usize { 13 }
}

impl AfEventout for Pe7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE8: Pe8 = Pe8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe8 {}

impl Pin for Pe8 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 8 }
}

impl AfTim1Ch1n for Pe8 {
   fn af_tim1_ch1n(&self) -> usize { 1 }
}

impl AfFmcD5 for Pe8 {
   fn af_fmc_d5(&self) -> usize { 12 }
}

impl AfDcmiD7 for Pe8 {
   fn af_dcmi_d7(&self) -> usize { 13 }
}

impl AfEventout for Pe8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE9: Pe9 = Pe9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe9 {}

impl Pin for Pe9 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 9 }
}

impl AfTim1Ch1 for Pe9 {
   fn af_tim1_ch1(&self) -> usize { 1 }
}

impl AfFmcD6 for Pe9 {
   fn af_fmc_d6(&self) -> usize { 12 }
}

impl AfEventout for Pe9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE10: Pe10 = Pe10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe10 {}

impl Pin for Pe10 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 10 }
}

impl AfTim1Ch2n for Pe10 {
   fn af_tim1_ch2n(&self) -> usize { 1 }
}

impl AfFmcD7 for Pe10 {
   fn af_fmc_d7(&self) -> usize { 12 }
}

impl AfEventout for Pe10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE11: Pe11 = Pe11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe11 {}

impl Pin for Pe11 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 11 }
}

impl AfTim1Ch2 for Pe11 {
   fn af_tim1_ch2(&self) -> usize { 1 }
}

impl AfSpi4Nss for Pe11 {
   fn af_spi4_nss(&self) -> usize { 5 }
}

impl AfFmcD8 for Pe11 {
   fn af_fmc_d8(&self) -> usize { 12 }
}

impl AfLcdG3 for Pe11 {
   fn af_lcd_g3(&self) -> usize { 14 }
}

impl AfEventout for Pe11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE12: Pe12 = Pe12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe12 {}

impl Pin for Pe12 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 12 }
}

impl AfTim1Ch3n for Pe12 {
   fn af_tim1_ch3n(&self) -> usize { 1 }
}

impl AfSpi4Sck for Pe12 {
   fn af_spi4_sck(&self) -> usize { 5 }
}

impl AfFmcD9 for Pe12 {
   fn af_fmc_d9(&self) -> usize { 12 }
}

impl AfLcdB4 for Pe12 {
   fn af_lcd_b4(&self) -> usize { 14 }
}

impl AfEventout for Pe12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE13: Pe13 = Pe13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe13 {}

impl Pin for Pe13 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 13 }
}

impl AfTim1Ch3 for Pe13 {
   fn af_tim1_ch3(&self) -> usize { 1 }
}

impl AfSpi4Miso for Pe13 {
   fn af_spi4_miso(&self) -> usize { 5 }
}

impl AfFmcD10 for Pe13 {
   fn af_fmc_d10(&self) -> usize { 12 }
}

impl AfLcdDe for Pe13 {
   fn af_lcd_de(&self) -> usize { 14 }
}

impl AfEventout for Pe13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE14: Pe14 = Pe14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe14 {}

impl Pin for Pe14 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 14 }
}

impl AfTim1Ch4 for Pe14 {
   fn af_tim1_ch4(&self) -> usize { 1 }
}

impl AfSpi4Mosi for Pe14 {
   fn af_spi4_mosi(&self) -> usize { 5 }
}

impl AfFmcD11 for Pe14 {
   fn af_fmc_d11(&self) -> usize { 12 }
}

impl AfLcdClk for Pe14 {
   fn af_lcd_clk(&self) -> usize { 14 }
}

impl AfEventout for Pe14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PE15: Pe15 = Pe15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe15 {}

impl Pin for Pe15 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 15 }
}

impl AfTim1Bkin for Pe15 {
   fn af_tim1_bkin(&self) -> usize { 1 }
}

impl AfFmcD12 for Pe15 {
   fn af_fmc_d12(&self) -> usize { 12 }
}

impl AfLcdR7 for Pe15 {
   fn af_lcd_r7(&self) -> usize { 14 }
}

impl AfEventout for Pe15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF0: Pf0 = Pf0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin for Pf0 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 0 }
}

impl AfI2c2Sda for Pf0 {
   fn af_i2c2_sda(&self) -> usize { 4 }
}

impl AfFmcA0 for Pf0 {
   fn af_fmc_a0(&self) -> usize { 12 }
}

impl AfEventout for Pf0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF1: Pf1 = Pf1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf1 {}

impl Pin for Pf1 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 1 }
}

impl AfI2c2Scl for Pf1 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfFmcA1 for Pf1 {
   fn af_fmc_a1(&self) -> usize { 12 }
}

impl AfEventout for Pf1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF2: Pf2 = Pf2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf2 {}

impl Pin for Pf2 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 2 }
}

impl AfI2c2Smba for Pf2 {
   fn af_i2c2_smba(&self) -> usize { 4 }
}

impl AfFmcA2 for Pf2 {
   fn af_fmc_a2(&self) -> usize { 12 }
}

impl AfEventout for Pf2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF3: Pf3 = Pf3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf3 {}

impl Pin for Pf3 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 3 }
}

impl AfFmcA3 for Pf3 {
   fn af_fmc_a3(&self) -> usize { 12 }
}

impl AfEventout for Pf3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF4: Pf4 = Pf4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf4 {}

impl Pin for Pf4 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 4 }
}

impl AfFmcA4 for Pf4 {
   fn af_fmc_a4(&self) -> usize { 12 }
}

impl AfEventout for Pf4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF5: Pf5 = Pf5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf5 {}

impl Pin for Pf5 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 5 }
}

impl AfFmcA5 for Pf5 {
   fn af_fmc_a5(&self) -> usize { 12 }
}

impl AfEventout for Pf5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF6: Pf6 = Pf6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf6 {}

impl Pin for Pf6 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 6 }
}

impl AfTim10Ch1 for Pf6 {
   fn af_tim10_ch1(&self) -> usize { 3 }
}

impl AfSpi5Nss for Pf6 {
   fn af_spi5_nss(&self) -> usize { 5 }
}

impl AfSai1SdB for Pf6 {
   fn af_sai1_sd_b(&self) -> usize { 6 }
}

impl AfUart7Rx for Pf6 {
   fn af_uart7_rx(&self) -> usize { 8 }
}

impl AfFmcNiord for Pf6 {
   fn af_fmc_niord(&self) -> usize { 12 }
}

impl AfEventout for Pf6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF7: Pf7 = Pf7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf7 {}

impl Pin for Pf7 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 7 }
}

impl AfTim11Ch1 for Pf7 {
   fn af_tim11_ch1(&self) -> usize { 3 }
}

impl AfSpi5Sck for Pf7 {
   fn af_spi5_sck(&self) -> usize { 5 }
}

impl AfSai1MclkB for Pf7 {
   fn af_sai1_mclk_b(&self) -> usize { 6 }
}

impl AfUart7Tx for Pf7 {
   fn af_uart7_tx(&self) -> usize { 8 }
}

impl AfFmcNreg for Pf7 {
   fn af_fmc_nreg(&self) -> usize { 12 }
}

impl AfEventout for Pf7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF8: Pf8 = Pf8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf8 {}

impl Pin for Pf8 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 8 }
}

impl AfSpi5Miso for Pf8 {
   fn af_spi5_miso(&self) -> usize { 5 }
}

impl AfSaiSckB for Pf8 {
   fn af_sai_sck_b(&self) -> usize { 6 }
}

impl AfTim13Ch1 for Pf8 {
   fn af_tim13_ch1(&self) -> usize { 9 }
}

impl AfFmcNiowr for Pf8 {
   fn af_fmc_niowr(&self) -> usize { 12 }
}

impl AfEventout for Pf8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF9: Pf9 = Pf9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf9 {}

impl Pin for Pf9 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 9 }
}

impl AfSpi5Mosi for Pf9 {
   fn af_spi5_mosi(&self) -> usize { 5 }
}

impl AfSaiFsB for Pf9 {
   fn af_sai_fs_b(&self) -> usize { 6 }
}

impl AfTim14Ch1 for Pf9 {
   fn af_tim14_ch1(&self) -> usize { 9 }
}

impl AfFmcCd for Pf9 {
   fn af_fmc_cd(&self) -> usize { 12 }
}

impl AfEventout for Pf9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF10: Pf10 = Pf10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf10 {}

impl Pin for Pf10 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 10 }
}

impl AfFmcIntr for Pf10 {
   fn af_fmc_intr(&self) -> usize { 12 }
}

impl AfDcmiD11 for Pf10 {
   fn af_dcmi_d11(&self) -> usize { 13 }
}

impl AfLcdDe for Pf10 {
   fn af_lcd_de(&self) -> usize { 14 }
}

impl AfEventout for Pf10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF11: Pf11 = Pf11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf11 {}

impl Pin for Pf11 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 11 }
}

impl AfSpi5Mosi for Pf11 {
   fn af_spi5_mosi(&self) -> usize { 5 }
}

impl AfFmcSdnras for Pf11 {
   fn af_fmc_sdnras(&self) -> usize { 12 }
}

impl AfDcmiD12 for Pf11 {
   fn af_dcmi_d12(&self) -> usize { 13 }
}

impl AfEventout for Pf11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF12: Pf12 = Pf12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf12 {}

impl Pin for Pf12 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 12 }
}

impl AfFmcA6 for Pf12 {
   fn af_fmc_a6(&self) -> usize { 12 }
}

impl AfEventout for Pf12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF13: Pf13 = Pf13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf13 {}

impl Pin for Pf13 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 13 }
}

impl AfFmcA7 for Pf13 {
   fn af_fmc_a7(&self) -> usize { 12 }
}

impl AfEventout for Pf13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF14: Pf14 = Pf14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf14 {}

impl Pin for Pf14 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 14 }
}

impl AfFmcA8 for Pf14 {
   fn af_fmc_a8(&self) -> usize { 12 }
}

impl AfEventout for Pf14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PF15: Pf15 = Pf15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf15 {}

impl Pin for Pf15 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 15 }
}

impl AfFmcA9 for Pf15 {
   fn af_fmc_a9(&self) -> usize { 12 }
}

impl AfEventout for Pf15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG0: Pg0 = Pg0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg0 {}

impl Pin for Pg0 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 0 }
}

impl AfFmcA10 for Pg0 {
   fn af_fmc_a10(&self) -> usize { 12 }
}

impl AfEventout for Pg0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG1: Pg1 = Pg1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg1 {}

impl Pin for Pg1 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 1 }
}

impl AfFmcA11 for Pg1 {
   fn af_fmc_a11(&self) -> usize { 12 }
}

impl AfEventout for Pg1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG2: Pg2 = Pg2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg2 {}

impl Pin for Pg2 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 2 }
}

impl AfFmcA12 for Pg2 {
   fn af_fmc_a12(&self) -> usize { 12 }
}

impl AfEventout for Pg2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG3: Pg3 = Pg3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg3 {}

impl Pin for Pg3 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 3 }
}

impl AfFmcA13 for Pg3 {
   fn af_fmc_a13(&self) -> usize { 12 }
}

impl AfEventout for Pg3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG4: Pg4 = Pg4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg4 {}

impl Pin for Pg4 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 4 }
}

impl AfFmcA14 for Pg4 {
   fn af_fmc_a14(&self) -> usize { 12 }
}

impl AfFmcBa0 for Pg4 {
   fn af_fmc_ba0(&self) -> usize { 12 }
}

impl AfEventout for Pg4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG5: Pg5 = Pg5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg5 {}

impl Pin for Pg5 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 5 }
}

impl AfFmcA15 for Pg5 {
   fn af_fmc_a15(&self) -> usize { 12 }
}

impl AfFmcBa1 for Pg5 {
   fn af_fmc_ba1(&self) -> usize { 12 }
}

impl AfEventout for Pg5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG6: Pg6 = Pg6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg6 {}

impl Pin for Pg6 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 6 }
}

impl AfFmcInt2 for Pg6 {
   fn af_fmc_int2(&self) -> usize { 12 }
}

impl AfDcmiD12 for Pg6 {
   fn af_dcmi_d12(&self) -> usize { 13 }
}

impl AfLcdR7 for Pg6 {
   fn af_lcd_r7(&self) -> usize { 14 }
}

impl AfEventout for Pg6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG7: Pg7 = Pg7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg7 {}

impl Pin for Pg7 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 7 }
}

impl AfUsart6Ck for Pg7 {
   fn af_usart6_ck(&self) -> usize { 8 }
}

impl AfFmcInt3 for Pg7 {
   fn af_fmc_int3(&self) -> usize { 12 }
}

impl AfDcmiD13 for Pg7 {
   fn af_dcmi_d13(&self) -> usize { 13 }
}

impl AfLcdClk for Pg7 {
   fn af_lcd_clk(&self) -> usize { 14 }
}

impl AfEventout for Pg7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG8: Pg8 = Pg8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg8 {}

impl Pin for Pg8 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 8 }
}

impl AfSpi6Nss for Pg8 {
   fn af_spi6_nss(&self) -> usize { 5 }
}

impl AfUsart6Rts for Pg8 {
   fn af_usart6_rts(&self) -> usize { 8 }
}

impl AfEthPsOut for Pg8 {
   fn af_eth_ps_out(&self) -> usize { 11 }
}

impl AfFmcSdclk for Pg8 {
   fn af_fmc_sdclk(&self) -> usize { 12 }
}

impl AfEventout for Pg8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG9: Pg9 = Pg9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg9 {}

impl Pin for Pg9 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 9 }
}

impl AfUsart6Rx for Pg9 {
   fn af_usart6_rx(&self) -> usize { 8 }
}

impl AfFmcNe2 for Pg9 {
   fn af_fmc_ne2(&self) -> usize { 12 }
}

impl AfFmcNce3 for Pg9 {
   fn af_fmc_nce3(&self) -> usize { 12 }
}

impl AfDcmiVsync for Pg9 {
   fn af_dcmi_vsync(&self) -> usize { 13 }
}

impl AfEventout for Pg9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG10: Pg10 = Pg10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg10 {}

impl Pin for Pg10 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 10 }
}

impl AfLcdG3 for Pg10 {
   fn af_lcd_g3(&self) -> usize { 9 }
}

impl AfFmcNce41 for Pg10 {
   fn af_fmc_nce4_1(&self) -> usize { 12 }
}

impl AfFmcNe3 for Pg10 {
   fn af_fmc_ne3(&self) -> usize { 12 }
}

impl AfDcmiD2 for Pg10 {
   fn af_dcmi_d2(&self) -> usize { 13 }
}

impl AfLcdB2 for Pg10 {
   fn af_lcd_b2(&self) -> usize { 14 }
}

impl AfEventout for Pg10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG11: Pg11 = Pg11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg11 {}

impl Pin for Pg11 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 11 }
}

impl AfSpi6Miso for Pg11 {
   fn af_spi6_miso(&self) -> usize { 5 }
}

impl AfUsart6Rts for Pg11 {
   fn af_usart6_rts(&self) -> usize { 8 }
}

impl AfEthMiiTxEn for Pg11 {
   fn af_eth_mii_tx_en(&self) -> usize { 11 }
}

impl AfEthRmiiTxEn for Pg11 {
   fn af_eth_rmii_tx_en(&self) -> usize { 11 }
}

impl AfFmcNce42 for Pg11 {
   fn af_fmc_nce4_2(&self) -> usize { 12 }
}

impl AfDcmiD3 for Pg11 {
   fn af_dcmi_d3(&self) -> usize { 13 }
}

impl AfLcdB3 for Pg11 {
   fn af_lcd_b3(&self) -> usize { 14 }
}

impl AfEventout for Pg11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG12: Pg12 = Pg12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg12 {}

impl Pin for Pg12 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 12 }
}

impl AfSpi6Sck for Pg12 {
   fn af_spi6_sck(&self) -> usize { 5 }
}

impl AfUsart6Cts for Pg12 {
   fn af_usart6_cts(&self) -> usize { 8 }
}

impl AfLcdB4 for Pg12 {
   fn af_lcd_b4(&self) -> usize { 9 }
}

impl AfFmcNe4 for Pg12 {
   fn af_fmc_ne4(&self) -> usize { 12 }
}

impl AfLcdB1 for Pg12 {
   fn af_lcd_b1(&self) -> usize { 14 }
}

impl AfEventout for Pg12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG13: Pg13 = Pg13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg13 {}

impl Pin for Pg13 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 13 }
}

impl AfSpi6Mosi for Pg13 {
   fn af_spi6_mosi(&self) -> usize { 5 }
}

impl AfUsart6Tx for Pg13 {
   fn af_usart6_tx(&self) -> usize { 8 }
}

impl AfEthMiiTxd0 for Pg13 {
   fn af_eth_mii_txd0(&self) -> usize { 11 }
}

impl AfEthRmiiTxd0 for Pg13 {
   fn af_eth_rmii_txd0(&self) -> usize { 11 }
}

impl AfFmcA24 for Pg13 {
   fn af_fmc_a24(&self) -> usize { 12 }
}

impl AfEventout for Pg13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG14: Pg14 = Pg14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg14 {}

impl Pin for Pg14 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 14 }
}

impl AfUsart6Cts for Pg14 {
   fn af_usart6_cts(&self) -> usize { 8 }
}

impl AfEthMiiTxd1 for Pg14 {
   fn af_eth_mii_txd1(&self) -> usize { 11 }
}

impl AfEthRmiiTxd1 for Pg14 {
   fn af_eth_rmii_txd1(&self) -> usize { 11 }
}

impl AfFmcA25 for Pg14 {
   fn af_fmc_a25(&self) -> usize { 12 }
}

impl AfEventout for Pg14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PG15: Pg15 = Pg15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg15 {}

impl Pin for Pg15 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 15 }
}

impl AfFmcSndcas for Pg15 {
   fn af_fmc_sndcas(&self) -> usize { 12 }
}

impl AfDcmiD13 for Pg15 {
   fn af_dcmi_d13(&self) -> usize { 13 }
}

impl AfEventout for Pg15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH0: Ph0 = Ph0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin for Ph0 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 0 }
}

impl AfEventout for Ph0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH1: Ph1 = Ph1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin for Ph1 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 1 }
}

impl AfEventout for Ph1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH2: Ph2 = Ph2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph2 {}

impl Pin for Ph2 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 2 }
}

impl AfEthMiiCrs for Ph2 {
   fn af_eth_mii_crs(&self) -> usize { 11 }
}

impl AfFmcSdcke0 for Ph2 {
   fn af_fmc_sdcke0(&self) -> usize { 12 }
}

impl AfLcdR0 for Ph2 {
   fn af_lcd_r0(&self) -> usize { 14 }
}

impl AfEventout for Ph2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH3: Ph3 = Ph3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph3 {}

impl Pin for Ph3 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 3 }
}

impl AfEthMiiCol for Ph3 {
   fn af_eth_mii_col(&self) -> usize { 11 }
}

impl AfFmcSdne0 for Ph3 {
   fn af_fmc_sdne0(&self) -> usize { 12 }
}

impl AfLcdR1 for Ph3 {
   fn af_lcd_r1(&self) -> usize { 14 }
}

impl AfEventout for Ph3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH4: Ph4 = Ph4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph4 {}

impl Pin for Ph4 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 4 }
}

impl AfI2c2Scl for Ph4 {
   fn af_i2c2_scl(&self) -> usize { 4 }
}

impl AfOtgHsUlpiNxt for Ph4 {
   fn af_otg_hs_ulpi_nxt(&self) -> usize { 10 }
}

impl AfEventout for Ph4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH5: Ph5 = Ph5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph5 {}

impl Pin for Ph5 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 5 }
}

impl AfI2c2Sda for Ph5 {
   fn af_i2c2_sda(&self) -> usize { 4 }
}

impl AfSpi5Nss for Ph5 {
   fn af_spi5_nss(&self) -> usize { 5 }
}

impl AfFmcSdnwe for Ph5 {
   fn af_fmc_sdnwe(&self) -> usize { 12 }
}

impl AfEventout for Ph5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH6: Ph6 = Ph6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph6 {}

impl Pin for Ph6 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 6 }
}

impl AfI2c2Smba for Ph6 {
   fn af_i2c2_smba(&self) -> usize { 4 }
}

impl AfSpi5Sck for Ph6 {
   fn af_spi5_sck(&self) -> usize { 5 }
}

impl AfTim12Ch1 for Ph6 {
   fn af_tim12_ch1(&self) -> usize { 9 }
}

impl AfFmcSdne1 for Ph6 {
   fn af_fmc_sdne1(&self) -> usize { 12 }
}

impl AfDcmiD8 for Ph6 {
   fn af_dcmi_d8(&self) -> usize { 13 }
}

impl AfEventout for Ph6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH7: Ph7 = Ph7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph7 {}

impl Pin for Ph7 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 7 }
}

impl AfI2c3Scl for Ph7 {
   fn af_i2c3_scl(&self) -> usize { 4 }
}

impl AfSpi5Miso for Ph7 {
   fn af_spi5_miso(&self) -> usize { 5 }
}

impl AfEthMiiRxd3 for Ph7 {
   fn af_eth_mii_rxd3(&self) -> usize { 11 }
}

impl AfFmcSdcke1 for Ph7 {
   fn af_fmc_sdcke1(&self) -> usize { 12 }
}

impl AfDcmiD9 for Ph7 {
   fn af_dcmi_d9(&self) -> usize { 13 }
}

impl AfEventout for Ph7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH8: Ph8 = Ph8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph8 {}

impl Pin for Ph8 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 8 }
}

impl AfI2c3Sda for Ph8 {
   fn af_i2c3_sda(&self) -> usize { 4 }
}

impl AfFmcD16 for Ph8 {
   fn af_fmc_d16(&self) -> usize { 12 }
}

impl AfDcmiHsync for Ph8 {
   fn af_dcmi_hsync(&self) -> usize { 13 }
}

impl AfLcdR2 for Ph8 {
   fn af_lcd_r2(&self) -> usize { 14 }
}

impl AfEventout for Ph8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH9: Ph9 = Ph9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph9 {}

impl Pin for Ph9 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 9 }
}

impl AfI2c3Smba for Ph9 {
   fn af_i2c3_smba(&self) -> usize { 4 }
}

impl AfTim2Ch2 for Ph9 {
   fn af_tim2_ch2(&self) -> usize { 9 }
}

impl AfFmcD17 for Ph9 {
   fn af_fmc_d17(&self) -> usize { 12 }
}

impl AfDcmiD0 for Ph9 {
   fn af_dcmi_d0(&self) -> usize { 13 }
}

impl AfLcdR3 for Ph9 {
   fn af_lcd_r3(&self) -> usize { 14 }
}

impl AfEventout for Ph9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH10: Ph10 = Ph10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph10 {}

impl Pin for Ph10 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 10 }
}

impl AfTim5Ch1 for Ph10 {
   fn af_tim5_ch1(&self) -> usize { 2 }
}

impl AfFmcD18 for Ph10 {
   fn af_fmc_d18(&self) -> usize { 12 }
}

impl AfDcmiD1 for Ph10 {
   fn af_dcmi_d1(&self) -> usize { 13 }
}

impl AfLcdR4 for Ph10 {
   fn af_lcd_r4(&self) -> usize { 14 }
}

impl AfEventout for Ph10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH11: Ph11 = Ph11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph11 {}

impl Pin for Ph11 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 11 }
}

impl AfTim5Ch2 for Ph11 {
   fn af_tim5_ch2(&self) -> usize { 2 }
}

impl AfFmcD19 for Ph11 {
   fn af_fmc_d19(&self) -> usize { 12 }
}

impl AfDcmiD2 for Ph11 {
   fn af_dcmi_d2(&self) -> usize { 13 }
}

impl AfLcdR5 for Ph11 {
   fn af_lcd_r5(&self) -> usize { 14 }
}

impl AfEventout for Ph11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH12: Ph12 = Ph12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph12 {}

impl Pin for Ph12 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 12 }
}

impl AfTim5Ch3 for Ph12 {
   fn af_tim5_ch3(&self) -> usize { 2 }
}

impl AfFmcD20 for Ph12 {
   fn af_fmc_d20(&self) -> usize { 12 }
}

impl AfDcmiD3 for Ph12 {
   fn af_dcmi_d3(&self) -> usize { 13 }
}

impl AfLcdR6 for Ph12 {
   fn af_lcd_r6(&self) -> usize { 14 }
}

impl AfEventout for Ph12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH13: Ph13 = Ph13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph13 {}

impl Pin for Ph13 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 13 }
}

impl AfTim8Ch1n for Ph13 {
   fn af_tim8_ch1n(&self) -> usize { 3 }
}

impl AfCan1Tx for Ph13 {
   fn af_can1_tx(&self) -> usize { 9 }
}

impl AfFmcD21 for Ph13 {
   fn af_fmc_d21(&self) -> usize { 12 }
}

impl AfLcdG2 for Ph13 {
   fn af_lcd_g2(&self) -> usize { 14 }
}

impl AfEventout for Ph13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH14: Ph14 = Ph14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph14 {}

impl Pin for Ph14 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 14 }
}

impl AfTim8Ch2n for Ph14 {
   fn af_tim8_ch2n(&self) -> usize { 3 }
}

impl AfFmcD22 for Ph14 {
   fn af_fmc_d22(&self) -> usize { 12 }
}

impl AfDcmiD4 for Ph14 {
   fn af_dcmi_d4(&self) -> usize { 13 }
}

impl AfLcdG3 for Ph14 {
   fn af_lcd_g3(&self) -> usize { 14 }
}

impl AfEventout for Ph14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PH15: Ph15 = Ph15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph15 {}

impl Pin for Ph15 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 15 }
}

impl AfTim8Ch3n for Ph15 {
   fn af_tim8_ch3n(&self) -> usize { 3 }
}

impl AfFmcD23 for Ph15 {
   fn af_fmc_d23(&self) -> usize { 12 }
}

impl AfDcmiD11 for Ph15 {
   fn af_dcmi_d11(&self) -> usize { 13 }
}

impl AfLcdG4 for Ph15 {
   fn af_lcd_g4(&self) -> usize { 14 }
}

impl AfEventout for Ph15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI0: Pi0 = Pi0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi0 {}

impl Pin for Pi0 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 0 }
}

impl AfTim5Ch4 for Pi0 {
   fn af_tim5_ch4(&self) -> usize { 2 }
}

impl AfSpi2Nss for Pi0 {
   fn af_spi2_nss(&self) -> usize { 5 }
}

impl AfI2s2Ws for Pi0 {
   fn af_i2s2_ws(&self) -> usize { 5 }
}

impl AfFmcD24 for Pi0 {
   fn af_fmc_d24(&self) -> usize { 12 }
}

impl AfDcmiD13 for Pi0 {
   fn af_dcmi_d13(&self) -> usize { 13 }
}

impl AfLcdG5 for Pi0 {
   fn af_lcd_g5(&self) -> usize { 14 }
}

impl AfEventout for Pi0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI1: Pi1 = Pi1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi1 {}

impl Pin for Pi1 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 1 }
}

impl AfSpi2Sck for Pi1 {
   fn af_spi2_sck(&self) -> usize { 5 }
}

impl AfI2s2Ck for Pi1 {
   fn af_i2s2_ck(&self) -> usize { 5 }
}

impl AfFmcD25 for Pi1 {
   fn af_fmc_d25(&self) -> usize { 12 }
}

impl AfDcmiD8 for Pi1 {
   fn af_dcmi_d8(&self) -> usize { 13 }
}

impl AfLcdG6 for Pi1 {
   fn af_lcd_g6(&self) -> usize { 14 }
}

impl AfEventout for Pi1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI2: Pi2 = Pi2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi2 {}

impl Pin for Pi2 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 2 }
}

impl AfTim8Ch4 for Pi2 {
   fn af_tim8_ch4(&self) -> usize { 3 }
}

impl AfSpi2Miso for Pi2 {
   fn af_spi2_miso(&self) -> usize { 5 }
}

impl AfI2s2extSd for Pi2 {
   fn af_i2s2ext_sd(&self) -> usize { 6 }
}

impl AfFmcD26 for Pi2 {
   fn af_fmc_d26(&self) -> usize { 12 }
}

impl AfDcmiD9 for Pi2 {
   fn af_dcmi_d9(&self) -> usize { 13 }
}

impl AfLcdG7 for Pi2 {
   fn af_lcd_g7(&self) -> usize { 14 }
}

impl AfEventout for Pi2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI3: Pi3 = Pi3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi3 {}

impl Pin for Pi3 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 3 }
}

impl AfTim8Etr for Pi3 {
   fn af_tim8_etr(&self) -> usize { 3 }
}

impl AfSpi2Mosi for Pi3 {
   fn af_spi2_mosi(&self) -> usize { 5 }
}

impl AfI2s2Sd for Pi3 {
   fn af_i2s2_sd(&self) -> usize { 5 }
}

impl AfFmcD27 for Pi3 {
   fn af_fmc_d27(&self) -> usize { 12 }
}

impl AfDcmiD10 for Pi3 {
   fn af_dcmi_d10(&self) -> usize { 13 }
}

impl AfEventout for Pi3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI4: Pi4 = Pi4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi4 {}

impl Pin for Pi4 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 4 }
}

impl AfTim8Bkin for Pi4 {
   fn af_tim8_bkin(&self) -> usize { 3 }
}

impl AfFmcNbl2 for Pi4 {
   fn af_fmc_nbl2(&self) -> usize { 12 }
}

impl AfDcmiD5 for Pi4 {
   fn af_dcmi_d5(&self) -> usize { 13 }
}

impl AfLcdB4 for Pi4 {
   fn af_lcd_b4(&self) -> usize { 14 }
}

impl AfEventout for Pi4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI5: Pi5 = Pi5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi5 {}

impl Pin for Pi5 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 5 }
}

impl AfTim8Ch1 for Pi5 {
   fn af_tim8_ch1(&self) -> usize { 3 }
}

impl AfFmcNbl3 for Pi5 {
   fn af_fmc_nbl3(&self) -> usize { 12 }
}

impl AfDcmVsync for Pi5 {
   fn af_dcm_vsync(&self) -> usize { 13 }
}

impl AfLcdB5 for Pi5 {
   fn af_lcd_b5(&self) -> usize { 14 }
}

impl AfEventout for Pi5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI6: Pi6 = Pi6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi6 {}

impl Pin for Pi6 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 6 }
}

impl AfTim8Ch2 for Pi6 {
   fn af_tim8_ch2(&self) -> usize { 3 }
}

impl AfFmcD28 for Pi6 {
   fn af_fmc_d28(&self) -> usize { 12 }
}

impl AfDcmiD6 for Pi6 {
   fn af_dcmi_d6(&self) -> usize { 13 }
}

impl AfLcdB6 for Pi6 {
   fn af_lcd_b6(&self) -> usize { 14 }
}

impl AfEventout for Pi6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI7: Pi7 = Pi7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi7 {}

impl Pin for Pi7 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 7 }
}

impl AfTim8Ch3 for Pi7 {
   fn af_tim8_ch3(&self) -> usize { 3 }
}

impl AfFmcD29 for Pi7 {
   fn af_fmc_d29(&self) -> usize { 12 }
}

impl AfDcmiD7 for Pi7 {
   fn af_dcmi_d7(&self) -> usize { 13 }
}

impl AfLcdB7 for Pi7 {
   fn af_lcd_b7(&self) -> usize { 14 }
}

impl AfEventout for Pi7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI8: Pi8 = Pi8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi8 {}

impl Pin for Pi8 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 8 }
}

impl AfEventout for Pi8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI9: Pi9 = Pi9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi9 {}

impl Pin for Pi9 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 9 }
}

impl AfCan1Rx for Pi9 {
   fn af_can1_rx(&self) -> usize { 9 }
}

impl AfFmcD30 for Pi9 {
   fn af_fmc_d30(&self) -> usize { 12 }
}

impl AfLcdVsync for Pi9 {
   fn af_lcd_vsync(&self) -> usize { 14 }
}

impl AfEventout for Pi9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI10: Pi10 = Pi10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi10 {}

impl Pin for Pi10 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 10 }
}

impl AfEthMiiRxEr for Pi10 {
   fn af_eth_mii_rx_er(&self) -> usize { 11 }
}

impl AfFmcD31 for Pi10 {
   fn af_fmc_d31(&self) -> usize { 12 }
}

impl AfLcdHsync for Pi10 {
   fn af_lcd_hsync(&self) -> usize { 14 }
}

impl AfEventout for Pi10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI11: Pi11 = Pi11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi11 {}

impl Pin for Pi11 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 11 }
}

impl AfOtgHsUlpiDir for Pi11 {
   fn af_otg_hs_ulpi_dir(&self) -> usize { 10 }
}

impl AfEventout for Pi11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI12: Pi12 = Pi12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi12 {}

impl Pin for Pi12 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 12 }
}

impl AfLcdHsync for Pi12 {
   fn af_lcd_hsync(&self) -> usize { 14 }
}

impl AfEventout for Pi12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI13: Pi13 = Pi13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi13 {}

impl Pin for Pi13 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 13 }
}

impl AfLcdVsync for Pi13 {
   fn af_lcd_vsync(&self) -> usize { 14 }
}

impl AfEventout for Pi13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI14: Pi14 = Pi14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi14 {}

impl Pin for Pi14 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 14 }
}

impl AfLcdClk for Pi14 {
   fn af_lcd_clk(&self) -> usize { 14 }
}

impl AfEventout for Pi14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PI15: Pi15 = Pi15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pi15 {}

impl Pin for Pi15 {
   fn port(&self) -> Gpio { GPIOI }
   fn index(&self) -> usize { 15 }
}

impl AfLcdR0 for Pi15 {
   fn af_lcd_r0(&self) -> usize { 14 }
}

impl AfEventout for Pi15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ0: Pj0 = Pj0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj0 {}

impl Pin for Pj0 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 0 }
}

impl AfLcdR1 for Pj0 {
   fn af_lcd_r1(&self) -> usize { 14 }
}

impl AfEventout for Pj0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ1: Pj1 = Pj1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj1 {}

impl Pin for Pj1 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 1 }
}

impl AfLcdR2 for Pj1 {
   fn af_lcd_r2(&self) -> usize { 14 }
}

impl AfEventout for Pj1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ2: Pj2 = Pj2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj2 {}

impl Pin for Pj2 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 2 }
}

impl AfLcdR3 for Pj2 {
   fn af_lcd_r3(&self) -> usize { 14 }
}

impl AfEventout for Pj2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ3: Pj3 = Pj3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj3 {}

impl Pin for Pj3 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 3 }
}

impl AfLcdR4 for Pj3 {
   fn af_lcd_r4(&self) -> usize { 14 }
}

impl AfEventout for Pj3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ4: Pj4 = Pj4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj4 {}

impl Pin for Pj4 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 4 }
}

impl AfLcdR5 for Pj4 {
   fn af_lcd_r5(&self) -> usize { 14 }
}

impl AfEventout for Pj4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ5: Pj5 = Pj5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj5 {}

impl Pin for Pj5 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 5 }
}

impl AfLcdR6 for Pj5 {
   fn af_lcd_r6(&self) -> usize { 14 }
}

impl AfEventout for Pj5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ6: Pj6 = Pj6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj6 {}

impl Pin for Pj6 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 6 }
}

impl AfLcdR7 for Pj6 {
   fn af_lcd_r7(&self) -> usize { 14 }
}

impl AfEventout for Pj6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ7: Pj7 = Pj7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj7 {}

impl Pin for Pj7 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 7 }
}

impl AfLcdG0 for Pj7 {
   fn af_lcd_g0(&self) -> usize { 14 }
}

impl AfEventout for Pj7 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ8: Pj8 = Pj8 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj8 {}

impl Pin for Pj8 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 8 }
}

impl AfLcdG1 for Pj8 {
   fn af_lcd_g1(&self) -> usize { 14 }
}

impl AfEventout for Pj8 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ9: Pj9 = Pj9 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj9 {}

impl Pin for Pj9 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 9 }
}

impl AfLcdG2 for Pj9 {
   fn af_lcd_g2(&self) -> usize { 14 }
}

impl AfEventout for Pj9 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ10: Pj10 = Pj10 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj10 {}

impl Pin for Pj10 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 10 }
}

impl AfLcdG3 for Pj10 {
   fn af_lcd_g3(&self) -> usize { 14 }
}

impl AfEventout for Pj10 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ11: Pj11 = Pj11 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj11 {}

impl Pin for Pj11 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 11 }
}

impl AfLcdG4 for Pj11 {
   fn af_lcd_g4(&self) -> usize { 14 }
}

impl AfEventout for Pj11 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ12: Pj12 = Pj12 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj12 {}

impl Pin for Pj12 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 12 }
}

impl AfLcdB0 for Pj12 {
   fn af_lcd_b0(&self) -> usize { 14 }
}

impl AfEventout for Pj12 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ13: Pj13 = Pj13 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj13 {}

impl Pin for Pj13 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 13 }
}

impl AfLcdB1 for Pj13 {
   fn af_lcd_b1(&self) -> usize { 14 }
}

impl AfEventout for Pj13 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ14: Pj14 = Pj14 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj14 {}

impl Pin for Pj14 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 14 }
}

impl AfLcdB2 for Pj14 {
   fn af_lcd_b2(&self) -> usize { 14 }
}

impl AfEventout for Pj14 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PJ15: Pj15 = Pj15 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj15 {}

impl Pin for Pj15 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 15 }
}

impl AfLcdB3 for Pj15 {
   fn af_lcd_b3(&self) -> usize { 14 }
}

impl AfEventout for Pj15 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK0: Pk0 = Pk0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk0 {}

impl Pin for Pk0 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 0 }
}

impl AfLcdG5 for Pk0 {
   fn af_lcd_g5(&self) -> usize { 14 }
}

impl AfEventout for Pk0 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK1: Pk1 = Pk1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk1 {}

impl Pin for Pk1 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 1 }
}

impl AfLcdG6 for Pk1 {
   fn af_lcd_g6(&self) -> usize { 14 }
}

impl AfEventout for Pk1 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK2: Pk2 = Pk2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk2 {}

impl Pin for Pk2 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 2 }
}

impl AfLcdG7 for Pk2 {
   fn af_lcd_g7(&self) -> usize { 14 }
}

impl AfEventout for Pk2 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK3: Pk3 = Pk3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk3 {}

impl Pin for Pk3 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 3 }
}

impl AfLcdB4 for Pk3 {
   fn af_lcd_b4(&self) -> usize { 14 }
}

impl AfEventout for Pk3 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK4: Pk4 = Pk4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk4 {}

impl Pin for Pk4 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 4 }
}

impl AfLcdB5 for Pk4 {
   fn af_lcd_b5(&self) -> usize { 14 }
}

impl AfEventout for Pk4 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK5: Pk5 = Pk5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk5 {}

impl Pin for Pk5 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 5 }
}

impl AfLcdB6 for Pk5 {
   fn af_lcd_b6(&self) -> usize { 14 }
}

impl AfEventout for Pk5 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK6: Pk6 = Pk6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk6 {}

impl Pin for Pk6 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 6 }
}

impl AfLcdB7 for Pk6 {
   fn af_lcd_b7(&self) -> usize { 14 }
}

impl AfEventout for Pk6 {
   fn af_eventout(&self) -> usize { 15 }
}

pub const PK7: Pk7 = Pk7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk7 {}

impl Pin for Pk7 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 7 }
}

impl AfLcdDe for Pk7 {
   fn af_lcd_de(&self) -> usize { 14 }
}

impl AfEventout for Pk7 {
   fn af_eventout(&self) -> usize { 15 }
}

