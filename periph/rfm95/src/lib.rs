#![no_std]

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

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Fifo(u8);

    impl From<u8> for Fifo {
        fn from(other: u8) -> Self { Fifo(other) }
    }

    impl From<Fifo> for u8 {
        fn from(other: Fifo) -> Self { other.0 }
    }

    impl Fifo {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Fifo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Fifo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo() != 0 { write!(f, " fifo=0x{:x}", self.fifo())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Opmode(u8);

    impl From<u8> for Opmode {
        fn from(other: u8) -> Self { Opmode(other) }
    }

    impl From<Opmode> for u8 {
        fn from(other: Opmode) -> Self { other.0 }
    }

    impl Opmode {
        pub fn value(&self) -> u8 { self.0 }

        pub fn long_range_mode(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_long_range_mode(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn access_shared_reg(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_access_shared_reg(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn low_frequency_mode_on(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_low_frequency_mode_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn mode(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_mode(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Opmode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Opmode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.long_range_mode() != 0 { write!(f, " long_range_mode")? }
            if self.access_shared_reg() != 0 { write!(f, " access_shared_reg")? }
            if self.low_frequency_mode_on() != 0 { write!(f, " low_frequency_mode_on")? }
            if self.mode() != 0 { write!(f, " mode=0x{:x}", self.mode())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FrfMsb(u8);

    impl From<u8> for FrfMsb {
        fn from(other: u8) -> Self { FrfMsb(other) }
    }

    impl From<FrfMsb> for u8 {
        fn from(other: FrfMsb) -> Self { other.0 }
    }

    impl FrfMsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn frf(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_frf(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FrfMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FrfMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FrfMid(u8);

    impl From<u8> for FrfMid {
        fn from(other: u8) -> Self { FrfMid(other) }
    }

    impl From<FrfMid> for u8 {
        fn from(other: FrfMid) -> Self { other.0 }
    }

    impl FrfMid {
        pub fn value(&self) -> u8 { self.0 }

        pub fn frf(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_frf(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FrfMid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FrfMid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FrfLsb(u8);

    impl From<u8> for FrfLsb {
        fn from(other: u8) -> Self { FrfLsb(other) }
    }

    impl From<FrfLsb> for u8 {
        fn from(other: FrfLsb) -> Self { other.0 }
    }

    impl FrfLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn frf(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_frf(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FrfLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FrfLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PaConfig(u8);

    impl From<u8> for PaConfig {
        fn from(other: u8) -> Self { PaConfig(other) }
    }

    impl From<PaConfig> for u8 {
        fn from(other: PaConfig) -> Self { other.0 }
    }

    impl PaConfig {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa_select(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_pa_select(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn max_power(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x7 // [6:4]
        }
    
        pub fn set_max_power(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn output_power(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_output_power(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PaConfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PaConfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa_select() != 0 { write!(f, " pa_select")? }
            if self.max_power() != 0 { write!(f, " max_power=0x{:x}", self.max_power())? }
            if self.output_power() != 0 { write!(f, " output_power=0x{:x}", self.output_power())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PaRamp(u8);

    impl From<u8> for PaRamp {
        fn from(other: u8) -> Self { PaRamp(other) }
    }

    impl From<PaRamp> for u8 {
        fn from(other: PaRamp) -> Self { other.0 }
    }

    impl PaRamp {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa_ramp(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_pa_ramp(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PaRamp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PaRamp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa_ramp() != 0 { write!(f, " pa_ramp=0x{:x}", self.pa_ramp())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Ocp(u8);

    impl From<u8> for Ocp {
        fn from(other: u8) -> Self { Ocp(other) }
    }

    impl From<Ocp> for u8 {
        fn from(other: Ocp) -> Self { other.0 }
    }

    impl Ocp {
        pub fn value(&self) -> u8 { self.0 }

        pub fn ocp_on(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_ocp_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn ocp_trim(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1f // [4:0]
        }
    
        pub fn set_ocp_trim(mut self, value: u8) -> Self {
            assert!((value & !0x1f) == 0);
            self.0 &= !(0x1f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Ocp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Ocp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.ocp_on() != 0 { write!(f, " ocp_on")? }
            if self.ocp_trim() != 0 { write!(f, " ocp_trim=0x{:x}", self.ocp_trim())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Lna(u8);

    impl From<u8> for Lna {
        fn from(other: u8) -> Self { Lna(other) }
    }

    impl From<Lna> for u8 {
        fn from(other: Lna) -> Self { other.0 }
    }

    impl Lna {
        pub fn value(&self) -> u8 { self.0 }

        pub fn lna_gain(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x7 // [7:5]
        }
    
        pub fn set_lna_gain(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn lna_boost_lf(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x3 // [4:3]
        }
    
        pub fn set_lna_boost_lf(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn lna_boost_hf(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_lna_boost_hf(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Lna {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Lna {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.lna_gain() != 0 { write!(f, " lna_gain=0x{:x}", self.lna_gain())? }
            if self.lna_boost_lf() != 0 { write!(f, " lna_boost_lf=0x{:x}", self.lna_boost_lf())? }
            if self.lna_boost_hf() != 0 { write!(f, " lna_boost_hf=0x{:x}", self.lna_boost_hf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FifoAddrPtr(u8);

    impl From<u8> for FifoAddrPtr {
        fn from(other: u8) -> Self { FifoAddrPtr(other) }
    }

    impl From<FifoAddrPtr> for u8 {
        fn from(other: FifoAddrPtr) -> Self { other.0 }
    }

    impl FifoAddrPtr {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_addr_ptr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_addr_ptr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FifoAddrPtr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FifoAddrPtr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_addr_ptr() != 0 { write!(f, " fifo_addr_ptr=0x{:x}", self.fifo_addr_ptr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FifoTxBaseAddr(u8);

    impl From<u8> for FifoTxBaseAddr {
        fn from(other: u8) -> Self { FifoTxBaseAddr(other) }
    }

    impl From<FifoTxBaseAddr> for u8 {
        fn from(other: FifoTxBaseAddr) -> Self { other.0 }
    }

    impl FifoTxBaseAddr {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_tx_base_addr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_tx_base_addr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FifoTxBaseAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FifoTxBaseAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_tx_base_addr() != 0 { write!(f, " fifo_tx_base_addr=0x{:x}", self.fifo_tx_base_addr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FifoRxBaseAddr(u8);

    impl From<u8> for FifoRxBaseAddr {
        fn from(other: u8) -> Self { FifoRxBaseAddr(other) }
    }

    impl From<FifoRxBaseAddr> for u8 {
        fn from(other: FifoRxBaseAddr) -> Self { other.0 }
    }

    impl FifoRxBaseAddr {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_rx_base_addr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_rx_base_addr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FifoRxBaseAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FifoRxBaseAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_rx_base_addr() != 0 { write!(f, " fifo_rx_base_addr=0x{:x}", self.fifo_rx_base_addr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FifoRxCurrentAddr(u8);

    impl From<u8> for FifoRxCurrentAddr {
        fn from(other: u8) -> Self { FifoRxCurrentAddr(other) }
    }

    impl From<FifoRxCurrentAddr> for u8 {
        fn from(other: FifoRxCurrentAddr) -> Self { other.0 }
    }

    impl FifoRxCurrentAddr {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_rx_current_addr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_rx_current_addr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FifoRxCurrentAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FifoRxCurrentAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_rx_current_addr() != 0 { write!(f, " fifo_rx_current_addr=0x{:x}", self.fifo_rx_current_addr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct IrqFlagsMask(u8);

    impl From<u8> for IrqFlagsMask {
        fn from(other: u8) -> Self { IrqFlagsMask(other) }
    }

    impl From<IrqFlagsMask> for u8 {
        fn from(other: IrqFlagsMask) -> Self { other.0 }
    }

    impl IrqFlagsMask {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rx_timeout_mask(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_rx_timeout_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn rx_done_mask(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_rx_done_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn payload_crc_error_mask(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_payload_crc_error_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn valid_header_mask(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_valid_header_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn tx_done_mask(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_tx_done_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn cad_done_mask(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_cad_done_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn fhss_change_channel_mask(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_fhss_change_channel_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn cad_detected_mask(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_cad_detected_mask(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for IrqFlagsMask {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for IrqFlagsMask {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rx_timeout_mask() != 0 { write!(f, " rx_timeout_mask")? }
            if self.rx_done_mask() != 0 { write!(f, " rx_done_mask")? }
            if self.payload_crc_error_mask() != 0 { write!(f, " payload_crc_error_mask")? }
            if self.valid_header_mask() != 0 { write!(f, " valid_header_mask")? }
            if self.tx_done_mask() != 0 { write!(f, " tx_done_mask")? }
            if self.cad_done_mask() != 0 { write!(f, " cad_done_mask")? }
            if self.fhss_change_channel_mask() != 0 { write!(f, " fhss_change_channel_mask")? }
            if self.cad_detected_mask() != 0 { write!(f, " cad_detected_mask")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct IrqFlags(u8);

    impl From<u8> for IrqFlags {
        fn from(other: u8) -> Self { IrqFlags(other) }
    }

    impl From<IrqFlags> for u8 {
        fn from(other: IrqFlags) -> Self { other.0 }
    }

    impl IrqFlags {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rx_timeout(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_rx_timeout(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn rx_done(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_rx_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn payload_crc_error(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_payload_crc_error(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn valid_header(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_valid_header(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn tx_done(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_tx_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn cad_done(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_cad_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn fhss_change_channel(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_fhss_change_channel(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn cad_detected(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_cad_detected(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for IrqFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for IrqFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rx_timeout() != 0 { write!(f, " rx_timeout")? }
            if self.rx_done() != 0 { write!(f, " rx_done")? }
            if self.payload_crc_error() != 0 { write!(f, " payload_crc_error")? }
            if self.valid_header() != 0 { write!(f, " valid_header")? }
            if self.tx_done() != 0 { write!(f, " tx_done")? }
            if self.cad_done() != 0 { write!(f, " cad_done")? }
            if self.fhss_change_channel() != 0 { write!(f, " fhss_change_channel")? }
            if self.cad_detected() != 0 { write!(f, " cad_detected")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RxNbBytes(u8);

    impl From<u8> for RxNbBytes {
        fn from(other: u8) -> Self { RxNbBytes(other) }
    }

    impl From<RxNbBytes> for u8 {
        fn from(other: RxNbBytes) -> Self { other.0 }
    }

    impl RxNbBytes {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_rx_bytes_nb(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_rx_bytes_nb(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RxNbBytes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RxNbBytes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_rx_bytes_nb() != 0 { write!(f, " fifo_rx_bytes_nb=0x{:x}", self.fifo_rx_bytes_nb())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RxHeaderCntValueMsb(u8);

    impl From<u8> for RxHeaderCntValueMsb {
        fn from(other: u8) -> Self { RxHeaderCntValueMsb(other) }
    }

    impl From<RxHeaderCntValueMsb> for u8 {
        fn from(other: RxHeaderCntValueMsb) -> Self { other.0 }
    }

    impl RxHeaderCntValueMsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn valid_header_cnt_msb(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_valid_header_cnt_msb(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RxHeaderCntValueMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RxHeaderCntValueMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.valid_header_cnt_msb() != 0 { write!(f, " valid_header_cnt_msb=0x{:x}", self.valid_header_cnt_msb())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RxHeaderCntValueLsb(u8);

    impl From<u8> for RxHeaderCntValueLsb {
        fn from(other: u8) -> Self { RxHeaderCntValueLsb(other) }
    }

    impl From<RxHeaderCntValueLsb> for u8 {
        fn from(other: RxHeaderCntValueLsb) -> Self { other.0 }
    }

    impl RxHeaderCntValueLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn valid_header_cnt_lsb(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_valid_header_cnt_lsb(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RxHeaderCntValueLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RxHeaderCntValueLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.valid_header_cnt_lsb() != 0 { write!(f, " valid_header_cnt_lsb=0x{:x}", self.valid_header_cnt_lsb())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RxPacketCntValueMsb(u8);

    impl From<u8> for RxPacketCntValueMsb {
        fn from(other: u8) -> Self { RxPacketCntValueMsb(other) }
    }

    impl From<RxPacketCntValueMsb> for u8 {
        fn from(other: RxPacketCntValueMsb) -> Self { other.0 }
    }

    impl RxPacketCntValueMsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn valid_packet_cnt_msb(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_valid_packet_cnt_msb(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RxPacketCntValueMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RxPacketCntValueMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.valid_packet_cnt_msb() != 0 { write!(f, " valid_packet_cnt_msb=0x{:x}", self.valid_packet_cnt_msb())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RxPacketCntValueLsb(u8);

    impl From<u8> for RxPacketCntValueLsb {
        fn from(other: u8) -> Self { RxPacketCntValueLsb(other) }
    }

    impl From<RxPacketCntValueLsb> for u8 {
        fn from(other: RxPacketCntValueLsb) -> Self { other.0 }
    }

    impl RxPacketCntValueLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn valid_packet_cnt_lsb(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_valid_packet_cnt_lsb(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RxPacketCntValueLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RxPacketCntValueLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.valid_packet_cnt_lsb() != 0 { write!(f, " valid_packet_cnt_lsb=0x{:x}", self.valid_packet_cnt_lsb())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct ModemStat(u8);

    impl From<u8> for ModemStat {
        fn from(other: u8) -> Self { ModemStat(other) }
    }

    impl From<ModemStat> for u8 {
        fn from(other: ModemStat) -> Self { other.0 }
    }

    impl ModemStat {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rx_coding_rate(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x7 // [7:5]
        }
    
        pub fn set_rx_coding_rate(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn modem_status(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1f // [4:0]
        }
    
        pub fn set_modem_status(mut self, value: u8) -> Self {
            assert!((value & !0x1f) == 0);
            self.0 &= !(0x1f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for ModemStat {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for ModemStat {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rx_coding_rate() != 0 { write!(f, " rx_coding_rate=0x{:x}", self.rx_coding_rate())? }
            if self.modem_status() != 0 { write!(f, " modem_status=0x{:x}", self.modem_status())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PktSnrValue(u8);

    impl From<u8> for PktSnrValue {
        fn from(other: u8) -> Self { PktSnrValue(other) }
    }

    impl From<PktSnrValue> for u8 {
        fn from(other: PktSnrValue) -> Self { other.0 }
    }

    impl PktSnrValue {
        pub fn value(&self) -> u8 { self.0 }

        pub fn packet_snr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_packet_snr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PktSnrValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PktSnrValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.packet_snr() != 0 { write!(f, " packet_snr=0x{:x}", self.packet_snr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PktRssiValue(u8);

    impl From<u8> for PktRssiValue {
        fn from(other: u8) -> Self { PktRssiValue(other) }
    }

    impl From<PktRssiValue> for u8 {
        fn from(other: PktRssiValue) -> Self { other.0 }
    }

    impl PktRssiValue {
        pub fn value(&self) -> u8 { self.0 }

        pub fn packet_rssi(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_packet_rssi(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PktRssiValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PktRssiValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.packet_rssi() != 0 { write!(f, " packet_rssi=0x{:x}", self.packet_rssi())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RssiValue(u8);

    impl From<u8> for RssiValue {
        fn from(other: u8) -> Self { RssiValue(other) }
    }

    impl From<RssiValue> for u8 {
        fn from(other: RssiValue) -> Self { other.0 }
    }

    impl RssiValue {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rssi(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_rssi(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RssiValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RssiValue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rssi() != 0 { write!(f, " rssi=0x{:x}", self.rssi())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct HopChannel(u8);

    impl From<u8> for HopChannel {
        fn from(other: u8) -> Self { HopChannel(other) }
    }

    impl From<HopChannel> for u8 {
        fn from(other: HopChannel) -> Self { other.0 }
    }

    impl HopChannel {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pll_timeout(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_pll_timeout(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn rx_payload_crc_on(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_rx_payload_crc_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn fhss_present_channel(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3f // [5:0]
        }
    
        pub fn set_fhss_present_channel(mut self, value: u8) -> Self {
            assert!((value & !0x3f) == 0);
            self.0 &= !(0x3f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for HopChannel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for HopChannel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pll_timeout() != 0 { write!(f, " pll_timeout")? }
            if self.rx_payload_crc_on() != 0 { write!(f, " rx_payload_crc_on")? }
            if self.fhss_present_channel() != 0 { write!(f, " fhss_present_channel=0x{:x}", self.fhss_present_channel())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct ModemConfig1(u8);

    impl From<u8> for ModemConfig1 {
        fn from(other: u8) -> Self { ModemConfig1(other) }
    }

    impl From<ModemConfig1> for u8 {
        fn from(other: ModemConfig1) -> Self { other.0 }
    }

    impl ModemConfig1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn bw(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0xf // [7:4]
        }
    
        pub fn set_bw(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn coding_rate(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x7 // [3:1]
        }
    
        pub fn set_coding_rate(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn implicit_header_mode_on(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_implicit_header_mode_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for ModemConfig1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for ModemConfig1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.bw() != 0 { write!(f, " bw=0x{:x}", self.bw())? }
            if self.coding_rate() != 0 { write!(f, " coding_rate=0x{:x}", self.coding_rate())? }
            if self.implicit_header_mode_on() != 0 { write!(f, " implicit_header_mode_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct ModemConfig2(u8);

    impl From<u8> for ModemConfig2 {
        fn from(other: u8) -> Self { ModemConfig2(other) }
    }

    impl From<ModemConfig2> for u8 {
        fn from(other: ModemConfig2) -> Self { other.0 }
    }

    impl ModemConfig2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn spreading_factor(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0xf // [7:4]
        }
    
        pub fn set_spreading_factor(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn tx_continuous_mode(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_tx_continuous_mode(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn rx_payload_crc_on(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_rx_payload_crc_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn symb_timeout(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_symb_timeout(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for ModemConfig2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for ModemConfig2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.spreading_factor() != 0 { write!(f, " spreading_factor=0x{:x}", self.spreading_factor())? }
            if self.tx_continuous_mode() != 0 { write!(f, " tx_continuous_mode")? }
            if self.rx_payload_crc_on() != 0 { write!(f, " rx_payload_crc_on")? }
            if self.symb_timeout() != 0 { write!(f, " symb_timeout=0x{:x}", self.symb_timeout())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct SymbTimeoutLsb(u8);

    impl From<u8> for SymbTimeoutLsb {
        fn from(other: u8) -> Self { SymbTimeoutLsb(other) }
    }

    impl From<SymbTimeoutLsb> for u8 {
        fn from(other: SymbTimeoutLsb) -> Self { other.0 }
    }

    impl SymbTimeoutLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn symb_timeout(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_symb_timeout(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for SymbTimeoutLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for SymbTimeoutLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.symb_timeout() != 0 { write!(f, " symb_timeout=0x{:x}", self.symb_timeout())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PreambleMsb(u8);

    impl From<u8> for PreambleMsb {
        fn from(other: u8) -> Self { PreambleMsb(other) }
    }

    impl From<PreambleMsb> for u8 {
        fn from(other: PreambleMsb) -> Self { other.0 }
    }

    impl PreambleMsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn preamble_length(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_preamble_length(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PreambleMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PreambleMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.preamble_length() != 0 { write!(f, " preamble_length=0x{:x}", self.preamble_length())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PreambleLsb(u8);

    impl From<u8> for PreambleLsb {
        fn from(other: u8) -> Self { PreambleLsb(other) }
    }

    impl From<PreambleLsb> for u8 {
        fn from(other: PreambleLsb) -> Self { other.0 }
    }

    impl PreambleLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn preamble_length(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_preamble_length(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PreambleLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PreambleLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.preamble_length() != 0 { write!(f, " preamble_length=0x{:x}", self.preamble_length())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PayloadLength(u8);

    impl From<u8> for PayloadLength {
        fn from(other: u8) -> Self { PayloadLength(other) }
    }

    impl From<PayloadLength> for u8 {
        fn from(other: PayloadLength) -> Self { other.0 }
    }

    impl PayloadLength {
        pub fn value(&self) -> u8 { self.0 }

        pub fn payload_length(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_payload_length(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PayloadLength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PayloadLength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.payload_length() != 0 { write!(f, " payload_length=0x{:x}", self.payload_length())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct MaxPayloadLength(u8);

    impl From<u8> for MaxPayloadLength {
        fn from(other: u8) -> Self { MaxPayloadLength(other) }
    }

    impl From<MaxPayloadLength> for u8 {
        fn from(other: MaxPayloadLength) -> Self { other.0 }
    }

    impl MaxPayloadLength {
        pub fn value(&self) -> u8 { self.0 }

        pub fn payload_max_length(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_payload_max_length(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for MaxPayloadLength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for MaxPayloadLength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.payload_max_length() != 0 { write!(f, " payload_max_length=0x{:x}", self.payload_max_length())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct HopPeriod(u8);

    impl From<u8> for HopPeriod {
        fn from(other: u8) -> Self { HopPeriod(other) }
    }

    impl From<HopPeriod> for u8 {
        fn from(other: HopPeriod) -> Self { other.0 }
    }

    impl HopPeriod {
        pub fn value(&self) -> u8 { self.0 }

        pub fn freq_hopping_period(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_freq_hopping_period(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for HopPeriod {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for HopPeriod {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.freq_hopping_period() != 0 { write!(f, " freq_hopping_period=0x{:x}", self.freq_hopping_period())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FifoRxByteAddr(u8);

    impl From<u8> for FifoRxByteAddr {
        fn from(other: u8) -> Self { FifoRxByteAddr(other) }
    }

    impl From<FifoRxByteAddr> for u8 {
        fn from(other: FifoRxByteAddr) -> Self { other.0 }
    }

    impl FifoRxByteAddr {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_rx_byte_addr_ptr(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fifo_rx_byte_addr_ptr(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FifoRxByteAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FifoRxByteAddr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_rx_byte_addr_ptr() != 0 { write!(f, " fifo_rx_byte_addr_ptr=0x{:x}", self.fifo_rx_byte_addr_ptr())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct ModemConfig3(u8);

    impl From<u8> for ModemConfig3 {
        fn from(other: u8) -> Self { ModemConfig3(other) }
    }

    impl From<ModemConfig3> for u8 {
        fn from(other: ModemConfig3) -> Self { other.0 }
    }

    impl ModemConfig3 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn mobile_node(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_mobile_node(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn agc_auto_on(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_agc_auto_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
    }

    impl ::core::fmt::Display for ModemConfig3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for ModemConfig3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.mobile_node() != 0 { write!(f, " mobile_node")? }
            if self.agc_auto_on() != 0 { write!(f, " agc_auto_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PpmCorrection(u8);

    impl From<u8> for PpmCorrection {
        fn from(other: u8) -> Self { PpmCorrection(other) }
    }

    impl From<PpmCorrection> for u8 {
        fn from(other: PpmCorrection) -> Self { other.0 }
    }

    impl PpmCorrection {
        pub fn value(&self) -> u8 { self.0 }

        pub fn ppm_correction(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_ppm_correction(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PpmCorrection {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PpmCorrection {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.ppm_correction() != 0 { write!(f, " ppm_correction=0x{:x}", self.ppm_correction())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FeiMsb(u8);

    impl From<u8> for FeiMsb {
        fn from(other: u8) -> Self { FeiMsb(other) }
    }

    impl From<FeiMsb> for u8 {
        fn from(other: FeiMsb) -> Self { other.0 }
    }

    impl FeiMsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn freq_error(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_freq_error(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FeiMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FeiMsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.freq_error() != 0 { write!(f, " freq_error=0x{:x}", self.freq_error())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FeiMid(u8);

    impl From<u8> for FeiMid {
        fn from(other: u8) -> Self { FeiMid(other) }
    }

    impl From<FeiMid> for u8 {
        fn from(other: FeiMid) -> Self { other.0 }
    }

    impl FeiMid {
        pub fn value(&self) -> u8 { self.0 }

        pub fn freq_error(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_freq_error(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FeiMid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FeiMid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.freq_error() != 0 { write!(f, " freq_error=0x{:x}", self.freq_error())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FeiLsb(u8);

    impl From<u8> for FeiLsb {
        fn from(other: u8) -> Self { FeiLsb(other) }
    }

    impl From<FeiLsb> for u8 {
        fn from(other: FeiLsb) -> Self { other.0 }
    }

    impl FeiLsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn freq_error(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_freq_error(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FeiLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FeiLsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.freq_error() != 0 { write!(f, " freq_error=0x{:x}", self.freq_error())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct RssiWideband(u8);

    impl From<u8> for RssiWideband {
        fn from(other: u8) -> Self { RssiWideband(other) }
    }

    impl From<RssiWideband> for u8 {
        fn from(other: RssiWideband) -> Self { other.0 }
    }

    impl RssiWideband {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rssi_wideband(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_rssi_wideband(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for RssiWideband {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for RssiWideband {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rssi_wideband() != 0 { write!(f, " rssi_wideband=0x{:x}", self.rssi_wideband())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DetectOptimize(u8);

    impl From<u8> for DetectOptimize {
        fn from(other: u8) -> Self { DetectOptimize(other) }
    }

    impl From<DetectOptimize> for u8 {
        fn from(other: DetectOptimize) -> Self { other.0 }
    }

    impl DetectOptimize {
        pub fn value(&self) -> u8 { self.0 }

        pub fn detection_optimize(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_detection_optimize(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for DetectOptimize {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for DetectOptimize {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.detection_optimize() != 0 { write!(f, " detection_optimize=0x{:x}", self.detection_optimize())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct InvertIq(u8);

    impl From<u8> for InvertIq {
        fn from(other: u8) -> Self { InvertIq(other) }
    }

    impl From<InvertIq> for u8 {
        fn from(other: InvertIq) -> Self { other.0 }
    }

    impl InvertIq {
        pub fn value(&self) -> u8 { self.0 }

        pub fn invert_iq(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_invert_iq(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
    }

    impl ::core::fmt::Display for InvertIq {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for InvertIq {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.invert_iq() != 0 { write!(f, " invert_iq")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DetectionThreshold(u8);

    impl From<u8> for DetectionThreshold {
        fn from(other: u8) -> Self { DetectionThreshold(other) }
    }

    impl From<DetectionThreshold> for u8 {
        fn from(other: DetectionThreshold) -> Self { other.0 }
    }

    impl DetectionThreshold {
        pub fn value(&self) -> u8 { self.0 }

        pub fn detection_threshold(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_detection_threshold(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for DetectionThreshold {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for DetectionThreshold {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.detection_threshold() != 0 { write!(f, " detection_threshold=0x{:x}", self.detection_threshold())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct SyncWord(u8);

    impl From<u8> for SyncWord {
        fn from(other: u8) -> Self { SyncWord(other) }
    }

    impl From<SyncWord> for u8 {
        fn from(other: SyncWord) -> Self { other.0 }
    }

    impl SyncWord {
        pub fn value(&self) -> u8 { self.0 }

        pub fn sync_word(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_sync_word(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for SyncWord {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for SyncWord {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.sync_word() != 0 { write!(f, " sync_word=0x{:x}", self.sync_word())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DioMapping1(u8);

    impl From<u8> for DioMapping1 {
        fn from(other: u8) -> Self { DioMapping1(other) }
    }

    impl From<DioMapping1> for u8 {
        fn from(other: DioMapping1) -> Self { other.0 }
    }

    impl DioMapping1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn dio0_mapping(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_dio0_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn dio1_mapping(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x3 // [5:4]
        }
    
        pub fn set_dio1_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn dio2_mapping(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x3 // [3:2]
        }
    
        pub fn set_dio2_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn dio3_mapping(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_dio3_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for DioMapping1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for DioMapping1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.dio0_mapping() != 0 { write!(f, " dio0_mapping=0x{:x}", self.dio0_mapping())? }
            if self.dio1_mapping() != 0 { write!(f, " dio1_mapping=0x{:x}", self.dio1_mapping())? }
            if self.dio2_mapping() != 0 { write!(f, " dio2_mapping=0x{:x}", self.dio2_mapping())? }
            if self.dio3_mapping() != 0 { write!(f, " dio3_mapping=0x{:x}", self.dio3_mapping())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DioMapping2(u8);

    impl From<u8> for DioMapping2 {
        fn from(other: u8) -> Self { DioMapping2(other) }
    }

    impl From<DioMapping2> for u8 {
        fn from(other: DioMapping2) -> Self { other.0 }
    }

    impl DioMapping2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn dio4_mapping(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_dio4_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn dio5_mapping(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x3 // [5:4]
        }
    
        pub fn set_dio5_mapping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn map_preamble_detect(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_map_preamble_detect(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for DioMapping2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for DioMapping2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.dio4_mapping() != 0 { write!(f, " dio4_mapping=0x{:x}", self.dio4_mapping())? }
            if self.dio5_mapping() != 0 { write!(f, " dio5_mapping=0x{:x}", self.dio5_mapping())? }
            if self.map_preamble_detect() != 0 { write!(f, " map_preamble_detect")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Version(u8);

    impl From<u8> for Version {
        fn from(other: u8) -> Self { Version(other) }
    }

    impl From<Version> for u8 {
        fn from(other: Version) -> Self { other.0 }
    }

    impl Version {
        pub fn value(&self) -> u8 { self.0 }

        pub fn version(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_version(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Version {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Version {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.version() != 0 { write!(f, " version=0x{:x}", self.version())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PllHop(u8);

    impl From<u8> for PllHop {
        fn from(other: u8) -> Self { PllHop(other) }
    }

    impl From<PllHop> for u8 {
        fn from(other: PllHop) -> Self { other.0 }
    }

    impl PllHop {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fast_hop_on(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_fast_hop_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
    }

    impl ::core::fmt::Display for PllHop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PllHop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fast_hop_on() != 0 { write!(f, " fast_hop_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Tcxo(u8);

    impl From<u8> for Tcxo {
        fn from(other: u8) -> Self { Tcxo(other) }
    }

    impl From<Tcxo> for u8 {
        fn from(other: Tcxo) -> Self { other.0 }
    }

    impl Tcxo {
        pub fn value(&self) -> u8 { self.0 }

        pub fn tcxo_input_on(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_tcxo_input_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
    }

    impl ::core::fmt::Display for Tcxo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Tcxo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.tcxo_input_on() != 0 { write!(f, " tcxo_input_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PaDac(u8);

    impl From<u8> for PaDac {
        fn from(other: u8) -> Self { PaDac(other) }
    }

    impl From<PaDac> for u8 {
        fn from(other: PaDac) -> Self { other.0 }
    }

    impl PaDac {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa_dac(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_pa_dac(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for PaDac {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PaDac {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa_dac() != 0 { write!(f, " pa_dac=0x{:x}", self.pa_dac())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct FormerTemp(u8);

    impl From<u8> for FormerTemp {
        fn from(other: u8) -> Self { FormerTemp(other) }
    }

    impl From<FormerTemp> for u8 {
        fn from(other: FormerTemp) -> Self { other.0 }
    }

    impl FormerTemp {
        pub fn value(&self) -> u8 { self.0 }

        pub fn former_temp(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_former_temp(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for FormerTemp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for FormerTemp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.former_temp() != 0 { write!(f, " former_temp=0x{:x}", self.former_temp())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct AgcRef(u8);

    impl From<u8> for AgcRef {
        fn from(other: u8) -> Self { AgcRef(other) }
    }

    impl From<AgcRef> for u8 {
        fn from(other: AgcRef) -> Self { other.0 }
    }

    impl AgcRef {
        pub fn value(&self) -> u8 { self.0 }

        pub fn agc_reference_level(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3f // [5:0]
        }
    
        pub fn set_agc_reference_level(mut self, value: u8) -> Self {
            assert!((value & !0x3f) == 0);
            self.0 &= !(0x3f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for AgcRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for AgcRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.agc_reference_level() != 0 { write!(f, " agc_reference_level=0x{:x}", self.agc_reference_level())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct AgcThresh1(u8);

    impl From<u8> for AgcThresh1 {
        fn from(other: u8) -> Self { AgcThresh1(other) }
    }

    impl From<AgcThresh1> for u8 {
        fn from(other: AgcThresh1) -> Self { other.0 }
    }

    impl AgcThresh1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn agc_step1(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1f // [4:0]
        }
    
        pub fn set_agc_step1(mut self, value: u8) -> Self {
            assert!((value & !0x1f) == 0);
            self.0 &= !(0x1f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for AgcThresh1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for AgcThresh1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.agc_step1() != 0 { write!(f, " agc_step1=0x{:x}", self.agc_step1())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct AgcThresh2(u8);

    impl From<u8> for AgcThresh2 {
        fn from(other: u8) -> Self { AgcThresh2(other) }
    }

    impl From<AgcThresh2> for u8 {
        fn from(other: AgcThresh2) -> Self { other.0 }
    }

    impl AgcThresh2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn agc_step2(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0xf // [7:4]
        }
    
        pub fn set_agc_step2(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn agc_step3(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_agc_step3(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for AgcThresh2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for AgcThresh2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.agc_step2() != 0 { write!(f, " agc_step2=0x{:x}", self.agc_step2())? }
            if self.agc_step3() != 0 { write!(f, " agc_step3=0x{:x}", self.agc_step3())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct AgcThresh3(u8);

    impl From<u8> for AgcThresh3 {
        fn from(other: u8) -> Self { AgcThresh3(other) }
    }

    impl From<AgcThresh3> for u8 {
        fn from(other: AgcThresh3) -> Self { other.0 }
    }

    impl AgcThresh3 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn agc_step4(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0xf // [7:4]
        }
    
        pub fn set_agc_step4(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn agc_step5(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_agc_step5(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for AgcThresh3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for AgcThresh3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.agc_step4() != 0 { write!(f, " agc_step4=0x{:x}", self.agc_step4())? }
            if self.agc_step5() != 0 { write!(f, " agc_step5=0x{:x}", self.agc_step5())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PllHf(u8);

    impl From<u8> for PllHf {
        fn from(other: u8) -> Self { PllHf(other) }
    }

    impl From<PllHf> for u8 {
        fn from(other: PllHf) -> Self { other.0 }
    }

    impl PllHf {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pll_bandwidth(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_pll_bandwidth(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
    }

    impl ::core::fmt::Display for PllHf {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for PllHf {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pll_bandwidth() != 0 { write!(f, " pll_bandwidth=0x{:x}", self.pll_bandwidth())? }
            write!(f, "]")?;
            Ok(())
        }
    }


}

