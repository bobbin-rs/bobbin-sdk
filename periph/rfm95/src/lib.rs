#![no_std]

extern crate bobbin_bits;
pub type Addr = u8;
pub type Value = u8;

pub mod addr {
    use super::Addr;
    pub const REG_FIFO: Addr = 0x00;
    pub const REG_OPMODE: Addr = 0x01;
    pub const REG_FRF_MSB: Addr = 0x06;
    pub const REG_FRF_MID: Addr = 0x07;
    pub const REG_FRF_LSB: Addr = 0x08;
    pub const REG_PA_CONFIG: Addr = 0x09;
    pub const REG_PA_RAMP: Addr = 0x0a;
    pub const REG_OCP: Addr = 0x0b;
    pub const REG_LNA: Addr = 0x0c;
    pub const REG_FIFO_ADDR_PTR: Addr = 0x0d;
    pub const REG_FIFO_TX_BASE_ADDR: Addr = 0x0e;
    pub const REG_FIFO_RX_BASE_ADDR: Addr = 0x0f;
    pub const REG_FIFO_RX_CURRENT_ADDR: Addr = 0x10;
    pub const REG_IRQ_FLAGS_MASK: Addr = 0x11;
    pub const REG_IRQ_FLAGS: Addr = 0x12;
    pub const REG_RX_NB_BYTES: Addr = 0x13;
    pub const REG_RX_HEADER_CNT_VALUE_MSB: Addr = 0x14;
    pub const REG_RX_HEADER_CNT_VALUE_LSB: Addr = 0x15;
    pub const REG_RX_PACKET_CNT_VALUE_MSB: Addr = 0x16;
    pub const REG_RX_PACKET_CNT_VALUE_LSB: Addr = 0x17;
    pub const REG_MODEM_STAT: Addr = 0x18;
    pub const REG_PKT_SNR_VALUE: Addr = 0x19;
    pub const REG_PKT_RSSI_VALUE: Addr = 0x1a;
    pub const REG_RSSI_VALUE: Addr = 0x1b;
    pub const REG_HOP_CHANNEL: Addr = 0x1c;
    pub const REG_MODEM_CONFIG1: Addr = 0x1d;
    pub const REG_MODEM_CONFIG2: Addr = 0x1e;
    pub const REG_SYMB_TIMEOUT_LSB: Addr = 0x1f;
    pub const REG_PREAMBLE_MSB: Addr = 0x20;
    pub const REG_PREAMBLE_LSB: Addr = 0x21;
    pub const REG_PAYLOAD_LENGTH: Addr = 0x22;
    pub const REG_MAX_PAYLOAD_LENGTH: Addr = 0x23;
    pub const REG_HOP_PERIOD: Addr = 0x24;
    pub const REG_FIFO_RX_BYTE_ADDR: Addr = 0x25;
    pub const REG_MODEM_CONFIG3: Addr = 0x26;
    pub const REG_PPM_CORRECTION: Addr = 0x27;
    pub const REG_FEI_MSB: Addr = 0x28;
    pub const REG_FEI_MID: Addr = 0x29;
    pub const REG_FEI_LSB: Addr = 0x2a;
    pub const REG_RSSI_WIDEBAND: Addr = 0x2c;
    pub const REG_DETECT_OPTIMIZE: Addr = 0x31;
    pub const REG_INVERT_IQ: Addr = 0x33;
    pub const REG_DETECTION_THRESHOLD: Addr = 0x37;
    pub const REG_SYNC_WORD: Addr = 0x39;
    pub const REG_DIO_MAPPING1: Addr = 0x40;
    pub const REG_DIO_MAPPING2: Addr = 0x41;
    pub const REG_VERSION: Addr = 0x42;
    pub const REG_PLL_HOP: Addr = 0x44;
    pub const REG_TCXO: Addr = 0x4b;
    pub const REG_PA_DAC: Addr = 0x4d;
    pub const REG_FORMER_TEMP: Addr = 0x5b;
    pub const REG_AGC_REF: Addr = 0x61;
    pub const REG_AGC_THRESH1: Addr = 0x62;
    pub const REG_AGC_THRESH2: Addr = 0x63;
    pub const REG_AGC_THRESH3: Addr = 0x64;
    pub const REG_PLL_HF: Addr = 0x70;
}

pub struct Rfm95<RW> { rw: RW }

pub trait ReadWrite {
    fn read(&self, addr: Addr) -> Value;
    fn write(&self, addr: Addr, val: Value);
}

pub trait TryReadWrite {
    type Error;
    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error>;
    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error>;
}

impl<RW: ReadWrite> ReadWrite for Rfm95<RW> {
    fn read(&self, addr: Addr) -> Value { self.rw.read(addr) }
    fn write(&self, addr: Addr, val: Value) { self.rw.write(addr, val) }
}

impl<RW: TryReadWrite> TryReadWrite for Rfm95<RW> {
    type Error = RW::Error;
    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error> { self.rw.try_read(addr) }
    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error> { self.rw.try_write(addr, val) }
}

impl<RW> Rfm95<RW> {
    pub fn new(rw: RW) -> Self { Rfm95 { rw } }
}

impl<RW: ReadWrite> Rfm95<RW> {
    pub fn fifo(&self) -> reg::Fifo {
        reg::Fifo(self.read(addr::REG_FIFO))
    }
    pub fn set_fifo(&self, value: reg::Fifo) {
        self.write(addr::REG_FIFO, value.0)
    }
    pub fn with_fifo<F: FnOnce(reg::Fifo) -> reg::Fifo>(&self, f: F) {
        let tmp = reg::Fifo(self.read(addr::REG_FIFO));
        self.write(addr::REG_FIFO, f(tmp).0)
    }

    pub fn opmode(&self) -> reg::Opmode {
        reg::Opmode(self.read(addr::REG_OPMODE))
    }
    pub fn set_opmode(&self, value: reg::Opmode) {
        self.write(addr::REG_OPMODE, value.0)
    }
    pub fn with_opmode<F: FnOnce(reg::Opmode) -> reg::Opmode>(&self, f: F) {
        let tmp = reg::Opmode(self.read(addr::REG_OPMODE));
        self.write(addr::REG_OPMODE, f(tmp).0)
    }

    pub fn frf_msb(&self) -> reg::FrfMsb {
        reg::FrfMsb(self.read(addr::REG_FRF_MSB))
    }
    pub fn set_frf_msb(&self, value: reg::FrfMsb) {
        self.write(addr::REG_FRF_MSB, value.0)
    }
    pub fn with_frf_msb<F: FnOnce(reg::FrfMsb) -> reg::FrfMsb>(&self, f: F) {
        let tmp = reg::FrfMsb(self.read(addr::REG_FRF_MSB));
        self.write(addr::REG_FRF_MSB, f(tmp).0)
    }

    pub fn frf_mid(&self) -> reg::FrfMid {
        reg::FrfMid(self.read(addr::REG_FRF_MID))
    }
    pub fn set_frf_mid(&self, value: reg::FrfMid) {
        self.write(addr::REG_FRF_MID, value.0)
    }
    pub fn with_frf_mid<F: FnOnce(reg::FrfMid) -> reg::FrfMid>(&self, f: F) {
        let tmp = reg::FrfMid(self.read(addr::REG_FRF_MID));
        self.write(addr::REG_FRF_MID, f(tmp).0)
    }

    pub fn frf_lsb(&self) -> reg::FrfLsb {
        reg::FrfLsb(self.read(addr::REG_FRF_LSB))
    }
    pub fn set_frf_lsb(&self, value: reg::FrfLsb) {
        self.write(addr::REG_FRF_LSB, value.0)
    }
    pub fn with_frf_lsb<F: FnOnce(reg::FrfLsb) -> reg::FrfLsb>(&self, f: F) {
        let tmp = reg::FrfLsb(self.read(addr::REG_FRF_LSB));
        self.write(addr::REG_FRF_LSB, f(tmp).0)
    }

    pub fn pa_config(&self) -> reg::PaConfig {
        reg::PaConfig(self.read(addr::REG_PA_CONFIG))
    }
    pub fn set_pa_config(&self, value: reg::PaConfig) {
        self.write(addr::REG_PA_CONFIG, value.0)
    }
    pub fn with_pa_config<F: FnOnce(reg::PaConfig) -> reg::PaConfig>(&self, f: F) {
        let tmp = reg::PaConfig(self.read(addr::REG_PA_CONFIG));
        self.write(addr::REG_PA_CONFIG, f(tmp).0)
    }

    pub fn pa_ramp(&self) -> reg::PaRamp {
        reg::PaRamp(self.read(addr::REG_PA_RAMP))
    }
    pub fn set_pa_ramp(&self, value: reg::PaRamp) {
        self.write(addr::REG_PA_RAMP, value.0)
    }
    pub fn with_pa_ramp<F: FnOnce(reg::PaRamp) -> reg::PaRamp>(&self, f: F) {
        let tmp = reg::PaRamp(self.read(addr::REG_PA_RAMP));
        self.write(addr::REG_PA_RAMP, f(tmp).0)
    }

    pub fn ocp(&self) -> reg::Ocp {
        reg::Ocp(self.read(addr::REG_OCP))
    }
    pub fn set_ocp(&self, value: reg::Ocp) {
        self.write(addr::REG_OCP, value.0)
    }
    pub fn with_ocp<F: FnOnce(reg::Ocp) -> reg::Ocp>(&self, f: F) {
        let tmp = reg::Ocp(self.read(addr::REG_OCP));
        self.write(addr::REG_OCP, f(tmp).0)
    }

    pub fn lna(&self) -> reg::Lna {
        reg::Lna(self.read(addr::REG_LNA))
    }
    pub fn set_lna(&self, value: reg::Lna) {
        self.write(addr::REG_LNA, value.0)
    }
    pub fn with_lna<F: FnOnce(reg::Lna) -> reg::Lna>(&self, f: F) {
        let tmp = reg::Lna(self.read(addr::REG_LNA));
        self.write(addr::REG_LNA, f(tmp).0)
    }

    pub fn fifo_addr_ptr(&self) -> reg::FifoAddrPtr {
        reg::FifoAddrPtr(self.read(addr::REG_FIFO_ADDR_PTR))
    }
    pub fn set_fifo_addr_ptr(&self, value: reg::FifoAddrPtr) {
        self.write(addr::REG_FIFO_ADDR_PTR, value.0)
    }
    pub fn with_fifo_addr_ptr<F: FnOnce(reg::FifoAddrPtr) -> reg::FifoAddrPtr>(&self, f: F) {
        let tmp = reg::FifoAddrPtr(self.read(addr::REG_FIFO_ADDR_PTR));
        self.write(addr::REG_FIFO_ADDR_PTR, f(tmp).0)
    }

    pub fn fifo_tx_base_addr(&self) -> reg::FifoTxBaseAddr {
        reg::FifoTxBaseAddr(self.read(addr::REG_FIFO_TX_BASE_ADDR))
    }
    pub fn set_fifo_tx_base_addr(&self, value: reg::FifoTxBaseAddr) {
        self.write(addr::REG_FIFO_TX_BASE_ADDR, value.0)
    }
    pub fn with_fifo_tx_base_addr<F: FnOnce(reg::FifoTxBaseAddr) -> reg::FifoTxBaseAddr>(&self, f: F) {
        let tmp = reg::FifoTxBaseAddr(self.read(addr::REG_FIFO_TX_BASE_ADDR));
        self.write(addr::REG_FIFO_TX_BASE_ADDR, f(tmp).0)
    }

    pub fn fifo_rx_base_addr(&self) -> reg::FifoRxBaseAddr {
        reg::FifoRxBaseAddr(self.read(addr::REG_FIFO_RX_BASE_ADDR))
    }
    pub fn set_fifo_rx_base_addr(&self, value: reg::FifoRxBaseAddr) {
        self.write(addr::REG_FIFO_RX_BASE_ADDR, value.0)
    }
    pub fn with_fifo_rx_base_addr<F: FnOnce(reg::FifoRxBaseAddr) -> reg::FifoRxBaseAddr>(&self, f: F) {
        let tmp = reg::FifoRxBaseAddr(self.read(addr::REG_FIFO_RX_BASE_ADDR));
        self.write(addr::REG_FIFO_RX_BASE_ADDR, f(tmp).0)
    }

    pub fn fifo_rx_current_addr(&self) -> reg::FifoRxCurrentAddr {
        reg::FifoRxCurrentAddr(self.read(addr::REG_FIFO_RX_CURRENT_ADDR))
    }
    pub fn set_fifo_rx_current_addr(&self, value: reg::FifoRxCurrentAddr) {
        self.write(addr::REG_FIFO_RX_CURRENT_ADDR, value.0)
    }
    pub fn with_fifo_rx_current_addr<F: FnOnce(reg::FifoRxCurrentAddr) -> reg::FifoRxCurrentAddr>(&self, f: F) {
        let tmp = reg::FifoRxCurrentAddr(self.read(addr::REG_FIFO_RX_CURRENT_ADDR));
        self.write(addr::REG_FIFO_RX_CURRENT_ADDR, f(tmp).0)
    }

    pub fn irq_flags_mask(&self) -> reg::IrqFlagsMask {
        reg::IrqFlagsMask(self.read(addr::REG_IRQ_FLAGS_MASK))
    }
    pub fn set_irq_flags_mask(&self, value: reg::IrqFlagsMask) {
        self.write(addr::REG_IRQ_FLAGS_MASK, value.0)
    }
    pub fn with_irq_flags_mask<F: FnOnce(reg::IrqFlagsMask) -> reg::IrqFlagsMask>(&self, f: F) {
        let tmp = reg::IrqFlagsMask(self.read(addr::REG_IRQ_FLAGS_MASK));
        self.write(addr::REG_IRQ_FLAGS_MASK, f(tmp).0)
    }

    pub fn irq_flags(&self) -> reg::IrqFlags {
        reg::IrqFlags(self.read(addr::REG_IRQ_FLAGS))
    }
    pub fn set_irq_flags(&self, value: reg::IrqFlags) {
        self.write(addr::REG_IRQ_FLAGS, value.0)
    }
    pub fn with_irq_flags<F: FnOnce(reg::IrqFlags) -> reg::IrqFlags>(&self, f: F) {
        let tmp = reg::IrqFlags(self.read(addr::REG_IRQ_FLAGS));
        self.write(addr::REG_IRQ_FLAGS, f(tmp).0)
    }

    pub fn rx_nb_bytes(&self) -> reg::RxNbBytes {
        reg::RxNbBytes(self.read(addr::REG_RX_NB_BYTES))
    }
    pub fn set_rx_nb_bytes(&self, value: reg::RxNbBytes) {
        self.write(addr::REG_RX_NB_BYTES, value.0)
    }
    pub fn with_rx_nb_bytes<F: FnOnce(reg::RxNbBytes) -> reg::RxNbBytes>(&self, f: F) {
        let tmp = reg::RxNbBytes(self.read(addr::REG_RX_NB_BYTES));
        self.write(addr::REG_RX_NB_BYTES, f(tmp).0)
    }

    pub fn rx_header_cnt_value_msb(&self) -> reg::RxHeaderCntValueMsb {
        reg::RxHeaderCntValueMsb(self.read(addr::REG_RX_HEADER_CNT_VALUE_MSB))
    }
    pub fn set_rx_header_cnt_value_msb(&self, value: reg::RxHeaderCntValueMsb) {
        self.write(addr::REG_RX_HEADER_CNT_VALUE_MSB, value.0)
    }
    pub fn with_rx_header_cnt_value_msb<F: FnOnce(reg::RxHeaderCntValueMsb) -> reg::RxHeaderCntValueMsb>(&self, f: F) {
        let tmp = reg::RxHeaderCntValueMsb(self.read(addr::REG_RX_HEADER_CNT_VALUE_MSB));
        self.write(addr::REG_RX_HEADER_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn rx_header_cnt_value_lsb(&self) -> reg::RxHeaderCntValueLsb {
        reg::RxHeaderCntValueLsb(self.read(addr::REG_RX_HEADER_CNT_VALUE_LSB))
    }
    pub fn set_rx_header_cnt_value_lsb(&self, value: reg::RxHeaderCntValueLsb) {
        self.write(addr::REG_RX_HEADER_CNT_VALUE_LSB, value.0)
    }
    pub fn with_rx_header_cnt_value_lsb<F: FnOnce(reg::RxHeaderCntValueLsb) -> reg::RxHeaderCntValueLsb>(&self, f: F) {
        let tmp = reg::RxHeaderCntValueLsb(self.read(addr::REG_RX_HEADER_CNT_VALUE_LSB));
        self.write(addr::REG_RX_HEADER_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn rx_packet_cnt_value_msb(&self) -> reg::RxPacketCntValueMsb {
        reg::RxPacketCntValueMsb(self.read(addr::REG_RX_PACKET_CNT_VALUE_MSB))
    }
    pub fn set_rx_packet_cnt_value_msb(&self, value: reg::RxPacketCntValueMsb) {
        self.write(addr::REG_RX_PACKET_CNT_VALUE_MSB, value.0)
    }
    pub fn with_rx_packet_cnt_value_msb<F: FnOnce(reg::RxPacketCntValueMsb) -> reg::RxPacketCntValueMsb>(&self, f: F) {
        let tmp = reg::RxPacketCntValueMsb(self.read(addr::REG_RX_PACKET_CNT_VALUE_MSB));
        self.write(addr::REG_RX_PACKET_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn rx_packet_cnt_value_lsb(&self) -> reg::RxPacketCntValueLsb {
        reg::RxPacketCntValueLsb(self.read(addr::REG_RX_PACKET_CNT_VALUE_LSB))
    }
    pub fn set_rx_packet_cnt_value_lsb(&self, value: reg::RxPacketCntValueLsb) {
        self.write(addr::REG_RX_PACKET_CNT_VALUE_LSB, value.0)
    }
    pub fn with_rx_packet_cnt_value_lsb<F: FnOnce(reg::RxPacketCntValueLsb) -> reg::RxPacketCntValueLsb>(&self, f: F) {
        let tmp = reg::RxPacketCntValueLsb(self.read(addr::REG_RX_PACKET_CNT_VALUE_LSB));
        self.write(addr::REG_RX_PACKET_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn modem_stat(&self) -> reg::ModemStat {
        reg::ModemStat(self.read(addr::REG_MODEM_STAT))
    }
    pub fn set_modem_stat(&self, value: reg::ModemStat) {
        self.write(addr::REG_MODEM_STAT, value.0)
    }
    pub fn with_modem_stat<F: FnOnce(reg::ModemStat) -> reg::ModemStat>(&self, f: F) {
        let tmp = reg::ModemStat(self.read(addr::REG_MODEM_STAT));
        self.write(addr::REG_MODEM_STAT, f(tmp).0)
    }

    pub fn pkt_snr_value(&self) -> reg::PktSnrValue {
        reg::PktSnrValue(self.read(addr::REG_PKT_SNR_VALUE))
    }
    pub fn set_pkt_snr_value(&self, value: reg::PktSnrValue) {
        self.write(addr::REG_PKT_SNR_VALUE, value.0)
    }
    pub fn with_pkt_snr_value<F: FnOnce(reg::PktSnrValue) -> reg::PktSnrValue>(&self, f: F) {
        let tmp = reg::PktSnrValue(self.read(addr::REG_PKT_SNR_VALUE));
        self.write(addr::REG_PKT_SNR_VALUE, f(tmp).0)
    }

    pub fn pkt_rssi_value(&self) -> reg::PktRssiValue {
        reg::PktRssiValue(self.read(addr::REG_PKT_RSSI_VALUE))
    }
    pub fn set_pkt_rssi_value(&self, value: reg::PktRssiValue) {
        self.write(addr::REG_PKT_RSSI_VALUE, value.0)
    }
    pub fn with_pkt_rssi_value<F: FnOnce(reg::PktRssiValue) -> reg::PktRssiValue>(&self, f: F) {
        let tmp = reg::PktRssiValue(self.read(addr::REG_PKT_RSSI_VALUE));
        self.write(addr::REG_PKT_RSSI_VALUE, f(tmp).0)
    }

    pub fn rssi_value(&self) -> reg::RssiValue {
        reg::RssiValue(self.read(addr::REG_RSSI_VALUE))
    }
    pub fn set_rssi_value(&self, value: reg::RssiValue) {
        self.write(addr::REG_RSSI_VALUE, value.0)
    }
    pub fn with_rssi_value<F: FnOnce(reg::RssiValue) -> reg::RssiValue>(&self, f: F) {
        let tmp = reg::RssiValue(self.read(addr::REG_RSSI_VALUE));
        self.write(addr::REG_RSSI_VALUE, f(tmp).0)
    }

    pub fn hop_channel(&self) -> reg::HopChannel {
        reg::HopChannel(self.read(addr::REG_HOP_CHANNEL))
    }
    pub fn set_hop_channel(&self, value: reg::HopChannel) {
        self.write(addr::REG_HOP_CHANNEL, value.0)
    }
    pub fn with_hop_channel<F: FnOnce(reg::HopChannel) -> reg::HopChannel>(&self, f: F) {
        let tmp = reg::HopChannel(self.read(addr::REG_HOP_CHANNEL));
        self.write(addr::REG_HOP_CHANNEL, f(tmp).0)
    }

    pub fn modem_config1(&self) -> reg::ModemConfig1 {
        reg::ModemConfig1(self.read(addr::REG_MODEM_CONFIG1))
    }
    pub fn set_modem_config1(&self, value: reg::ModemConfig1) {
        self.write(addr::REG_MODEM_CONFIG1, value.0)
    }
    pub fn with_modem_config1<F: FnOnce(reg::ModemConfig1) -> reg::ModemConfig1>(&self, f: F) {
        let tmp = reg::ModemConfig1(self.read(addr::REG_MODEM_CONFIG1));
        self.write(addr::REG_MODEM_CONFIG1, f(tmp).0)
    }

    pub fn modem_config2(&self) -> reg::ModemConfig2 {
        reg::ModemConfig2(self.read(addr::REG_MODEM_CONFIG2))
    }
    pub fn set_modem_config2(&self, value: reg::ModemConfig2) {
        self.write(addr::REG_MODEM_CONFIG2, value.0)
    }
    pub fn with_modem_config2<F: FnOnce(reg::ModemConfig2) -> reg::ModemConfig2>(&self, f: F) {
        let tmp = reg::ModemConfig2(self.read(addr::REG_MODEM_CONFIG2));
        self.write(addr::REG_MODEM_CONFIG2, f(tmp).0)
    }

    pub fn symb_timeout_lsb(&self) -> reg::SymbTimeoutLsb {
        reg::SymbTimeoutLsb(self.read(addr::REG_SYMB_TIMEOUT_LSB))
    }
    pub fn set_symb_timeout_lsb(&self, value: reg::SymbTimeoutLsb) {
        self.write(addr::REG_SYMB_TIMEOUT_LSB, value.0)
    }
    pub fn with_symb_timeout_lsb<F: FnOnce(reg::SymbTimeoutLsb) -> reg::SymbTimeoutLsb>(&self, f: F) {
        let tmp = reg::SymbTimeoutLsb(self.read(addr::REG_SYMB_TIMEOUT_LSB));
        self.write(addr::REG_SYMB_TIMEOUT_LSB, f(tmp).0)
    }

    pub fn preamble_msb(&self) -> reg::PreambleMsb {
        reg::PreambleMsb(self.read(addr::REG_PREAMBLE_MSB))
    }
    pub fn set_preamble_msb(&self, value: reg::PreambleMsb) {
        self.write(addr::REG_PREAMBLE_MSB, value.0)
    }
    pub fn with_preamble_msb<F: FnOnce(reg::PreambleMsb) -> reg::PreambleMsb>(&self, f: F) {
        let tmp = reg::PreambleMsb(self.read(addr::REG_PREAMBLE_MSB));
        self.write(addr::REG_PREAMBLE_MSB, f(tmp).0)
    }

    pub fn preamble_lsb(&self) -> reg::PreambleLsb {
        reg::PreambleLsb(self.read(addr::REG_PREAMBLE_LSB))
    }
    pub fn set_preamble_lsb(&self, value: reg::PreambleLsb) {
        self.write(addr::REG_PREAMBLE_LSB, value.0)
    }
    pub fn with_preamble_lsb<F: FnOnce(reg::PreambleLsb) -> reg::PreambleLsb>(&self, f: F) {
        let tmp = reg::PreambleLsb(self.read(addr::REG_PREAMBLE_LSB));
        self.write(addr::REG_PREAMBLE_LSB, f(tmp).0)
    }

    pub fn payload_length(&self) -> reg::PayloadLength {
        reg::PayloadLength(self.read(addr::REG_PAYLOAD_LENGTH))
    }
    pub fn set_payload_length(&self, value: reg::PayloadLength) {
        self.write(addr::REG_PAYLOAD_LENGTH, value.0)
    }
    pub fn with_payload_length<F: FnOnce(reg::PayloadLength) -> reg::PayloadLength>(&self, f: F) {
        let tmp = reg::PayloadLength(self.read(addr::REG_PAYLOAD_LENGTH));
        self.write(addr::REG_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn max_payload_length(&self) -> reg::MaxPayloadLength {
        reg::MaxPayloadLength(self.read(addr::REG_MAX_PAYLOAD_LENGTH))
    }
    pub fn set_max_payload_length(&self, value: reg::MaxPayloadLength) {
        self.write(addr::REG_MAX_PAYLOAD_LENGTH, value.0)
    }
    pub fn with_max_payload_length<F: FnOnce(reg::MaxPayloadLength) -> reg::MaxPayloadLength>(&self, f: F) {
        let tmp = reg::MaxPayloadLength(self.read(addr::REG_MAX_PAYLOAD_LENGTH));
        self.write(addr::REG_MAX_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn hop_period(&self) -> reg::HopPeriod {
        reg::HopPeriod(self.read(addr::REG_HOP_PERIOD))
    }
    pub fn set_hop_period(&self, value: reg::HopPeriod) {
        self.write(addr::REG_HOP_PERIOD, value.0)
    }
    pub fn with_hop_period<F: FnOnce(reg::HopPeriod) -> reg::HopPeriod>(&self, f: F) {
        let tmp = reg::HopPeriod(self.read(addr::REG_HOP_PERIOD));
        self.write(addr::REG_HOP_PERIOD, f(tmp).0)
    }

    pub fn fifo_rx_byte_addr(&self) -> reg::FifoRxByteAddr {
        reg::FifoRxByteAddr(self.read(addr::REG_FIFO_RX_BYTE_ADDR))
    }
    pub fn set_fifo_rx_byte_addr(&self, value: reg::FifoRxByteAddr) {
        self.write(addr::REG_FIFO_RX_BYTE_ADDR, value.0)
    }
    pub fn with_fifo_rx_byte_addr<F: FnOnce(reg::FifoRxByteAddr) -> reg::FifoRxByteAddr>(&self, f: F) {
        let tmp = reg::FifoRxByteAddr(self.read(addr::REG_FIFO_RX_BYTE_ADDR));
        self.write(addr::REG_FIFO_RX_BYTE_ADDR, f(tmp).0)
    }

    pub fn modem_config3(&self) -> reg::ModemConfig3 {
        reg::ModemConfig3(self.read(addr::REG_MODEM_CONFIG3))
    }
    pub fn set_modem_config3(&self, value: reg::ModemConfig3) {
        self.write(addr::REG_MODEM_CONFIG3, value.0)
    }
    pub fn with_modem_config3<F: FnOnce(reg::ModemConfig3) -> reg::ModemConfig3>(&self, f: F) {
        let tmp = reg::ModemConfig3(self.read(addr::REG_MODEM_CONFIG3));
        self.write(addr::REG_MODEM_CONFIG3, f(tmp).0)
    }

    pub fn ppm_correction(&self) -> reg::PpmCorrection {
        reg::PpmCorrection(self.read(addr::REG_PPM_CORRECTION))
    }
    pub fn set_ppm_correction(&self, value: reg::PpmCorrection) {
        self.write(addr::REG_PPM_CORRECTION, value.0)
    }
    pub fn with_ppm_correction<F: FnOnce(reg::PpmCorrection) -> reg::PpmCorrection>(&self, f: F) {
        let tmp = reg::PpmCorrection(self.read(addr::REG_PPM_CORRECTION));
        self.write(addr::REG_PPM_CORRECTION, f(tmp).0)
    }

    pub fn fei_msb(&self) -> reg::FeiMsb {
        reg::FeiMsb(self.read(addr::REG_FEI_MSB))
    }
    pub fn set_fei_msb(&self, value: reg::FeiMsb) {
        self.write(addr::REG_FEI_MSB, value.0)
    }
    pub fn with_fei_msb<F: FnOnce(reg::FeiMsb) -> reg::FeiMsb>(&self, f: F) {
        let tmp = reg::FeiMsb(self.read(addr::REG_FEI_MSB));
        self.write(addr::REG_FEI_MSB, f(tmp).0)
    }

    pub fn fei_mid(&self) -> reg::FeiMid {
        reg::FeiMid(self.read(addr::REG_FEI_MID))
    }
    pub fn set_fei_mid(&self, value: reg::FeiMid) {
        self.write(addr::REG_FEI_MID, value.0)
    }
    pub fn with_fei_mid<F: FnOnce(reg::FeiMid) -> reg::FeiMid>(&self, f: F) {
        let tmp = reg::FeiMid(self.read(addr::REG_FEI_MID));
        self.write(addr::REG_FEI_MID, f(tmp).0)
    }

    pub fn fei_lsb(&self) -> reg::FeiLsb {
        reg::FeiLsb(self.read(addr::REG_FEI_LSB))
    }
    pub fn set_fei_lsb(&self, value: reg::FeiLsb) {
        self.write(addr::REG_FEI_LSB, value.0)
    }
    pub fn with_fei_lsb<F: FnOnce(reg::FeiLsb) -> reg::FeiLsb>(&self, f: F) {
        let tmp = reg::FeiLsb(self.read(addr::REG_FEI_LSB));
        self.write(addr::REG_FEI_LSB, f(tmp).0)
    }

    pub fn rssi_wideband(&self) -> reg::RssiWideband {
        reg::RssiWideband(self.read(addr::REG_RSSI_WIDEBAND))
    }
    pub fn set_rssi_wideband(&self, value: reg::RssiWideband) {
        self.write(addr::REG_RSSI_WIDEBAND, value.0)
    }
    pub fn with_rssi_wideband<F: FnOnce(reg::RssiWideband) -> reg::RssiWideband>(&self, f: F) {
        let tmp = reg::RssiWideband(self.read(addr::REG_RSSI_WIDEBAND));
        self.write(addr::REG_RSSI_WIDEBAND, f(tmp).0)
    }

    pub fn detect_optimize(&self) -> reg::DetectOptimize {
        reg::DetectOptimize(self.read(addr::REG_DETECT_OPTIMIZE))
    }
    pub fn set_detect_optimize(&self, value: reg::DetectOptimize) {
        self.write(addr::REG_DETECT_OPTIMIZE, value.0)
    }
    pub fn with_detect_optimize<F: FnOnce(reg::DetectOptimize) -> reg::DetectOptimize>(&self, f: F) {
        let tmp = reg::DetectOptimize(self.read(addr::REG_DETECT_OPTIMIZE));
        self.write(addr::REG_DETECT_OPTIMIZE, f(tmp).0)
    }

    pub fn invert_iq(&self) -> reg::InvertIq {
        reg::InvertIq(self.read(addr::REG_INVERT_IQ))
    }
    pub fn set_invert_iq(&self, value: reg::InvertIq) {
        self.write(addr::REG_INVERT_IQ, value.0)
    }
    pub fn with_invert_iq<F: FnOnce(reg::InvertIq) -> reg::InvertIq>(&self, f: F) {
        let tmp = reg::InvertIq(self.read(addr::REG_INVERT_IQ));
        self.write(addr::REG_INVERT_IQ, f(tmp).0)
    }

    pub fn detection_threshold(&self) -> reg::DetectionThreshold {
        reg::DetectionThreshold(self.read(addr::REG_DETECTION_THRESHOLD))
    }
    pub fn set_detection_threshold(&self, value: reg::DetectionThreshold) {
        self.write(addr::REG_DETECTION_THRESHOLD, value.0)
    }
    pub fn with_detection_threshold<F: FnOnce(reg::DetectionThreshold) -> reg::DetectionThreshold>(&self, f: F) {
        let tmp = reg::DetectionThreshold(self.read(addr::REG_DETECTION_THRESHOLD));
        self.write(addr::REG_DETECTION_THRESHOLD, f(tmp).0)
    }

    pub fn sync_word(&self) -> reg::SyncWord {
        reg::SyncWord(self.read(addr::REG_SYNC_WORD))
    }
    pub fn set_sync_word(&self, value: reg::SyncWord) {
        self.write(addr::REG_SYNC_WORD, value.0)
    }
    pub fn with_sync_word<F: FnOnce(reg::SyncWord) -> reg::SyncWord>(&self, f: F) {
        let tmp = reg::SyncWord(self.read(addr::REG_SYNC_WORD));
        self.write(addr::REG_SYNC_WORD, f(tmp).0)
    }

    pub fn dio_mapping1(&self) -> reg::DioMapping1 {
        reg::DioMapping1(self.read(addr::REG_DIO_MAPPING1))
    }
    pub fn set_dio_mapping1(&self, value: reg::DioMapping1) {
        self.write(addr::REG_DIO_MAPPING1, value.0)
    }
    pub fn with_dio_mapping1<F: FnOnce(reg::DioMapping1) -> reg::DioMapping1>(&self, f: F) {
        let tmp = reg::DioMapping1(self.read(addr::REG_DIO_MAPPING1));
        self.write(addr::REG_DIO_MAPPING1, f(tmp).0)
    }

    pub fn dio_mapping2(&self) -> reg::DioMapping2 {
        reg::DioMapping2(self.read(addr::REG_DIO_MAPPING2))
    }
    pub fn set_dio_mapping2(&self, value: reg::DioMapping2) {
        self.write(addr::REG_DIO_MAPPING2, value.0)
    }
    pub fn with_dio_mapping2<F: FnOnce(reg::DioMapping2) -> reg::DioMapping2>(&self, f: F) {
        let tmp = reg::DioMapping2(self.read(addr::REG_DIO_MAPPING2));
        self.write(addr::REG_DIO_MAPPING2, f(tmp).0)
    }

    pub fn version(&self) -> reg::Version {
        reg::Version(self.read(addr::REG_VERSION))
    }
    pub fn set_version(&self, value: reg::Version) {
        self.write(addr::REG_VERSION, value.0)
    }
    pub fn with_version<F: FnOnce(reg::Version) -> reg::Version>(&self, f: F) {
        let tmp = reg::Version(self.read(addr::REG_VERSION));
        self.write(addr::REG_VERSION, f(tmp).0)
    }

    pub fn pll_hop(&self) -> reg::PllHop {
        reg::PllHop(self.read(addr::REG_PLL_HOP))
    }
    pub fn set_pll_hop(&self, value: reg::PllHop) {
        self.write(addr::REG_PLL_HOP, value.0)
    }
    pub fn with_pll_hop<F: FnOnce(reg::PllHop) -> reg::PllHop>(&self, f: F) {
        let tmp = reg::PllHop(self.read(addr::REG_PLL_HOP));
        self.write(addr::REG_PLL_HOP, f(tmp).0)
    }

    pub fn tcxo(&self) -> reg::Tcxo {
        reg::Tcxo(self.read(addr::REG_TCXO))
    }
    pub fn set_tcxo(&self, value: reg::Tcxo) {
        self.write(addr::REG_TCXO, value.0)
    }
    pub fn with_tcxo<F: FnOnce(reg::Tcxo) -> reg::Tcxo>(&self, f: F) {
        let tmp = reg::Tcxo(self.read(addr::REG_TCXO));
        self.write(addr::REG_TCXO, f(tmp).0)
    }

    pub fn pa_dac(&self) -> reg::PaDac {
        reg::PaDac(self.read(addr::REG_PA_DAC))
    }
    pub fn set_pa_dac(&self, value: reg::PaDac) {
        self.write(addr::REG_PA_DAC, value.0)
    }
    pub fn with_pa_dac<F: FnOnce(reg::PaDac) -> reg::PaDac>(&self, f: F) {
        let tmp = reg::PaDac(self.read(addr::REG_PA_DAC));
        self.write(addr::REG_PA_DAC, f(tmp).0)
    }

    pub fn former_temp(&self) -> reg::FormerTemp {
        reg::FormerTemp(self.read(addr::REG_FORMER_TEMP))
    }
    pub fn set_former_temp(&self, value: reg::FormerTemp) {
        self.write(addr::REG_FORMER_TEMP, value.0)
    }
    pub fn with_former_temp<F: FnOnce(reg::FormerTemp) -> reg::FormerTemp>(&self, f: F) {
        let tmp = reg::FormerTemp(self.read(addr::REG_FORMER_TEMP));
        self.write(addr::REG_FORMER_TEMP, f(tmp).0)
    }

    pub fn agc_ref(&self) -> reg::AgcRef {
        reg::AgcRef(self.read(addr::REG_AGC_REF))
    }
    pub fn set_agc_ref(&self, value: reg::AgcRef) {
        self.write(addr::REG_AGC_REF, value.0)
    }
    pub fn with_agc_ref<F: FnOnce(reg::AgcRef) -> reg::AgcRef>(&self, f: F) {
        let tmp = reg::AgcRef(self.read(addr::REG_AGC_REF));
        self.write(addr::REG_AGC_REF, f(tmp).0)
    }

    pub fn agc_thresh1(&self) -> reg::AgcThresh1 {
        reg::AgcThresh1(self.read(addr::REG_AGC_THRESH1))
    }
    pub fn set_agc_thresh1(&self, value: reg::AgcThresh1) {
        self.write(addr::REG_AGC_THRESH1, value.0)
    }
    pub fn with_agc_thresh1<F: FnOnce(reg::AgcThresh1) -> reg::AgcThresh1>(&self, f: F) {
        let tmp = reg::AgcThresh1(self.read(addr::REG_AGC_THRESH1));
        self.write(addr::REG_AGC_THRESH1, f(tmp).0)
    }

    pub fn agc_thresh2(&self) -> reg::AgcThresh2 {
        reg::AgcThresh2(self.read(addr::REG_AGC_THRESH2))
    }
    pub fn set_agc_thresh2(&self, value: reg::AgcThresh2) {
        self.write(addr::REG_AGC_THRESH2, value.0)
    }
    pub fn with_agc_thresh2<F: FnOnce(reg::AgcThresh2) -> reg::AgcThresh2>(&self, f: F) {
        let tmp = reg::AgcThresh2(self.read(addr::REG_AGC_THRESH2));
        self.write(addr::REG_AGC_THRESH2, f(tmp).0)
    }

    pub fn agc_thresh3(&self) -> reg::AgcThresh3 {
        reg::AgcThresh3(self.read(addr::REG_AGC_THRESH3))
    }
    pub fn set_agc_thresh3(&self, value: reg::AgcThresh3) {
        self.write(addr::REG_AGC_THRESH3, value.0)
    }
    pub fn with_agc_thresh3<F: FnOnce(reg::AgcThresh3) -> reg::AgcThresh3>(&self, f: F) {
        let tmp = reg::AgcThresh3(self.read(addr::REG_AGC_THRESH3));
        self.write(addr::REG_AGC_THRESH3, f(tmp).0)
    }

    pub fn pll_hf(&self) -> reg::PllHf {
        reg::PllHf(self.read(addr::REG_PLL_HF))
    }
    pub fn set_pll_hf(&self, value: reg::PllHf) {
        self.write(addr::REG_PLL_HF, value.0)
    }
    pub fn with_pll_hf<F: FnOnce(reg::PllHf) -> reg::PllHf>(&self, f: F) {
        let tmp = reg::PllHf(self.read(addr::REG_PLL_HF));
        self.write(addr::REG_PLL_HF, f(tmp).0)
    }

}

impl<RW: TryReadWrite> Rfm95<RW> {
    pub fn try_fifo(&self) -> Result<reg::Fifo, RW::Error> {
        Ok(reg::Fifo(self.try_read(REG_FIFO)?))
    }
    pub fn try_set_fifo(&self, value: reg::Fifo) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO, value.0)
    }
    pub fn try_with_fifo<F: FnOnce(reg::Fifo) -> reg::Fifo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Fifo(self.try_read(REG_FIFO)?);
        self.try_write(REG_FIFO, f(tmp).0)
    }

    pub fn try_opmode(&self) -> Result<reg::Opmode, RW::Error> {
        Ok(reg::Opmode(self.try_read(REG_OPMODE)?))
    }
    pub fn try_set_opmode(&self, value: reg::Opmode) -> Result<(), RW::Error> {
        self.try_write(REG_OPMODE, value.0)
    }
    pub fn try_with_opmode<F: FnOnce(reg::Opmode) -> reg::Opmode>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Opmode(self.try_read(REG_OPMODE)?);
        self.try_write(REG_OPMODE, f(tmp).0)
    }

    pub fn try_frf_msb(&self) -> Result<reg::FrfMsb, RW::Error> {
        Ok(reg::FrfMsb(self.try_read(REG_FRF_MSB)?))
    }
    pub fn try_set_frf_msb(&self, value: reg::FrfMsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_MSB, value.0)
    }
    pub fn try_with_frf_msb<F: FnOnce(reg::FrfMsb) -> reg::FrfMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FrfMsb(self.try_read(REG_FRF_MSB)?);
        self.try_write(REG_FRF_MSB, f(tmp).0)
    }

    pub fn try_frf_mid(&self) -> Result<reg::FrfMid, RW::Error> {
        Ok(reg::FrfMid(self.try_read(REG_FRF_MID)?))
    }
    pub fn try_set_frf_mid(&self, value: reg::FrfMid) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_MID, value.0)
    }
    pub fn try_with_frf_mid<F: FnOnce(reg::FrfMid) -> reg::FrfMid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FrfMid(self.try_read(REG_FRF_MID)?);
        self.try_write(REG_FRF_MID, f(tmp).0)
    }

    pub fn try_frf_lsb(&self) -> Result<reg::FrfLsb, RW::Error> {
        Ok(reg::FrfLsb(self.try_read(REG_FRF_LSB)?))
    }
    pub fn try_set_frf_lsb(&self, value: reg::FrfLsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_LSB, value.0)
    }
    pub fn try_with_frf_lsb<F: FnOnce(reg::FrfLsb) -> reg::FrfLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FrfLsb(self.try_read(REG_FRF_LSB)?);
        self.try_write(REG_FRF_LSB, f(tmp).0)
    }

    pub fn try_pa_config(&self) -> Result<reg::PaConfig, RW::Error> {
        Ok(reg::PaConfig(self.try_read(REG_PA_CONFIG)?))
    }
    pub fn try_set_pa_config(&self, value: reg::PaConfig) -> Result<(), RW::Error> {
        self.try_write(REG_PA_CONFIG, value.0)
    }
    pub fn try_with_pa_config<F: FnOnce(reg::PaConfig) -> reg::PaConfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PaConfig(self.try_read(REG_PA_CONFIG)?);
        self.try_write(REG_PA_CONFIG, f(tmp).0)
    }

    pub fn try_pa_ramp(&self) -> Result<reg::PaRamp, RW::Error> {
        Ok(reg::PaRamp(self.try_read(REG_PA_RAMP)?))
    }
    pub fn try_set_pa_ramp(&self, value: reg::PaRamp) -> Result<(), RW::Error> {
        self.try_write(REG_PA_RAMP, value.0)
    }
    pub fn try_with_pa_ramp<F: FnOnce(reg::PaRamp) -> reg::PaRamp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PaRamp(self.try_read(REG_PA_RAMP)?);
        self.try_write(REG_PA_RAMP, f(tmp).0)
    }

    pub fn try_ocp(&self) -> Result<reg::Ocp, RW::Error> {
        Ok(reg::Ocp(self.try_read(REG_OCP)?))
    }
    pub fn try_set_ocp(&self, value: reg::Ocp) -> Result<(), RW::Error> {
        self.try_write(REG_OCP, value.0)
    }
    pub fn try_with_ocp<F: FnOnce(reg::Ocp) -> reg::Ocp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Ocp(self.try_read(REG_OCP)?);
        self.try_write(REG_OCP, f(tmp).0)
    }

    pub fn try_lna(&self) -> Result<reg::Lna, RW::Error> {
        Ok(reg::Lna(self.try_read(REG_LNA)?))
    }
    pub fn try_set_lna(&self, value: reg::Lna) -> Result<(), RW::Error> {
        self.try_write(REG_LNA, value.0)
    }
    pub fn try_with_lna<F: FnOnce(reg::Lna) -> reg::Lna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Lna(self.try_read(REG_LNA)?);
        self.try_write(REG_LNA, f(tmp).0)
    }

    pub fn try_fifo_addr_ptr(&self) -> Result<reg::FifoAddrPtr, RW::Error> {
        Ok(reg::FifoAddrPtr(self.try_read(REG_FIFO_ADDR_PTR)?))
    }
    pub fn try_set_fifo_addr_ptr(&self, value: reg::FifoAddrPtr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_ADDR_PTR, value.0)
    }
    pub fn try_with_fifo_addr_ptr<F: FnOnce(reg::FifoAddrPtr) -> reg::FifoAddrPtr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FifoAddrPtr(self.try_read(REG_FIFO_ADDR_PTR)?);
        self.try_write(REG_FIFO_ADDR_PTR, f(tmp).0)
    }

    pub fn try_fifo_tx_base_addr(&self) -> Result<reg::FifoTxBaseAddr, RW::Error> {
        Ok(reg::FifoTxBaseAddr(self.try_read(REG_FIFO_TX_BASE_ADDR)?))
    }
    pub fn try_set_fifo_tx_base_addr(&self, value: reg::FifoTxBaseAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_TX_BASE_ADDR, value.0)
    }
    pub fn try_with_fifo_tx_base_addr<F: FnOnce(reg::FifoTxBaseAddr) -> reg::FifoTxBaseAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FifoTxBaseAddr(self.try_read(REG_FIFO_TX_BASE_ADDR)?);
        self.try_write(REG_FIFO_TX_BASE_ADDR, f(tmp).0)
    }

    pub fn try_fifo_rx_base_addr(&self) -> Result<reg::FifoRxBaseAddr, RW::Error> {
        Ok(reg::FifoRxBaseAddr(self.try_read(REG_FIFO_RX_BASE_ADDR)?))
    }
    pub fn try_set_fifo_rx_base_addr(&self, value: reg::FifoRxBaseAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_BASE_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_base_addr<F: FnOnce(reg::FifoRxBaseAddr) -> reg::FifoRxBaseAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FifoRxBaseAddr(self.try_read(REG_FIFO_RX_BASE_ADDR)?);
        self.try_write(REG_FIFO_RX_BASE_ADDR, f(tmp).0)
    }

    pub fn try_fifo_rx_current_addr(&self) -> Result<reg::FifoRxCurrentAddr, RW::Error> {
        Ok(reg::FifoRxCurrentAddr(self.try_read(REG_FIFO_RX_CURRENT_ADDR)?))
    }
    pub fn try_set_fifo_rx_current_addr(&self, value: reg::FifoRxCurrentAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_CURRENT_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_current_addr<F: FnOnce(reg::FifoRxCurrentAddr) -> reg::FifoRxCurrentAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FifoRxCurrentAddr(self.try_read(REG_FIFO_RX_CURRENT_ADDR)?);
        self.try_write(REG_FIFO_RX_CURRENT_ADDR, f(tmp).0)
    }

    pub fn try_irq_flags_mask(&self) -> Result<reg::IrqFlagsMask, RW::Error> {
        Ok(reg::IrqFlagsMask(self.try_read(REG_IRQ_FLAGS_MASK)?))
    }
    pub fn try_set_irq_flags_mask(&self, value: reg::IrqFlagsMask) -> Result<(), RW::Error> {
        self.try_write(REG_IRQ_FLAGS_MASK, value.0)
    }
    pub fn try_with_irq_flags_mask<F: FnOnce(reg::IrqFlagsMask) -> reg::IrqFlagsMask>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::IrqFlagsMask(self.try_read(REG_IRQ_FLAGS_MASK)?);
        self.try_write(REG_IRQ_FLAGS_MASK, f(tmp).0)
    }

    pub fn try_irq_flags(&self) -> Result<reg::IrqFlags, RW::Error> {
        Ok(reg::IrqFlags(self.try_read(REG_IRQ_FLAGS)?))
    }
    pub fn try_set_irq_flags(&self, value: reg::IrqFlags) -> Result<(), RW::Error> {
        self.try_write(REG_IRQ_FLAGS, value.0)
    }
    pub fn try_with_irq_flags<F: FnOnce(reg::IrqFlags) -> reg::IrqFlags>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::IrqFlags(self.try_read(REG_IRQ_FLAGS)?);
        self.try_write(REG_IRQ_FLAGS, f(tmp).0)
    }

    pub fn try_rx_nb_bytes(&self) -> Result<reg::RxNbBytes, RW::Error> {
        Ok(reg::RxNbBytes(self.try_read(REG_RX_NB_BYTES)?))
    }
    pub fn try_set_rx_nb_bytes(&self, value: reg::RxNbBytes) -> Result<(), RW::Error> {
        self.try_write(REG_RX_NB_BYTES, value.0)
    }
    pub fn try_with_rx_nb_bytes<F: FnOnce(reg::RxNbBytes) -> reg::RxNbBytes>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RxNbBytes(self.try_read(REG_RX_NB_BYTES)?);
        self.try_write(REG_RX_NB_BYTES, f(tmp).0)
    }

    pub fn try_rx_header_cnt_value_msb(&self) -> Result<reg::RxHeaderCntValueMsb, RW::Error> {
        Ok(reg::RxHeaderCntValueMsb(self.try_read(REG_RX_HEADER_CNT_VALUE_MSB)?))
    }
    pub fn try_set_rx_header_cnt_value_msb(&self, value: reg::RxHeaderCntValueMsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_HEADER_CNT_VALUE_MSB, value.0)
    }
    pub fn try_with_rx_header_cnt_value_msb<F: FnOnce(reg::RxHeaderCntValueMsb) -> reg::RxHeaderCntValueMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RxHeaderCntValueMsb(self.try_read(REG_RX_HEADER_CNT_VALUE_MSB)?);
        self.try_write(REG_RX_HEADER_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn try_rx_header_cnt_value_lsb(&self) -> Result<reg::RxHeaderCntValueLsb, RW::Error> {
        Ok(reg::RxHeaderCntValueLsb(self.try_read(REG_RX_HEADER_CNT_VALUE_LSB)?))
    }
    pub fn try_set_rx_header_cnt_value_lsb(&self, value: reg::RxHeaderCntValueLsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_HEADER_CNT_VALUE_LSB, value.0)
    }
    pub fn try_with_rx_header_cnt_value_lsb<F: FnOnce(reg::RxHeaderCntValueLsb) -> reg::RxHeaderCntValueLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RxHeaderCntValueLsb(self.try_read(REG_RX_HEADER_CNT_VALUE_LSB)?);
        self.try_write(REG_RX_HEADER_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn try_rx_packet_cnt_value_msb(&self) -> Result<reg::RxPacketCntValueMsb, RW::Error> {
        Ok(reg::RxPacketCntValueMsb(self.try_read(REG_RX_PACKET_CNT_VALUE_MSB)?))
    }
    pub fn try_set_rx_packet_cnt_value_msb(&self, value: reg::RxPacketCntValueMsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_PACKET_CNT_VALUE_MSB, value.0)
    }
    pub fn try_with_rx_packet_cnt_value_msb<F: FnOnce(reg::RxPacketCntValueMsb) -> reg::RxPacketCntValueMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RxPacketCntValueMsb(self.try_read(REG_RX_PACKET_CNT_VALUE_MSB)?);
        self.try_write(REG_RX_PACKET_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn try_rx_packet_cnt_value_lsb(&self) -> Result<reg::RxPacketCntValueLsb, RW::Error> {
        Ok(reg::RxPacketCntValueLsb(self.try_read(REG_RX_PACKET_CNT_VALUE_LSB)?))
    }
    pub fn try_set_rx_packet_cnt_value_lsb(&self, value: reg::RxPacketCntValueLsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_PACKET_CNT_VALUE_LSB, value.0)
    }
    pub fn try_with_rx_packet_cnt_value_lsb<F: FnOnce(reg::RxPacketCntValueLsb) -> reg::RxPacketCntValueLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RxPacketCntValueLsb(self.try_read(REG_RX_PACKET_CNT_VALUE_LSB)?);
        self.try_write(REG_RX_PACKET_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn try_modem_stat(&self) -> Result<reg::ModemStat, RW::Error> {
        Ok(reg::ModemStat(self.try_read(REG_MODEM_STAT)?))
    }
    pub fn try_set_modem_stat(&self, value: reg::ModemStat) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_STAT, value.0)
    }
    pub fn try_with_modem_stat<F: FnOnce(reg::ModemStat) -> reg::ModemStat>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::ModemStat(self.try_read(REG_MODEM_STAT)?);
        self.try_write(REG_MODEM_STAT, f(tmp).0)
    }

    pub fn try_pkt_snr_value(&self) -> Result<reg::PktSnrValue, RW::Error> {
        Ok(reg::PktSnrValue(self.try_read(REG_PKT_SNR_VALUE)?))
    }
    pub fn try_set_pkt_snr_value(&self, value: reg::PktSnrValue) -> Result<(), RW::Error> {
        self.try_write(REG_PKT_SNR_VALUE, value.0)
    }
    pub fn try_with_pkt_snr_value<F: FnOnce(reg::PktSnrValue) -> reg::PktSnrValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PktSnrValue(self.try_read(REG_PKT_SNR_VALUE)?);
        self.try_write(REG_PKT_SNR_VALUE, f(tmp).0)
    }

    pub fn try_pkt_rssi_value(&self) -> Result<reg::PktRssiValue, RW::Error> {
        Ok(reg::PktRssiValue(self.try_read(REG_PKT_RSSI_VALUE)?))
    }
    pub fn try_set_pkt_rssi_value(&self, value: reg::PktRssiValue) -> Result<(), RW::Error> {
        self.try_write(REG_PKT_RSSI_VALUE, value.0)
    }
    pub fn try_with_pkt_rssi_value<F: FnOnce(reg::PktRssiValue) -> reg::PktRssiValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PktRssiValue(self.try_read(REG_PKT_RSSI_VALUE)?);
        self.try_write(REG_PKT_RSSI_VALUE, f(tmp).0)
    }

    pub fn try_rssi_value(&self) -> Result<reg::RssiValue, RW::Error> {
        Ok(reg::RssiValue(self.try_read(REG_RSSI_VALUE)?))
    }
    pub fn try_set_rssi_value(&self, value: reg::RssiValue) -> Result<(), RW::Error> {
        self.try_write(REG_RSSI_VALUE, value.0)
    }
    pub fn try_with_rssi_value<F: FnOnce(reg::RssiValue) -> reg::RssiValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RssiValue(self.try_read(REG_RSSI_VALUE)?);
        self.try_write(REG_RSSI_VALUE, f(tmp).0)
    }

    pub fn try_hop_channel(&self) -> Result<reg::HopChannel, RW::Error> {
        Ok(reg::HopChannel(self.try_read(REG_HOP_CHANNEL)?))
    }
    pub fn try_set_hop_channel(&self, value: reg::HopChannel) -> Result<(), RW::Error> {
        self.try_write(REG_HOP_CHANNEL, value.0)
    }
    pub fn try_with_hop_channel<F: FnOnce(reg::HopChannel) -> reg::HopChannel>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::HopChannel(self.try_read(REG_HOP_CHANNEL)?);
        self.try_write(REG_HOP_CHANNEL, f(tmp).0)
    }

    pub fn try_modem_config1(&self) -> Result<reg::ModemConfig1, RW::Error> {
        Ok(reg::ModemConfig1(self.try_read(REG_MODEM_CONFIG1)?))
    }
    pub fn try_set_modem_config1(&self, value: reg::ModemConfig1) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG1, value.0)
    }
    pub fn try_with_modem_config1<F: FnOnce(reg::ModemConfig1) -> reg::ModemConfig1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::ModemConfig1(self.try_read(REG_MODEM_CONFIG1)?);
        self.try_write(REG_MODEM_CONFIG1, f(tmp).0)
    }

    pub fn try_modem_config2(&self) -> Result<reg::ModemConfig2, RW::Error> {
        Ok(reg::ModemConfig2(self.try_read(REG_MODEM_CONFIG2)?))
    }
    pub fn try_set_modem_config2(&self, value: reg::ModemConfig2) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG2, value.0)
    }
    pub fn try_with_modem_config2<F: FnOnce(reg::ModemConfig2) -> reg::ModemConfig2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::ModemConfig2(self.try_read(REG_MODEM_CONFIG2)?);
        self.try_write(REG_MODEM_CONFIG2, f(tmp).0)
    }

    pub fn try_symb_timeout_lsb(&self) -> Result<reg::SymbTimeoutLsb, RW::Error> {
        Ok(reg::SymbTimeoutLsb(self.try_read(REG_SYMB_TIMEOUT_LSB)?))
    }
    pub fn try_set_symb_timeout_lsb(&self, value: reg::SymbTimeoutLsb) -> Result<(), RW::Error> {
        self.try_write(REG_SYMB_TIMEOUT_LSB, value.0)
    }
    pub fn try_with_symb_timeout_lsb<F: FnOnce(reg::SymbTimeoutLsb) -> reg::SymbTimeoutLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::SymbTimeoutLsb(self.try_read(REG_SYMB_TIMEOUT_LSB)?);
        self.try_write(REG_SYMB_TIMEOUT_LSB, f(tmp).0)
    }

    pub fn try_preamble_msb(&self) -> Result<reg::PreambleMsb, RW::Error> {
        Ok(reg::PreambleMsb(self.try_read(REG_PREAMBLE_MSB)?))
    }
    pub fn try_set_preamble_msb(&self, value: reg::PreambleMsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLE_MSB, value.0)
    }
    pub fn try_with_preamble_msb<F: FnOnce(reg::PreambleMsb) -> reg::PreambleMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PreambleMsb(self.try_read(REG_PREAMBLE_MSB)?);
        self.try_write(REG_PREAMBLE_MSB, f(tmp).0)
    }

    pub fn try_preamble_lsb(&self) -> Result<reg::PreambleLsb, RW::Error> {
        Ok(reg::PreambleLsb(self.try_read(REG_PREAMBLE_LSB)?))
    }
    pub fn try_set_preamble_lsb(&self, value: reg::PreambleLsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLE_LSB, value.0)
    }
    pub fn try_with_preamble_lsb<F: FnOnce(reg::PreambleLsb) -> reg::PreambleLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PreambleLsb(self.try_read(REG_PREAMBLE_LSB)?);
        self.try_write(REG_PREAMBLE_LSB, f(tmp).0)
    }

    pub fn try_payload_length(&self) -> Result<reg::PayloadLength, RW::Error> {
        Ok(reg::PayloadLength(self.try_read(REG_PAYLOAD_LENGTH)?))
    }
    pub fn try_set_payload_length(&self, value: reg::PayloadLength) -> Result<(), RW::Error> {
        self.try_write(REG_PAYLOAD_LENGTH, value.0)
    }
    pub fn try_with_payload_length<F: FnOnce(reg::PayloadLength) -> reg::PayloadLength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PayloadLength(self.try_read(REG_PAYLOAD_LENGTH)?);
        self.try_write(REG_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn try_max_payload_length(&self) -> Result<reg::MaxPayloadLength, RW::Error> {
        Ok(reg::MaxPayloadLength(self.try_read(REG_MAX_PAYLOAD_LENGTH)?))
    }
    pub fn try_set_max_payload_length(&self, value: reg::MaxPayloadLength) -> Result<(), RW::Error> {
        self.try_write(REG_MAX_PAYLOAD_LENGTH, value.0)
    }
    pub fn try_with_max_payload_length<F: FnOnce(reg::MaxPayloadLength) -> reg::MaxPayloadLength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::MaxPayloadLength(self.try_read(REG_MAX_PAYLOAD_LENGTH)?);
        self.try_write(REG_MAX_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn try_hop_period(&self) -> Result<reg::HopPeriod, RW::Error> {
        Ok(reg::HopPeriod(self.try_read(REG_HOP_PERIOD)?))
    }
    pub fn try_set_hop_period(&self, value: reg::HopPeriod) -> Result<(), RW::Error> {
        self.try_write(REG_HOP_PERIOD, value.0)
    }
    pub fn try_with_hop_period<F: FnOnce(reg::HopPeriod) -> reg::HopPeriod>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::HopPeriod(self.try_read(REG_HOP_PERIOD)?);
        self.try_write(REG_HOP_PERIOD, f(tmp).0)
    }

    pub fn try_fifo_rx_byte_addr(&self) -> Result<reg::FifoRxByteAddr, RW::Error> {
        Ok(reg::FifoRxByteAddr(self.try_read(REG_FIFO_RX_BYTE_ADDR)?))
    }
    pub fn try_set_fifo_rx_byte_addr(&self, value: reg::FifoRxByteAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_BYTE_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_byte_addr<F: FnOnce(reg::FifoRxByteAddr) -> reg::FifoRxByteAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FifoRxByteAddr(self.try_read(REG_FIFO_RX_BYTE_ADDR)?);
        self.try_write(REG_FIFO_RX_BYTE_ADDR, f(tmp).0)
    }

    pub fn try_modem_config3(&self) -> Result<reg::ModemConfig3, RW::Error> {
        Ok(reg::ModemConfig3(self.try_read(REG_MODEM_CONFIG3)?))
    }
    pub fn try_set_modem_config3(&self, value: reg::ModemConfig3) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG3, value.0)
    }
    pub fn try_with_modem_config3<F: FnOnce(reg::ModemConfig3) -> reg::ModemConfig3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::ModemConfig3(self.try_read(REG_MODEM_CONFIG3)?);
        self.try_write(REG_MODEM_CONFIG3, f(tmp).0)
    }

    pub fn try_ppm_correction(&self) -> Result<reg::PpmCorrection, RW::Error> {
        Ok(reg::PpmCorrection(self.try_read(REG_PPM_CORRECTION)?))
    }
    pub fn try_set_ppm_correction(&self, value: reg::PpmCorrection) -> Result<(), RW::Error> {
        self.try_write(REG_PPM_CORRECTION, value.0)
    }
    pub fn try_with_ppm_correction<F: FnOnce(reg::PpmCorrection) -> reg::PpmCorrection>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PpmCorrection(self.try_read(REG_PPM_CORRECTION)?);
        self.try_write(REG_PPM_CORRECTION, f(tmp).0)
    }

    pub fn try_fei_msb(&self) -> Result<reg::FeiMsb, RW::Error> {
        Ok(reg::FeiMsb(self.try_read(REG_FEI_MSB)?))
    }
    pub fn try_set_fei_msb(&self, value: reg::FeiMsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_MSB, value.0)
    }
    pub fn try_with_fei_msb<F: FnOnce(reg::FeiMsb) -> reg::FeiMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FeiMsb(self.try_read(REG_FEI_MSB)?);
        self.try_write(REG_FEI_MSB, f(tmp).0)
    }

    pub fn try_fei_mid(&self) -> Result<reg::FeiMid, RW::Error> {
        Ok(reg::FeiMid(self.try_read(REG_FEI_MID)?))
    }
    pub fn try_set_fei_mid(&self, value: reg::FeiMid) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_MID, value.0)
    }
    pub fn try_with_fei_mid<F: FnOnce(reg::FeiMid) -> reg::FeiMid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FeiMid(self.try_read(REG_FEI_MID)?);
        self.try_write(REG_FEI_MID, f(tmp).0)
    }

    pub fn try_fei_lsb(&self) -> Result<reg::FeiLsb, RW::Error> {
        Ok(reg::FeiLsb(self.try_read(REG_FEI_LSB)?))
    }
    pub fn try_set_fei_lsb(&self, value: reg::FeiLsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_LSB, value.0)
    }
    pub fn try_with_fei_lsb<F: FnOnce(reg::FeiLsb) -> reg::FeiLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FeiLsb(self.try_read(REG_FEI_LSB)?);
        self.try_write(REG_FEI_LSB, f(tmp).0)
    }

    pub fn try_rssi_wideband(&self) -> Result<reg::RssiWideband, RW::Error> {
        Ok(reg::RssiWideband(self.try_read(REG_RSSI_WIDEBAND)?))
    }
    pub fn try_set_rssi_wideband(&self, value: reg::RssiWideband) -> Result<(), RW::Error> {
        self.try_write(REG_RSSI_WIDEBAND, value.0)
    }
    pub fn try_with_rssi_wideband<F: FnOnce(reg::RssiWideband) -> reg::RssiWideband>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::RssiWideband(self.try_read(REG_RSSI_WIDEBAND)?);
        self.try_write(REG_RSSI_WIDEBAND, f(tmp).0)
    }

    pub fn try_detect_optimize(&self) -> Result<reg::DetectOptimize, RW::Error> {
        Ok(reg::DetectOptimize(self.try_read(REG_DETECT_OPTIMIZE)?))
    }
    pub fn try_set_detect_optimize(&self, value: reg::DetectOptimize) -> Result<(), RW::Error> {
        self.try_write(REG_DETECT_OPTIMIZE, value.0)
    }
    pub fn try_with_detect_optimize<F: FnOnce(reg::DetectOptimize) -> reg::DetectOptimize>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::DetectOptimize(self.try_read(REG_DETECT_OPTIMIZE)?);
        self.try_write(REG_DETECT_OPTIMIZE, f(tmp).0)
    }

    pub fn try_invert_iq(&self) -> Result<reg::InvertIq, RW::Error> {
        Ok(reg::InvertIq(self.try_read(REG_INVERT_IQ)?))
    }
    pub fn try_set_invert_iq(&self, value: reg::InvertIq) -> Result<(), RW::Error> {
        self.try_write(REG_INVERT_IQ, value.0)
    }
    pub fn try_with_invert_iq<F: FnOnce(reg::InvertIq) -> reg::InvertIq>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::InvertIq(self.try_read(REG_INVERT_IQ)?);
        self.try_write(REG_INVERT_IQ, f(tmp).0)
    }

    pub fn try_detection_threshold(&self) -> Result<reg::DetectionThreshold, RW::Error> {
        Ok(reg::DetectionThreshold(self.try_read(REG_DETECTION_THRESHOLD)?))
    }
    pub fn try_set_detection_threshold(&self, value: reg::DetectionThreshold) -> Result<(), RW::Error> {
        self.try_write(REG_DETECTION_THRESHOLD, value.0)
    }
    pub fn try_with_detection_threshold<F: FnOnce(reg::DetectionThreshold) -> reg::DetectionThreshold>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::DetectionThreshold(self.try_read(REG_DETECTION_THRESHOLD)?);
        self.try_write(REG_DETECTION_THRESHOLD, f(tmp).0)
    }

    pub fn try_sync_word(&self) -> Result<reg::SyncWord, RW::Error> {
        Ok(reg::SyncWord(self.try_read(REG_SYNC_WORD)?))
    }
    pub fn try_set_sync_word(&self, value: reg::SyncWord) -> Result<(), RW::Error> {
        self.try_write(REG_SYNC_WORD, value.0)
    }
    pub fn try_with_sync_word<F: FnOnce(reg::SyncWord) -> reg::SyncWord>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::SyncWord(self.try_read(REG_SYNC_WORD)?);
        self.try_write(REG_SYNC_WORD, f(tmp).0)
    }

    pub fn try_dio_mapping1(&self) -> Result<reg::DioMapping1, RW::Error> {
        Ok(reg::DioMapping1(self.try_read(REG_DIO_MAPPING1)?))
    }
    pub fn try_set_dio_mapping1(&self, value: reg::DioMapping1) -> Result<(), RW::Error> {
        self.try_write(REG_DIO_MAPPING1, value.0)
    }
    pub fn try_with_dio_mapping1<F: FnOnce(reg::DioMapping1) -> reg::DioMapping1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::DioMapping1(self.try_read(REG_DIO_MAPPING1)?);
        self.try_write(REG_DIO_MAPPING1, f(tmp).0)
    }

    pub fn try_dio_mapping2(&self) -> Result<reg::DioMapping2, RW::Error> {
        Ok(reg::DioMapping2(self.try_read(REG_DIO_MAPPING2)?))
    }
    pub fn try_set_dio_mapping2(&self, value: reg::DioMapping2) -> Result<(), RW::Error> {
        self.try_write(REG_DIO_MAPPING2, value.0)
    }
    pub fn try_with_dio_mapping2<F: FnOnce(reg::DioMapping2) -> reg::DioMapping2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::DioMapping2(self.try_read(REG_DIO_MAPPING2)?);
        self.try_write(REG_DIO_MAPPING2, f(tmp).0)
    }

    pub fn try_version(&self) -> Result<reg::Version, RW::Error> {
        Ok(reg::Version(self.try_read(REG_VERSION)?))
    }
    pub fn try_set_version(&self, value: reg::Version) -> Result<(), RW::Error> {
        self.try_write(REG_VERSION, value.0)
    }
    pub fn try_with_version<F: FnOnce(reg::Version) -> reg::Version>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Version(self.try_read(REG_VERSION)?);
        self.try_write(REG_VERSION, f(tmp).0)
    }

    pub fn try_pll_hop(&self) -> Result<reg::PllHop, RW::Error> {
        Ok(reg::PllHop(self.try_read(REG_PLL_HOP)?))
    }
    pub fn try_set_pll_hop(&self, value: reg::PllHop) -> Result<(), RW::Error> {
        self.try_write(REG_PLL_HOP, value.0)
    }
    pub fn try_with_pll_hop<F: FnOnce(reg::PllHop) -> reg::PllHop>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PllHop(self.try_read(REG_PLL_HOP)?);
        self.try_write(REG_PLL_HOP, f(tmp).0)
    }

    pub fn try_tcxo(&self) -> Result<reg::Tcxo, RW::Error> {
        Ok(reg::Tcxo(self.try_read(REG_TCXO)?))
    }
    pub fn try_set_tcxo(&self, value: reg::Tcxo) -> Result<(), RW::Error> {
        self.try_write(REG_TCXO, value.0)
    }
    pub fn try_with_tcxo<F: FnOnce(reg::Tcxo) -> reg::Tcxo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Tcxo(self.try_read(REG_TCXO)?);
        self.try_write(REG_TCXO, f(tmp).0)
    }

    pub fn try_pa_dac(&self) -> Result<reg::PaDac, RW::Error> {
        Ok(reg::PaDac(self.try_read(REG_PA_DAC)?))
    }
    pub fn try_set_pa_dac(&self, value: reg::PaDac) -> Result<(), RW::Error> {
        self.try_write(REG_PA_DAC, value.0)
    }
    pub fn try_with_pa_dac<F: FnOnce(reg::PaDac) -> reg::PaDac>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PaDac(self.try_read(REG_PA_DAC)?);
        self.try_write(REG_PA_DAC, f(tmp).0)
    }

    pub fn try_former_temp(&self) -> Result<reg::FormerTemp, RW::Error> {
        Ok(reg::FormerTemp(self.try_read(REG_FORMER_TEMP)?))
    }
    pub fn try_set_former_temp(&self, value: reg::FormerTemp) -> Result<(), RW::Error> {
        self.try_write(REG_FORMER_TEMP, value.0)
    }
    pub fn try_with_former_temp<F: FnOnce(reg::FormerTemp) -> reg::FormerTemp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::FormerTemp(self.try_read(REG_FORMER_TEMP)?);
        self.try_write(REG_FORMER_TEMP, f(tmp).0)
    }

    pub fn try_agc_ref(&self) -> Result<reg::AgcRef, RW::Error> {
        Ok(reg::AgcRef(self.try_read(REG_AGC_REF)?))
    }
    pub fn try_set_agc_ref(&self, value: reg::AgcRef) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_REF, value.0)
    }
    pub fn try_with_agc_ref<F: FnOnce(reg::AgcRef) -> reg::AgcRef>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::AgcRef(self.try_read(REG_AGC_REF)?);
        self.try_write(REG_AGC_REF, f(tmp).0)
    }

    pub fn try_agc_thresh1(&self) -> Result<reg::AgcThresh1, RW::Error> {
        Ok(reg::AgcThresh1(self.try_read(REG_AGC_THRESH1)?))
    }
    pub fn try_set_agc_thresh1(&self, value: reg::AgcThresh1) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH1, value.0)
    }
    pub fn try_with_agc_thresh1<F: FnOnce(reg::AgcThresh1) -> reg::AgcThresh1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::AgcThresh1(self.try_read(REG_AGC_THRESH1)?);
        self.try_write(REG_AGC_THRESH1, f(tmp).0)
    }

    pub fn try_agc_thresh2(&self) -> Result<reg::AgcThresh2, RW::Error> {
        Ok(reg::AgcThresh2(self.try_read(REG_AGC_THRESH2)?))
    }
    pub fn try_set_agc_thresh2(&self, value: reg::AgcThresh2) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH2, value.0)
    }
    pub fn try_with_agc_thresh2<F: FnOnce(reg::AgcThresh2) -> reg::AgcThresh2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::AgcThresh2(self.try_read(REG_AGC_THRESH2)?);
        self.try_write(REG_AGC_THRESH2, f(tmp).0)
    }

    pub fn try_agc_thresh3(&self) -> Result<reg::AgcThresh3, RW::Error> {
        Ok(reg::AgcThresh3(self.try_read(REG_AGC_THRESH3)?))
    }
    pub fn try_set_agc_thresh3(&self, value: reg::AgcThresh3) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH3, value.0)
    }
    pub fn try_with_agc_thresh3<F: FnOnce(reg::AgcThresh3) -> reg::AgcThresh3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::AgcThresh3(self.try_read(REG_AGC_THRESH3)?);
        self.try_write(REG_AGC_THRESH3, f(tmp).0)
    }

    pub fn try_pll_hf(&self) -> Result<reg::PllHf, RW::Error> {
        Ok(reg::PllHf(self.try_read(REG_PLL_HF)?))
    }
    pub fn try_set_pll_hf(&self, value: reg::PllHf) -> Result<(), RW::Error> {
        self.try_write(REG_PLL_HF, value.0)
    }
    pub fn try_with_pll_hf<F: FnOnce(reg::PllHf) -> reg::PllHf>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::PllHf(self.try_read(REG_PLL_HF)?);
        self.try_write(REG_PLL_HF, f(tmp).0)
    }

}

pub mod reg {
    use ::bobbin_bits as bits;

#[doc="FIFO read/write access"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u8);
impl Fifo {
    #[doc="FIFO data input/output"]
    #[inline] pub fn fifo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO != 0"]
    #[inline] pub fn test_fifo(&self) -> bool {
        self.fifo() != 0
    }

    #[doc="Sets the FIFO field."]
    #[inline] pub fn set_fifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fifo {
    #[inline]
    fn from(other: u8) -> Self {
         Fifo(other)
    }
}

impl ::core::fmt::Display for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo() != 0 { try!(write!(f, " fifo=0x{:x}", self.fifo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opmode(pub u8);
impl Opmode {
    #[inline] pub fn long_range_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LONG_RANGE_MODE != 0"]
    #[inline] pub fn test_long_range_mode(&self) -> bool {
        self.long_range_mode() != 0
    }

    #[doc="Sets the LONG_RANGE_MODE field."]
    #[inline] pub fn set_long_range_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn access_shared_reg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ACCESS_SHARED_REG != 0"]
    #[inline] pub fn test_access_shared_reg(&self) -> bool {
        self.access_shared_reg() != 0
    }

    #[doc="Sets the ACCESS_SHARED_REG field."]
    #[inline] pub fn set_access_shared_reg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn low_frequency_mode_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOW_FREQUENCY_MODE_ON != 0"]
    #[inline] pub fn test_low_frequency_mode_on(&self) -> bool {
        self.low_frequency_mode_on() != 0
    }

    #[doc="Sets the LOW_FREQUENCY_MODE_ON field."]
    #[inline] pub fn set_low_frequency_mode_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn mode(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Opmode {
    #[inline]
    fn from(other: u8) -> Self {
         Opmode(other)
    }
}

impl ::core::fmt::Display for Opmode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Opmode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.long_range_mode() != 0 { try!(write!(f, " long_range_mode"))}
        if self.access_shared_reg() != 0 { try!(write!(f, " access_shared_reg"))}
        if self.low_frequency_mode_on() != 0 { try!(write!(f, " low_frequency_mode_on"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MSB of RF carrier frequency"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FrfMsb(pub u8);
impl FrfMsb {
    #[inline] pub fn frf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRF != 0"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf() != 0
    }

    #[doc="Sets the FRF field."]
    #[inline] pub fn set_frf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FrfMsb {
    #[inline]
    fn from(other: u8) -> Self {
         FrfMsb(other)
    }
}

impl ::core::fmt::Display for FrfMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FrfMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MSB of RF carrier frequency"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FrfMid(pub u8);
impl FrfMid {
    #[inline] pub fn frf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRF != 0"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf() != 0
    }

    #[doc="Sets the FRF field."]
    #[inline] pub fn set_frf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FrfMid {
    #[inline]
    fn from(other: u8) -> Self {
         FrfMid(other)
    }
}

impl ::core::fmt::Display for FrfMid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FrfMid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LSB of RF carrier frequency"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FrfLsb(pub u8);
impl FrfLsb {
    #[inline] pub fn frf(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRF != 0"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf() != 0
    }

    #[doc="Sets the FRF field."]
    #[inline] pub fn set_frf<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FrfLsb {
    #[inline]
    fn from(other: u8) -> Self {
         FrfLsb(other)
    }
}

impl ::core::fmt::Display for FrfLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FrfLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PaConfig(pub u8);
impl PaConfig {
    #[inline] pub fn pa_select(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PA_SELECT != 0"]
    #[inline] pub fn test_pa_select(&self) -> bool {
        self.pa_select() != 0
    }

    #[doc="Sets the PA_SELECT field."]
    #[inline] pub fn set_pa_select<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn max_power(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if MAX_POWER != 0"]
    #[inline] pub fn test_max_power(&self) -> bool {
        self.max_power() != 0
    }

    #[doc="Sets the MAX_POWER field."]
    #[inline] pub fn set_max_power<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn output_power(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if OUTPUT_POWER != 0"]
    #[inline] pub fn test_output_power(&self) -> bool {
        self.output_power() != 0
    }

    #[doc="Sets the OUTPUT_POWER field."]
    #[inline] pub fn set_output_power<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PaConfig {
    #[inline]
    fn from(other: u8) -> Self {
         PaConfig(other)
    }
}

impl ::core::fmt::Display for PaConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PaConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa_select() != 0 { try!(write!(f, " pa_select"))}
        if self.max_power() != 0 { try!(write!(f, " max_power=0x{:x}", self.max_power()))}
        if self.output_power() != 0 { try!(write!(f, " output_power=0x{:x}", self.output_power()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PaRamp(pub u8);
impl PaRamp {
    #[inline] pub fn pa_ramp(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PA_RAMP != 0"]
    #[inline] pub fn test_pa_ramp(&self) -> bool {
        self.pa_ramp() != 0
    }

    #[doc="Sets the PA_RAMP field."]
    #[inline] pub fn set_pa_ramp<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PaRamp {
    #[inline]
    fn from(other: u8) -> Self {
         PaRamp(other)
    }
}

impl ::core::fmt::Display for PaRamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PaRamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa_ramp() != 0 { try!(write!(f, " pa_ramp=0x{:x}", self.pa_ramp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ocp(pub u8);
impl Ocp {
    #[inline] pub fn ocp_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OCP_ON != 0"]
    #[inline] pub fn test_ocp_on(&self) -> bool {
        self.ocp_on() != 0
    }

    #[doc="Sets the OCP_ON field."]
    #[inline] pub fn set_ocp_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn ocp_trim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if OCP_TRIM != 0"]
    #[inline] pub fn test_ocp_trim(&self) -> bool {
        self.ocp_trim() != 0
    }

    #[doc="Sets the OCP_TRIM field."]
    #[inline] pub fn set_ocp_trim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ocp {
    #[inline]
    fn from(other: u8) -> Self {
         Ocp(other)
    }
}

impl ::core::fmt::Display for Ocp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ocp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ocp_on() != 0 { try!(write!(f, " ocp_on"))}
        if self.ocp_trim() != 0 { try!(write!(f, " ocp_trim=0x{:x}", self.ocp_trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lna(pub u8);
impl Lna {
    #[inline] pub fn lna_gain(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if LNA_GAIN != 0"]
    #[inline] pub fn test_lna_gain(&self) -> bool {
        self.lna_gain() != 0
    }

    #[doc="Sets the LNA_GAIN field."]
    #[inline] pub fn set_lna_gain<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn lna_boost_lf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if LNA_BOOST_LF != 0"]
    #[inline] pub fn test_lna_boost_lf(&self) -> bool {
        self.lna_boost_lf() != 0
    }

    #[doc="Sets the LNA_BOOST_LF field."]
    #[inline] pub fn set_lna_boost_lf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn lna_boost_hf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if LNA_BOOST_HF != 0"]
    #[inline] pub fn test_lna_boost_hf(&self) -> bool {
        self.lna_boost_hf() != 0
    }

    #[doc="Sets the LNA_BOOST_HF field."]
    #[inline] pub fn set_lna_boost_hf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Lna {
    #[inline]
    fn from(other: u8) -> Self {
         Lna(other)
    }
}

impl ::core::fmt::Display for Lna {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lna {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lna_gain() != 0 { try!(write!(f, " lna_gain=0x{:x}", self.lna_gain()))}
        if self.lna_boost_lf() != 0 { try!(write!(f, " lna_boost_lf=0x{:x}", self.lna_boost_lf()))}
        if self.lna_boost_hf() != 0 { try!(write!(f, " lna_boost_hf=0x{:x}", self.lna_boost_hf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FifoAddrPtr(pub u8);
impl FifoAddrPtr {
    #[inline] pub fn fifo_addr_ptr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_ADDR_PTR != 0"]
    #[inline] pub fn test_fifo_addr_ptr(&self) -> bool {
        self.fifo_addr_ptr() != 0
    }

    #[doc="Sets the FIFO_ADDR_PTR field."]
    #[inline] pub fn set_fifo_addr_ptr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FifoAddrPtr {
    #[inline]
    fn from(other: u8) -> Self {
         FifoAddrPtr(other)
    }
}

impl ::core::fmt::Display for FifoAddrPtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FifoAddrPtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_addr_ptr() != 0 { try!(write!(f, " fifo_addr_ptr=0x{:x}", self.fifo_addr_ptr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FifoTxBaseAddr(pub u8);
impl FifoTxBaseAddr {
    #[inline] pub fn fifo_tx_base_addr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_TX_BASE_ADDR != 0"]
    #[inline] pub fn test_fifo_tx_base_addr(&self) -> bool {
        self.fifo_tx_base_addr() != 0
    }

    #[doc="Sets the FIFO_TX_BASE_ADDR field."]
    #[inline] pub fn set_fifo_tx_base_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FifoTxBaseAddr {
    #[inline]
    fn from(other: u8) -> Self {
         FifoTxBaseAddr(other)
    }
}

impl ::core::fmt::Display for FifoTxBaseAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FifoTxBaseAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_tx_base_addr() != 0 { try!(write!(f, " fifo_tx_base_addr=0x{:x}", self.fifo_tx_base_addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FifoRxBaseAddr(pub u8);
impl FifoRxBaseAddr {
    #[inline] pub fn fifo_rx_base_addr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_RX_BASE_ADDR != 0"]
    #[inline] pub fn test_fifo_rx_base_addr(&self) -> bool {
        self.fifo_rx_base_addr() != 0
    }

    #[doc="Sets the FIFO_RX_BASE_ADDR field."]
    #[inline] pub fn set_fifo_rx_base_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FifoRxBaseAddr {
    #[inline]
    fn from(other: u8) -> Self {
         FifoRxBaseAddr(other)
    }
}

impl ::core::fmt::Display for FifoRxBaseAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FifoRxBaseAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_rx_base_addr() != 0 { try!(write!(f, " fifo_rx_base_addr=0x{:x}", self.fifo_rx_base_addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FifoRxCurrentAddr(pub u8);
impl FifoRxCurrentAddr {
    #[inline] pub fn fifo_rx_current_addr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_RX_CURRENT_ADDR != 0"]
    #[inline] pub fn test_fifo_rx_current_addr(&self) -> bool {
        self.fifo_rx_current_addr() != 0
    }

    #[doc="Sets the FIFO_RX_CURRENT_ADDR field."]
    #[inline] pub fn set_fifo_rx_current_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FifoRxCurrentAddr {
    #[inline]
    fn from(other: u8) -> Self {
         FifoRxCurrentAddr(other)
    }
}

impl ::core::fmt::Display for FifoRxCurrentAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FifoRxCurrentAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_rx_current_addr() != 0 { try!(write!(f, " fifo_rx_current_addr=0x{:x}", self.fifo_rx_current_addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IrqFlagsMask(pub u8);
impl IrqFlagsMask {
    #[inline] pub fn rx_timeout_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RX_TIMEOUT_MASK != 0"]
    #[inline] pub fn test_rx_timeout_mask(&self) -> bool {
        self.rx_timeout_mask() != 0
    }

    #[doc="Sets the RX_TIMEOUT_MASK field."]
    #[inline] pub fn set_rx_timeout_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn rx_done_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RX_DONE_MASK != 0"]
    #[inline] pub fn test_rx_done_mask(&self) -> bool {
        self.rx_done_mask() != 0
    }

    #[doc="Sets the RX_DONE_MASK field."]
    #[inline] pub fn set_rx_done_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn payload_crc_error_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PAYLOAD_CRC_ERROR_MASK != 0"]
    #[inline] pub fn test_payload_crc_error_mask(&self) -> bool {
        self.payload_crc_error_mask() != 0
    }

    #[doc="Sets the PAYLOAD_CRC_ERROR_MASK field."]
    #[inline] pub fn set_payload_crc_error_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn valid_header_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if VALID_HEADER_MASK != 0"]
    #[inline] pub fn test_valid_header_mask(&self) -> bool {
        self.valid_header_mask() != 0
    }

    #[doc="Sets the VALID_HEADER_MASK field."]
    #[inline] pub fn set_valid_header_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn tx_done_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TX_DONE_MASK != 0"]
    #[inline] pub fn test_tx_done_mask(&self) -> bool {
        self.tx_done_mask() != 0
    }

    #[doc="Sets the TX_DONE_MASK field."]
    #[inline] pub fn set_tx_done_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn cad_done_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CAD_DONE_MASK != 0"]
    #[inline] pub fn test_cad_done_mask(&self) -> bool {
        self.cad_done_mask() != 0
    }

    #[doc="Sets the CAD_DONE_MASK field."]
    #[inline] pub fn set_cad_done_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn fhss_change_channel_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FHSS_CHANGE_CHANNEL_MASK != 0"]
    #[inline] pub fn test_fhss_change_channel_mask(&self) -> bool {
        self.fhss_change_channel_mask() != 0
    }

    #[doc="Sets the FHSS_CHANGE_CHANNEL_MASK field."]
    #[inline] pub fn set_fhss_change_channel_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn cad_detected_mask(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAD_DETECTED_MASK != 0"]
    #[inline] pub fn test_cad_detected_mask(&self) -> bool {
        self.cad_detected_mask() != 0
    }

    #[doc="Sets the CAD_DETECTED_MASK field."]
    #[inline] pub fn set_cad_detected_mask<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for IrqFlagsMask {
    #[inline]
    fn from(other: u8) -> Self {
         IrqFlagsMask(other)
    }
}

impl ::core::fmt::Display for IrqFlagsMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IrqFlagsMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_timeout_mask() != 0 { try!(write!(f, " rx_timeout_mask"))}
        if self.rx_done_mask() != 0 { try!(write!(f, " rx_done_mask"))}
        if self.payload_crc_error_mask() != 0 { try!(write!(f, " payload_crc_error_mask"))}
        if self.valid_header_mask() != 0 { try!(write!(f, " valid_header_mask"))}
        if self.tx_done_mask() != 0 { try!(write!(f, " tx_done_mask"))}
        if self.cad_done_mask() != 0 { try!(write!(f, " cad_done_mask"))}
        if self.fhss_change_channel_mask() != 0 { try!(write!(f, " fhss_change_channel_mask"))}
        if self.cad_detected_mask() != 0 { try!(write!(f, " cad_detected_mask"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IrqFlags(pub u8);
impl IrqFlags {
    #[inline] pub fn rx_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RX_TIMEOUT != 0"]
    #[inline] pub fn test_rx_timeout(&self) -> bool {
        self.rx_timeout() != 0
    }

    #[doc="Sets the RX_TIMEOUT field."]
    #[inline] pub fn set_rx_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn rx_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RX_DONE != 0"]
    #[inline] pub fn test_rx_done(&self) -> bool {
        self.rx_done() != 0
    }

    #[doc="Sets the RX_DONE field."]
    #[inline] pub fn set_rx_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn payload_crc_error(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PAYLOAD_CRC_ERROR != 0"]
    #[inline] pub fn test_payload_crc_error(&self) -> bool {
        self.payload_crc_error() != 0
    }

    #[doc="Sets the PAYLOAD_CRC_ERROR field."]
    #[inline] pub fn set_payload_crc_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn valid_header(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if VALID_HEADER != 0"]
    #[inline] pub fn test_valid_header(&self) -> bool {
        self.valid_header() != 0
    }

    #[doc="Sets the VALID_HEADER field."]
    #[inline] pub fn set_valid_header<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn tx_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TX_DONE != 0"]
    #[inline] pub fn test_tx_done(&self) -> bool {
        self.tx_done() != 0
    }

    #[doc="Sets the TX_DONE field."]
    #[inline] pub fn set_tx_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn cad_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CAD_DONE != 0"]
    #[inline] pub fn test_cad_done(&self) -> bool {
        self.cad_done() != 0
    }

    #[doc="Sets the CAD_DONE field."]
    #[inline] pub fn set_cad_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn fhss_change_channel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FHSS_CHANGE_CHANNEL != 0"]
    #[inline] pub fn test_fhss_change_channel(&self) -> bool {
        self.fhss_change_channel() != 0
    }

    #[doc="Sets the FHSS_CHANGE_CHANNEL field."]
    #[inline] pub fn set_fhss_change_channel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn cad_detected(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAD_DETECTED != 0"]
    #[inline] pub fn test_cad_detected(&self) -> bool {
        self.cad_detected() != 0
    }

    #[doc="Sets the CAD_DETECTED field."]
    #[inline] pub fn set_cad_detected<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for IrqFlags {
    #[inline]
    fn from(other: u8) -> Self {
         IrqFlags(other)
    }
}

impl ::core::fmt::Display for IrqFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for IrqFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_timeout() != 0 { try!(write!(f, " rx_timeout"))}
        if self.rx_done() != 0 { try!(write!(f, " rx_done"))}
        if self.payload_crc_error() != 0 { try!(write!(f, " payload_crc_error"))}
        if self.valid_header() != 0 { try!(write!(f, " valid_header"))}
        if self.tx_done() != 0 { try!(write!(f, " tx_done"))}
        if self.cad_done() != 0 { try!(write!(f, " cad_done"))}
        if self.fhss_change_channel() != 0 { try!(write!(f, " fhss_change_channel"))}
        if self.cad_detected() != 0 { try!(write!(f, " cad_detected"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RxNbBytes(pub u8);
impl RxNbBytes {
    #[inline] pub fn fifo_rx_bytes_nb(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_RX_BYTES_NB != 0"]
    #[inline] pub fn test_fifo_rx_bytes_nb(&self) -> bool {
        self.fifo_rx_bytes_nb() != 0
    }

    #[doc="Sets the FIFO_RX_BYTES_NB field."]
    #[inline] pub fn set_fifo_rx_bytes_nb<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RxNbBytes {
    #[inline]
    fn from(other: u8) -> Self {
         RxNbBytes(other)
    }
}

impl ::core::fmt::Display for RxNbBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RxNbBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_rx_bytes_nb() != 0 { try!(write!(f, " fifo_rx_bytes_nb=0x{:x}", self.fifo_rx_bytes_nb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RxHeaderCntValueMsb(pub u8);
impl RxHeaderCntValueMsb {
    #[inline] pub fn valid_header_cnt_msb(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VALID_HEADER_CNT_MSB != 0"]
    #[inline] pub fn test_valid_header_cnt_msb(&self) -> bool {
        self.valid_header_cnt_msb() != 0
    }

    #[doc="Sets the VALID_HEADER_CNT_MSB field."]
    #[inline] pub fn set_valid_header_cnt_msb<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RxHeaderCntValueMsb {
    #[inline]
    fn from(other: u8) -> Self {
         RxHeaderCntValueMsb(other)
    }
}

impl ::core::fmt::Display for RxHeaderCntValueMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RxHeaderCntValueMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.valid_header_cnt_msb() != 0 { try!(write!(f, " valid_header_cnt_msb=0x{:x}", self.valid_header_cnt_msb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RxHeaderCntValueLsb(pub u8);
impl RxHeaderCntValueLsb {
    #[inline] pub fn valid_header_cnt_lsb(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VALID_HEADER_CNT_LSB != 0"]
    #[inline] pub fn test_valid_header_cnt_lsb(&self) -> bool {
        self.valid_header_cnt_lsb() != 0
    }

    #[doc="Sets the VALID_HEADER_CNT_LSB field."]
    #[inline] pub fn set_valid_header_cnt_lsb<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RxHeaderCntValueLsb {
    #[inline]
    fn from(other: u8) -> Self {
         RxHeaderCntValueLsb(other)
    }
}

impl ::core::fmt::Display for RxHeaderCntValueLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RxHeaderCntValueLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.valid_header_cnt_lsb() != 0 { try!(write!(f, " valid_header_cnt_lsb=0x{:x}", self.valid_header_cnt_lsb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RxPacketCntValueMsb(pub u8);
impl RxPacketCntValueMsb {
    #[inline] pub fn valid_packet_cnt_msb(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VALID_PACKET_CNT_MSB != 0"]
    #[inline] pub fn test_valid_packet_cnt_msb(&self) -> bool {
        self.valid_packet_cnt_msb() != 0
    }

    #[doc="Sets the VALID_PACKET_CNT_MSB field."]
    #[inline] pub fn set_valid_packet_cnt_msb<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RxPacketCntValueMsb {
    #[inline]
    fn from(other: u8) -> Self {
         RxPacketCntValueMsb(other)
    }
}

impl ::core::fmt::Display for RxPacketCntValueMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RxPacketCntValueMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.valid_packet_cnt_msb() != 0 { try!(write!(f, " valid_packet_cnt_msb=0x{:x}", self.valid_packet_cnt_msb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RxPacketCntValueLsb(pub u8);
impl RxPacketCntValueLsb {
    #[inline] pub fn valid_packet_cnt_lsb(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VALID_PACKET_CNT_LSB != 0"]
    #[inline] pub fn test_valid_packet_cnt_lsb(&self) -> bool {
        self.valid_packet_cnt_lsb() != 0
    }

    #[doc="Sets the VALID_PACKET_CNT_LSB field."]
    #[inline] pub fn set_valid_packet_cnt_lsb<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RxPacketCntValueLsb {
    #[inline]
    fn from(other: u8) -> Self {
         RxPacketCntValueLsb(other)
    }
}

impl ::core::fmt::Display for RxPacketCntValueLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RxPacketCntValueLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.valid_packet_cnt_lsb() != 0 { try!(write!(f, " valid_packet_cnt_lsb=0x{:x}", self.valid_packet_cnt_lsb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModemStat(pub u8);
impl ModemStat {
    #[inline] pub fn rx_coding_rate(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if RX_CODING_RATE != 0"]
    #[inline] pub fn test_rx_coding_rate(&self) -> bool {
        self.rx_coding_rate() != 0
    }

    #[doc="Sets the RX_CODING_RATE field."]
    #[inline] pub fn set_rx_coding_rate<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn modem_status(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if MODEM_STATUS != 0"]
    #[inline] pub fn test_modem_status(&self) -> bool {
        self.modem_status() != 0
    }

    #[doc="Sets the MODEM_STATUS field."]
    #[inline] pub fn set_modem_status<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for ModemStat {
    #[inline]
    fn from(other: u8) -> Self {
         ModemStat(other)
    }
}

impl ::core::fmt::Display for ModemStat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ModemStat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx_coding_rate() != 0 { try!(write!(f, " rx_coding_rate=0x{:x}", self.rx_coding_rate()))}
        if self.modem_status() != 0 { try!(write!(f, " modem_status=0x{:x}", self.modem_status()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PktSnrValue(pub u8);
impl PktSnrValue {
    #[inline] pub fn packet_snr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PACKET_SNR != 0"]
    #[inline] pub fn test_packet_snr(&self) -> bool {
        self.packet_snr() != 0
    }

    #[doc="Sets the PACKET_SNR field."]
    #[inline] pub fn set_packet_snr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PktSnrValue {
    #[inline]
    fn from(other: u8) -> Self {
         PktSnrValue(other)
    }
}

impl ::core::fmt::Display for PktSnrValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PktSnrValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.packet_snr() != 0 { try!(write!(f, " packet_snr=0x{:x}", self.packet_snr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PktRssiValue(pub u8);
impl PktRssiValue {
    #[inline] pub fn packet_rssi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PACKET_RSSI != 0"]
    #[inline] pub fn test_packet_rssi(&self) -> bool {
        self.packet_rssi() != 0
    }

    #[doc="Sets the PACKET_RSSI field."]
    #[inline] pub fn set_packet_rssi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PktRssiValue {
    #[inline]
    fn from(other: u8) -> Self {
         PktRssiValue(other)
    }
}

impl ::core::fmt::Display for PktRssiValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PktRssiValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.packet_rssi() != 0 { try!(write!(f, " packet_rssi=0x{:x}", self.packet_rssi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RssiValue(pub u8);
impl RssiValue {
    #[inline] pub fn rssi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSSI != 0"]
    #[inline] pub fn test_rssi(&self) -> bool {
        self.rssi() != 0
    }

    #[doc="Sets the RSSI field."]
    #[inline] pub fn set_rssi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RssiValue {
    #[inline]
    fn from(other: u8) -> Self {
         RssiValue(other)
    }
}

impl ::core::fmt::Display for RssiValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RssiValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rssi() != 0 { try!(write!(f, " rssi=0x{:x}", self.rssi()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HopChannel(pub u8);
impl HopChannel {
    #[inline] pub fn pll_timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PLL_TIMEOUT != 0"]
    #[inline] pub fn test_pll_timeout(&self) -> bool {
        self.pll_timeout() != 0
    }

    #[doc="Sets the PLL_TIMEOUT field."]
    #[inline] pub fn set_pll_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn rx_payload_crc_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RX_PAYLOAD_CRC_ON != 0"]
    #[inline] pub fn test_rx_payload_crc_on(&self) -> bool {
        self.rx_payload_crc_on() != 0
    }

    #[doc="Sets the RX_PAYLOAD_CRC_ON field."]
    #[inline] pub fn set_rx_payload_crc_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn fhss_present_channel(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if FHSS_PRESENT_CHANNEL != 0"]
    #[inline] pub fn test_fhss_present_channel(&self) -> bool {
        self.fhss_present_channel() != 0
    }

    #[doc="Sets the FHSS_PRESENT_CHANNEL field."]
    #[inline] pub fn set_fhss_present_channel<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for HopChannel {
    #[inline]
    fn from(other: u8) -> Self {
         HopChannel(other)
    }
}

impl ::core::fmt::Display for HopChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HopChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pll_timeout() != 0 { try!(write!(f, " pll_timeout"))}
        if self.rx_payload_crc_on() != 0 { try!(write!(f, " rx_payload_crc_on"))}
        if self.fhss_present_channel() != 0 { try!(write!(f, " fhss_present_channel=0x{:x}", self.fhss_present_channel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModemConfig1(pub u8);
impl ModemConfig1 {
    #[inline] pub fn bw(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if BW != 0"]
    #[inline] pub fn test_bw(&self) -> bool {
        self.bw() != 0
    }

    #[doc="Sets the BW field."]
    #[inline] pub fn set_bw<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn coding_rate(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if CODING_RATE != 0"]
    #[inline] pub fn test_coding_rate(&self) -> bool {
        self.coding_rate() != 0
    }

    #[doc="Sets the CODING_RATE field."]
    #[inline] pub fn set_coding_rate<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn implicit_header_mode_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IMPLICIT_HEADER_MODE_ON != 0"]
    #[inline] pub fn test_implicit_header_mode_on(&self) -> bool {
        self.implicit_header_mode_on() != 0
    }

    #[doc="Sets the IMPLICIT_HEADER_MODE_ON field."]
    #[inline] pub fn set_implicit_header_mode_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for ModemConfig1 {
    #[inline]
    fn from(other: u8) -> Self {
         ModemConfig1(other)
    }
}

impl ::core::fmt::Display for ModemConfig1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ModemConfig1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bw() != 0 { try!(write!(f, " bw=0x{:x}", self.bw()))}
        if self.coding_rate() != 0 { try!(write!(f, " coding_rate=0x{:x}", self.coding_rate()))}
        if self.implicit_header_mode_on() != 0 { try!(write!(f, " implicit_header_mode_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModemConfig2(pub u8);
impl ModemConfig2 {
    #[inline] pub fn spreading_factor(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if SPREADING_FACTOR != 0"]
    #[inline] pub fn test_spreading_factor(&self) -> bool {
        self.spreading_factor() != 0
    }

    #[doc="Sets the SPREADING_FACTOR field."]
    #[inline] pub fn set_spreading_factor<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn tx_continuous_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TX_CONTINUOUS_MODE != 0"]
    #[inline] pub fn test_tx_continuous_mode(&self) -> bool {
        self.tx_continuous_mode() != 0
    }

    #[doc="Sets the TX_CONTINUOUS_MODE field."]
    #[inline] pub fn set_tx_continuous_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn rx_payload_crc_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RX_PAYLOAD_CRC_ON != 0"]
    #[inline] pub fn test_rx_payload_crc_on(&self) -> bool {
        self.rx_payload_crc_on() != 0
    }

    #[doc="Sets the RX_PAYLOAD_CRC_ON field."]
    #[inline] pub fn set_rx_payload_crc_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn symb_timeout(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SYMB_TIMEOUT != 0"]
    #[inline] pub fn test_symb_timeout(&self) -> bool {
        self.symb_timeout() != 0
    }

    #[doc="Sets the SYMB_TIMEOUT field."]
    #[inline] pub fn set_symb_timeout<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for ModemConfig2 {
    #[inline]
    fn from(other: u8) -> Self {
         ModemConfig2(other)
    }
}

impl ::core::fmt::Display for ModemConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ModemConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spreading_factor() != 0 { try!(write!(f, " spreading_factor=0x{:x}", self.spreading_factor()))}
        if self.tx_continuous_mode() != 0 { try!(write!(f, " tx_continuous_mode"))}
        if self.rx_payload_crc_on() != 0 { try!(write!(f, " rx_payload_crc_on"))}
        if self.symb_timeout() != 0 { try!(write!(f, " symb_timeout=0x{:x}", self.symb_timeout()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SymbTimeoutLsb(pub u8);
impl SymbTimeoutLsb {
    #[inline] pub fn symb_timeout(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SYMB_TIMEOUT != 0"]
    #[inline] pub fn test_symb_timeout(&self) -> bool {
        self.symb_timeout() != 0
    }

    #[doc="Sets the SYMB_TIMEOUT field."]
    #[inline] pub fn set_symb_timeout<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for SymbTimeoutLsb {
    #[inline]
    fn from(other: u8) -> Self {
         SymbTimeoutLsb(other)
    }
}

impl ::core::fmt::Display for SymbTimeoutLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for SymbTimeoutLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.symb_timeout() != 0 { try!(write!(f, " symb_timeout=0x{:x}", self.symb_timeout()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PreambleMsb(pub u8);
impl PreambleMsb {
    #[inline] pub fn preamble_length(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLE_LENGTH != 0"]
    #[inline] pub fn test_preamble_length(&self) -> bool {
        self.preamble_length() != 0
    }

    #[doc="Sets the PREAMBLE_LENGTH field."]
    #[inline] pub fn set_preamble_length<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PreambleMsb {
    #[inline]
    fn from(other: u8) -> Self {
         PreambleMsb(other)
    }
}

impl ::core::fmt::Display for PreambleMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PreambleMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preamble_length() != 0 { try!(write!(f, " preamble_length=0x{:x}", self.preamble_length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PreambleLsb(pub u8);
impl PreambleLsb {
    #[inline] pub fn preamble_length(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLE_LENGTH != 0"]
    #[inline] pub fn test_preamble_length(&self) -> bool {
        self.preamble_length() != 0
    }

    #[doc="Sets the PREAMBLE_LENGTH field."]
    #[inline] pub fn set_preamble_length<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PreambleLsb {
    #[inline]
    fn from(other: u8) -> Self {
         PreambleLsb(other)
    }
}

impl ::core::fmt::Display for PreambleLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PreambleLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preamble_length() != 0 { try!(write!(f, " preamble_length=0x{:x}", self.preamble_length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PayloadLength(pub u8);
impl PayloadLength {
    #[inline] pub fn payload_length(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PAYLOAD_LENGTH != 0"]
    #[inline] pub fn test_payload_length(&self) -> bool {
        self.payload_length() != 0
    }

    #[doc="Sets the PAYLOAD_LENGTH field."]
    #[inline] pub fn set_payload_length<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PayloadLength {
    #[inline]
    fn from(other: u8) -> Self {
         PayloadLength(other)
    }
}

impl ::core::fmt::Display for PayloadLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PayloadLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.payload_length() != 0 { try!(write!(f, " payload_length=0x{:x}", self.payload_length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct MaxPayloadLength(pub u8);
impl MaxPayloadLength {
    #[inline] pub fn payload_max_length(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PAYLOAD_MAX_LENGTH != 0"]
    #[inline] pub fn test_payload_max_length(&self) -> bool {
        self.payload_max_length() != 0
    }

    #[doc="Sets the PAYLOAD_MAX_LENGTH field."]
    #[inline] pub fn set_payload_max_length<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for MaxPayloadLength {
    #[inline]
    fn from(other: u8) -> Self {
         MaxPayloadLength(other)
    }
}

impl ::core::fmt::Display for MaxPayloadLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for MaxPayloadLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.payload_max_length() != 0 { try!(write!(f, " payload_max_length=0x{:x}", self.payload_max_length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HopPeriod(pub u8);
impl HopPeriod {
    #[inline] pub fn freq_hopping_period(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FREQ_HOPPING_PERIOD != 0"]
    #[inline] pub fn test_freq_hopping_period(&self) -> bool {
        self.freq_hopping_period() != 0
    }

    #[doc="Sets the FREQ_HOPPING_PERIOD field."]
    #[inline] pub fn set_freq_hopping_period<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for HopPeriod {
    #[inline]
    fn from(other: u8) -> Self {
         HopPeriod(other)
    }
}

impl ::core::fmt::Display for HopPeriod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HopPeriod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freq_hopping_period() != 0 { try!(write!(f, " freq_hopping_period=0x{:x}", self.freq_hopping_period()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FifoRxByteAddr(pub u8);
impl FifoRxByteAddr {
    #[inline] pub fn fifo_rx_byte_addr_ptr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFO_RX_BYTE_ADDR_PTR != 0"]
    #[inline] pub fn test_fifo_rx_byte_addr_ptr(&self) -> bool {
        self.fifo_rx_byte_addr_ptr() != 0
    }

    #[doc="Sets the FIFO_RX_BYTE_ADDR_PTR field."]
    #[inline] pub fn set_fifo_rx_byte_addr_ptr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FifoRxByteAddr {
    #[inline]
    fn from(other: u8) -> Self {
         FifoRxByteAddr(other)
    }
}

impl ::core::fmt::Display for FifoRxByteAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FifoRxByteAddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_rx_byte_addr_ptr() != 0 { try!(write!(f, " fifo_rx_byte_addr_ptr=0x{:x}", self.fifo_rx_byte_addr_ptr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModemConfig3(pub u8);
impl ModemConfig3 {
    #[inline] pub fn mobile_node(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MOBILE_NODE != 0"]
    #[inline] pub fn test_mobile_node(&self) -> bool {
        self.mobile_node() != 0
    }

    #[doc="Sets the MOBILE_NODE field."]
    #[inline] pub fn set_mobile_node<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn agc_auto_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AGC_AUTO_ON != 0"]
    #[inline] pub fn test_agc_auto_on(&self) -> bool {
        self.agc_auto_on() != 0
    }

    #[doc="Sets the AGC_AUTO_ON field."]
    #[inline] pub fn set_agc_auto_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for ModemConfig3 {
    #[inline]
    fn from(other: u8) -> Self {
         ModemConfig3(other)
    }
}

impl ::core::fmt::Display for ModemConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ModemConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mobile_node() != 0 { try!(write!(f, " mobile_node"))}
        if self.agc_auto_on() != 0 { try!(write!(f, " agc_auto_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PpmCorrection(pub u8);
impl PpmCorrection {
    #[inline] pub fn ppm_correction(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PPM_CORRECTION != 0"]
    #[inline] pub fn test_ppm_correction(&self) -> bool {
        self.ppm_correction() != 0
    }

    #[doc="Sets the PPM_CORRECTION field."]
    #[inline] pub fn set_ppm_correction<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PpmCorrection {
    #[inline]
    fn from(other: u8) -> Self {
         PpmCorrection(other)
    }
}

impl ::core::fmt::Display for PpmCorrection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PpmCorrection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ppm_correction() != 0 { try!(write!(f, " ppm_correction=0x{:x}", self.ppm_correction()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FeiMsb(pub u8);
impl FeiMsb {
    #[inline] pub fn freq_error(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if FREQ_ERROR != 0"]
    #[inline] pub fn test_freq_error(&self) -> bool {
        self.freq_error() != 0
    }

    #[doc="Sets the FREQ_ERROR field."]
    #[inline] pub fn set_freq_error<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FeiMsb {
    #[inline]
    fn from(other: u8) -> Self {
         FeiMsb(other)
    }
}

impl ::core::fmt::Display for FeiMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FeiMsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freq_error() != 0 { try!(write!(f, " freq_error=0x{:x}", self.freq_error()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FeiMid(pub u8);
impl FeiMid {
    #[inline] pub fn freq_error(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FREQ_ERROR != 0"]
    #[inline] pub fn test_freq_error(&self) -> bool {
        self.freq_error() != 0
    }

    #[doc="Sets the FREQ_ERROR field."]
    #[inline] pub fn set_freq_error<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FeiMid {
    #[inline]
    fn from(other: u8) -> Self {
         FeiMid(other)
    }
}

impl ::core::fmt::Display for FeiMid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FeiMid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freq_error() != 0 { try!(write!(f, " freq_error=0x{:x}", self.freq_error()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FeiLsb(pub u8);
impl FeiLsb {
    #[inline] pub fn freq_error(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FREQ_ERROR != 0"]
    #[inline] pub fn test_freq_error(&self) -> bool {
        self.freq_error() != 0
    }

    #[doc="Sets the FREQ_ERROR field."]
    #[inline] pub fn set_freq_error<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FeiLsb {
    #[inline]
    fn from(other: u8) -> Self {
         FeiLsb(other)
    }
}

impl ::core::fmt::Display for FeiLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FeiLsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freq_error() != 0 { try!(write!(f, " freq_error=0x{:x}", self.freq_error()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RssiWideband(pub u8);
impl RssiWideband {
    #[inline] pub fn rssi_wideband(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSSI_WIDEBAND != 0"]
    #[inline] pub fn test_rssi_wideband(&self) -> bool {
        self.rssi_wideband() != 0
    }

    #[doc="Sets the RSSI_WIDEBAND field."]
    #[inline] pub fn set_rssi_wideband<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for RssiWideband {
    #[inline]
    fn from(other: u8) -> Self {
         RssiWideband(other)
    }
}

impl ::core::fmt::Display for RssiWideband {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for RssiWideband {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rssi_wideband() != 0 { try!(write!(f, " rssi_wideband=0x{:x}", self.rssi_wideband()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DetectOptimize(pub u8);
impl DetectOptimize {
    #[inline] pub fn detection_optimize(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if DETECTION_OPTIMIZE != 0"]
    #[inline] pub fn test_detection_optimize(&self) -> bool {
        self.detection_optimize() != 0
    }

    #[doc="Sets the DETECTION_OPTIMIZE field."]
    #[inline] pub fn set_detection_optimize<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for DetectOptimize {
    #[inline]
    fn from(other: u8) -> Self {
         DetectOptimize(other)
    }
}

impl ::core::fmt::Display for DetectOptimize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DetectOptimize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.detection_optimize() != 0 { try!(write!(f, " detection_optimize=0x{:x}", self.detection_optimize()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct InvertIq(pub u8);
impl InvertIq {
    #[inline] pub fn invert_iq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INVERT_IQ != 0"]
    #[inline] pub fn test_invert_iq(&self) -> bool {
        self.invert_iq() != 0
    }

    #[doc="Sets the INVERT_IQ field."]
    #[inline] pub fn set_invert_iq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for InvertIq {
    #[inline]
    fn from(other: u8) -> Self {
         InvertIq(other)
    }
}

impl ::core::fmt::Display for InvertIq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for InvertIq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.invert_iq() != 0 { try!(write!(f, " invert_iq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DetectionThreshold(pub u8);
impl DetectionThreshold {
    #[inline] pub fn detection_threshold(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DETECTION_THRESHOLD != 0"]
    #[inline] pub fn test_detection_threshold(&self) -> bool {
        self.detection_threshold() != 0
    }

    #[doc="Sets the DETECTION_THRESHOLD field."]
    #[inline] pub fn set_detection_threshold<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for DetectionThreshold {
    #[inline]
    fn from(other: u8) -> Self {
         DetectionThreshold(other)
    }
}

impl ::core::fmt::Display for DetectionThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DetectionThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.detection_threshold() != 0 { try!(write!(f, " detection_threshold=0x{:x}", self.detection_threshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SyncWord(pub u8);
impl SyncWord {
    #[inline] pub fn sync_word(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SYNC_WORD != 0"]
    #[inline] pub fn test_sync_word(&self) -> bool {
        self.sync_word() != 0
    }

    #[doc="Sets the SYNC_WORD field."]
    #[inline] pub fn set_sync_word<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for SyncWord {
    #[inline]
    fn from(other: u8) -> Self {
         SyncWord(other)
    }
}

impl ::core::fmt::Display for SyncWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for SyncWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_word() != 0 { try!(write!(f, " sync_word=0x{:x}", self.sync_word()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DioMapping1(pub u8);
impl DioMapping1 {
    #[inline] pub fn dio0_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DIO0_MAPPING != 0"]
    #[inline] pub fn test_dio0_mapping(&self) -> bool {
        self.dio0_mapping() != 0
    }

    #[doc="Sets the DIO0_MAPPING field."]
    #[inline] pub fn set_dio0_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn dio1_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DIO1_MAPPING != 0"]
    #[inline] pub fn test_dio1_mapping(&self) -> bool {
        self.dio1_mapping() != 0
    }

    #[doc="Sets the DIO1_MAPPING field."]
    #[inline] pub fn set_dio1_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn dio2_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if DIO2_MAPPING != 0"]
    #[inline] pub fn test_dio2_mapping(&self) -> bool {
        self.dio2_mapping() != 0
    }

    #[doc="Sets the DIO2_MAPPING field."]
    #[inline] pub fn set_dio2_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn dio3_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if DIO3_MAPPING != 0"]
    #[inline] pub fn test_dio3_mapping(&self) -> bool {
        self.dio3_mapping() != 0
    }

    #[doc="Sets the DIO3_MAPPING field."]
    #[inline] pub fn set_dio3_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for DioMapping1 {
    #[inline]
    fn from(other: u8) -> Self {
         DioMapping1(other)
    }
}

impl ::core::fmt::Display for DioMapping1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DioMapping1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dio0_mapping() != 0 { try!(write!(f, " dio0_mapping=0x{:x}", self.dio0_mapping()))}
        if self.dio1_mapping() != 0 { try!(write!(f, " dio1_mapping=0x{:x}", self.dio1_mapping()))}
        if self.dio2_mapping() != 0 { try!(write!(f, " dio2_mapping=0x{:x}", self.dio2_mapping()))}
        if self.dio3_mapping() != 0 { try!(write!(f, " dio3_mapping=0x{:x}", self.dio3_mapping()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DioMapping2(pub u8);
impl DioMapping2 {
    #[inline] pub fn dio4_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DIO4_MAPPING != 0"]
    #[inline] pub fn test_dio4_mapping(&self) -> bool {
        self.dio4_mapping() != 0
    }

    #[doc="Sets the DIO4_MAPPING field."]
    #[inline] pub fn set_dio4_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn dio5_mapping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if DIO5_MAPPING != 0"]
    #[inline] pub fn test_dio5_mapping(&self) -> bool {
        self.dio5_mapping() != 0
    }

    #[doc="Sets the DIO5_MAPPING field."]
    #[inline] pub fn set_dio5_mapping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn map_preamble_detect(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MAP_PREAMBLE_DETECT != 0"]
    #[inline] pub fn test_map_preamble_detect(&self) -> bool {
        self.map_preamble_detect() != 0
    }

    #[doc="Sets the MAP_PREAMBLE_DETECT field."]
    #[inline] pub fn set_map_preamble_detect<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for DioMapping2 {
    #[inline]
    fn from(other: u8) -> Self {
         DioMapping2(other)
    }
}

impl ::core::fmt::Display for DioMapping2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DioMapping2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dio4_mapping() != 0 { try!(write!(f, " dio4_mapping=0x{:x}", self.dio4_mapping()))}
        if self.dio5_mapping() != 0 { try!(write!(f, " dio5_mapping=0x{:x}", self.dio5_mapping()))}
        if self.map_preamble_detect() != 0 { try!(write!(f, " map_preamble_detect"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Version(pub u8);
impl Version {
    #[inline] pub fn version(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VERSION != 0"]
    #[inline] pub fn test_version(&self) -> bool {
        self.version() != 0
    }

    #[doc="Sets the VERSION field."]
    #[inline] pub fn set_version<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Version {
    #[inline]
    fn from(other: u8) -> Self {
         Version(other)
    }
}

impl ::core::fmt::Display for Version {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Version {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.version() != 0 { try!(write!(f, " version=0x{:x}", self.version()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PllHop(pub u8);
impl PllHop {
    #[inline] pub fn fast_hop_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FAST_HOP_ON != 0"]
    #[inline] pub fn test_fast_hop_on(&self) -> bool {
        self.fast_hop_on() != 0
    }

    #[doc="Sets the FAST_HOP_ON field."]
    #[inline] pub fn set_fast_hop_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for PllHop {
    #[inline]
    fn from(other: u8) -> Self {
         PllHop(other)
    }
}

impl ::core::fmt::Display for PllHop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PllHop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fast_hop_on() != 0 { try!(write!(f, " fast_hop_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcxo(pub u8);
impl Tcxo {
    #[inline] pub fn tcxo_input_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCXO_INPUT_ON != 0"]
    #[inline] pub fn test_tcxo_input_on(&self) -> bool {
        self.tcxo_input_on() != 0
    }

    #[doc="Sets the TCXO_INPUT_ON field."]
    #[inline] pub fn set_tcxo_input_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Tcxo {
    #[inline]
    fn from(other: u8) -> Self {
         Tcxo(other)
    }
}

impl ::core::fmt::Display for Tcxo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcxo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcxo_input_on() != 0 { try!(write!(f, " tcxo_input_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PaDac(pub u8);
impl PaDac {
    #[inline] pub fn pa_dac(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PA_DAC != 0"]
    #[inline] pub fn test_pa_dac(&self) -> bool {
        self.pa_dac() != 0
    }

    #[doc="Sets the PA_DAC field."]
    #[inline] pub fn set_pa_dac<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for PaDac {
    #[inline]
    fn from(other: u8) -> Self {
         PaDac(other)
    }
}

impl ::core::fmt::Display for PaDac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PaDac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa_dac() != 0 { try!(write!(f, " pa_dac=0x{:x}", self.pa_dac()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FormerTemp(pub u8);
impl FormerTemp {
    #[inline] pub fn former_temp(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FORMER_TEMP != 0"]
    #[inline] pub fn test_former_temp(&self) -> bool {
        self.former_temp() != 0
    }

    #[doc="Sets the FORMER_TEMP field."]
    #[inline] pub fn set_former_temp<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for FormerTemp {
    #[inline]
    fn from(other: u8) -> Self {
         FormerTemp(other)
    }
}

impl ::core::fmt::Display for FormerTemp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for FormerTemp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.former_temp() != 0 { try!(write!(f, " former_temp=0x{:x}", self.former_temp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AgcRef(pub u8);
impl AgcRef {
    #[inline] pub fn agc_reference_level(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if AGC_REFERENCE_LEVEL != 0"]
    #[inline] pub fn test_agc_reference_level(&self) -> bool {
        self.agc_reference_level() != 0
    }

    #[doc="Sets the AGC_REFERENCE_LEVEL field."]
    #[inline] pub fn set_agc_reference_level<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for AgcRef {
    #[inline]
    fn from(other: u8) -> Self {
         AgcRef(other)
    }
}

impl ::core::fmt::Display for AgcRef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for AgcRef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.agc_reference_level() != 0 { try!(write!(f, " agc_reference_level=0x{:x}", self.agc_reference_level()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AgcThresh1(pub u8);
impl AgcThresh1 {
    #[inline] pub fn agc_step1(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if AGC_STEP1 != 0"]
    #[inline] pub fn test_agc_step1(&self) -> bool {
        self.agc_step1() != 0
    }

    #[doc="Sets the AGC_STEP1 field."]
    #[inline] pub fn set_agc_step1<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for AgcThresh1 {
    #[inline]
    fn from(other: u8) -> Self {
         AgcThresh1(other)
    }
}

impl ::core::fmt::Display for AgcThresh1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for AgcThresh1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.agc_step1() != 0 { try!(write!(f, " agc_step1=0x{:x}", self.agc_step1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AgcThresh2(pub u8);
impl AgcThresh2 {
    #[inline] pub fn agc_step2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if AGC_STEP2 != 0"]
    #[inline] pub fn test_agc_step2(&self) -> bool {
        self.agc_step2() != 0
    }

    #[doc="Sets the AGC_STEP2 field."]
    #[inline] pub fn set_agc_step2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn agc_step3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if AGC_STEP3 != 0"]
    #[inline] pub fn test_agc_step3(&self) -> bool {
        self.agc_step3() != 0
    }

    #[doc="Sets the AGC_STEP3 field."]
    #[inline] pub fn set_agc_step3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for AgcThresh2 {
    #[inline]
    fn from(other: u8) -> Self {
         AgcThresh2(other)
    }
}

impl ::core::fmt::Display for AgcThresh2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for AgcThresh2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.agc_step2() != 0 { try!(write!(f, " agc_step2=0x{:x}", self.agc_step2()))}
        if self.agc_step3() != 0 { try!(write!(f, " agc_step3=0x{:x}", self.agc_step3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AgcThresh3(pub u8);
impl AgcThresh3 {
    #[inline] pub fn agc_step4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if AGC_STEP4 != 0"]
    #[inline] pub fn test_agc_step4(&self) -> bool {
        self.agc_step4() != 0
    }

    #[doc="Sets the AGC_STEP4 field."]
    #[inline] pub fn set_agc_step4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn agc_step5(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if AGC_STEP5 != 0"]
    #[inline] pub fn test_agc_step5(&self) -> bool {
        self.agc_step5() != 0
    }

    #[doc="Sets the AGC_STEP5 field."]
    #[inline] pub fn set_agc_step5<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for AgcThresh3 {
    #[inline]
    fn from(other: u8) -> Self {
         AgcThresh3(other)
    }
}

impl ::core::fmt::Display for AgcThresh3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for AgcThresh3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.agc_step4() != 0 { try!(write!(f, " agc_step4=0x{:x}", self.agc_step4()))}
        if self.agc_step5() != 0 { try!(write!(f, " agc_step5=0x{:x}", self.agc_step5()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PllHf(pub u8);
impl PllHf {
    #[inline] pub fn pll_bandwidth(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PLL_BANDWIDTH != 0"]
    #[inline] pub fn test_pll_bandwidth(&self) -> bool {
        self.pll_bandwidth() != 0
    }

    #[doc="Sets the PLL_BANDWIDTH field."]
    #[inline] pub fn set_pll_bandwidth<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for PllHf {
    #[inline]
    fn from(other: u8) -> Self {
         PllHf(other)
    }
}

impl ::core::fmt::Display for PllHf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PllHf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pll_bandwidth() != 0 { try!(write!(f, " pll_bandwidth=0x{:x}", self.pll_bandwidth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


}

