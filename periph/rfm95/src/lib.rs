#![no_std]

pub type Addr = u8;
pub type Value = u8;

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
    pub fn fifo(&self) -> Fifo {
        Fifo(self.read(REG_FIFO))
    }
    pub fn set_fifo(&self, value: Fifo) {
        self.write(REG_FIFO, value.0)
    }
    pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) {
        let tmp = Fifo(self.read(REG_FIFO));
        self.write(REG_FIFO, f(tmp).0)
    }

    pub fn opmode(&self) -> Opmode {
        Opmode(self.read(REG_OPMODE))
    }
    pub fn set_opmode(&self, value: Opmode) {
        self.write(REG_OPMODE, value.0)
    }
    pub fn with_opmode<F: FnOnce(Opmode) -> Opmode>(&self, f: F) {
        let tmp = Opmode(self.read(REG_OPMODE));
        self.write(REG_OPMODE, f(tmp).0)
    }

    pub fn frf_msb(&self) -> FrfMsb {
        FrfMsb(self.read(REG_FRF_MSB))
    }
    pub fn set_frf_msb(&self, value: FrfMsb) {
        self.write(REG_FRF_MSB, value.0)
    }
    pub fn with_frf_msb<F: FnOnce(FrfMsb) -> FrfMsb>(&self, f: F) {
        let tmp = FrfMsb(self.read(REG_FRF_MSB));
        self.write(REG_FRF_MSB, f(tmp).0)
    }

    pub fn frf_mid(&self) -> FrfMid {
        FrfMid(self.read(REG_FRF_MID))
    }
    pub fn set_frf_mid(&self, value: FrfMid) {
        self.write(REG_FRF_MID, value.0)
    }
    pub fn with_frf_mid<F: FnOnce(FrfMid) -> FrfMid>(&self, f: F) {
        let tmp = FrfMid(self.read(REG_FRF_MID));
        self.write(REG_FRF_MID, f(tmp).0)
    }

    pub fn frf_lsb(&self) -> FrfLsb {
        FrfLsb(self.read(REG_FRF_LSB))
    }
    pub fn set_frf_lsb(&self, value: FrfLsb) {
        self.write(REG_FRF_LSB, value.0)
    }
    pub fn with_frf_lsb<F: FnOnce(FrfLsb) -> FrfLsb>(&self, f: F) {
        let tmp = FrfLsb(self.read(REG_FRF_LSB));
        self.write(REG_FRF_LSB, f(tmp).0)
    }

    pub fn pa_config(&self) -> PaConfig {
        PaConfig(self.read(REG_PA_CONFIG))
    }
    pub fn set_pa_config(&self, value: PaConfig) {
        self.write(REG_PA_CONFIG, value.0)
    }
    pub fn with_pa_config<F: FnOnce(PaConfig) -> PaConfig>(&self, f: F) {
        let tmp = PaConfig(self.read(REG_PA_CONFIG));
        self.write(REG_PA_CONFIG, f(tmp).0)
    }

    pub fn pa_ramp(&self) -> PaRamp {
        PaRamp(self.read(REG_PA_RAMP))
    }
    pub fn set_pa_ramp(&self, value: PaRamp) {
        self.write(REG_PA_RAMP, value.0)
    }
    pub fn with_pa_ramp<F: FnOnce(PaRamp) -> PaRamp>(&self, f: F) {
        let tmp = PaRamp(self.read(REG_PA_RAMP));
        self.write(REG_PA_RAMP, f(tmp).0)
    }

    pub fn ocp(&self) -> Ocp {
        Ocp(self.read(REG_OCP))
    }
    pub fn set_ocp(&self, value: Ocp) {
        self.write(REG_OCP, value.0)
    }
    pub fn with_ocp<F: FnOnce(Ocp) -> Ocp>(&self, f: F) {
        let tmp = Ocp(self.read(REG_OCP));
        self.write(REG_OCP, f(tmp).0)
    }

    pub fn lna(&self) -> Lna {
        Lna(self.read(REG_LNA))
    }
    pub fn set_lna(&self, value: Lna) {
        self.write(REG_LNA, value.0)
    }
    pub fn with_lna<F: FnOnce(Lna) -> Lna>(&self, f: F) {
        let tmp = Lna(self.read(REG_LNA));
        self.write(REG_LNA, f(tmp).0)
    }

    pub fn fifo_addr_ptr(&self) -> FifoAddrPtr {
        FifoAddrPtr(self.read(REG_FIFO_ADDR_PTR))
    }
    pub fn set_fifo_addr_ptr(&self, value: FifoAddrPtr) {
        self.write(REG_FIFO_ADDR_PTR, value.0)
    }
    pub fn with_fifo_addr_ptr<F: FnOnce(FifoAddrPtr) -> FifoAddrPtr>(&self, f: F) {
        let tmp = FifoAddrPtr(self.read(REG_FIFO_ADDR_PTR));
        self.write(REG_FIFO_ADDR_PTR, f(tmp).0)
    }

    pub fn fifo_tx_base_addr(&self) -> FifoTxBaseAddr {
        FifoTxBaseAddr(self.read(REG_FIFO_TX_BASE_ADDR))
    }
    pub fn set_fifo_tx_base_addr(&self, value: FifoTxBaseAddr) {
        self.write(REG_FIFO_TX_BASE_ADDR, value.0)
    }
    pub fn with_fifo_tx_base_addr<F: FnOnce(FifoTxBaseAddr) -> FifoTxBaseAddr>(&self, f: F) {
        let tmp = FifoTxBaseAddr(self.read(REG_FIFO_TX_BASE_ADDR));
        self.write(REG_FIFO_TX_BASE_ADDR, f(tmp).0)
    }

    pub fn fifo_rx_base_addr(&self) -> FifoRxBaseAddr {
        FifoRxBaseAddr(self.read(REG_FIFO_RX_BASE_ADDR))
    }
    pub fn set_fifo_rx_base_addr(&self, value: FifoRxBaseAddr) {
        self.write(REG_FIFO_RX_BASE_ADDR, value.0)
    }
    pub fn with_fifo_rx_base_addr<F: FnOnce(FifoRxBaseAddr) -> FifoRxBaseAddr>(&self, f: F) {
        let tmp = FifoRxBaseAddr(self.read(REG_FIFO_RX_BASE_ADDR));
        self.write(REG_FIFO_RX_BASE_ADDR, f(tmp).0)
    }

    pub fn fifo_rx_current_addr(&self) -> FifoRxCurrentAddr {
        FifoRxCurrentAddr(self.read(REG_FIFO_RX_CURRENT_ADDR))
    }
    pub fn set_fifo_rx_current_addr(&self, value: FifoRxCurrentAddr) {
        self.write(REG_FIFO_RX_CURRENT_ADDR, value.0)
    }
    pub fn with_fifo_rx_current_addr<F: FnOnce(FifoRxCurrentAddr) -> FifoRxCurrentAddr>(&self, f: F) {
        let tmp = FifoRxCurrentAddr(self.read(REG_FIFO_RX_CURRENT_ADDR));
        self.write(REG_FIFO_RX_CURRENT_ADDR, f(tmp).0)
    }

    pub fn irq_flags_mask(&self) -> IrqFlagsMask {
        IrqFlagsMask(self.read(REG_IRQ_FLAGS_MASK))
    }
    pub fn set_irq_flags_mask(&self, value: IrqFlagsMask) {
        self.write(REG_IRQ_FLAGS_MASK, value.0)
    }
    pub fn with_irq_flags_mask<F: FnOnce(IrqFlagsMask) -> IrqFlagsMask>(&self, f: F) {
        let tmp = IrqFlagsMask(self.read(REG_IRQ_FLAGS_MASK));
        self.write(REG_IRQ_FLAGS_MASK, f(tmp).0)
    }

    pub fn irq_flags(&self) -> IrqFlags {
        IrqFlags(self.read(REG_IRQ_FLAGS))
    }
    pub fn set_irq_flags(&self, value: IrqFlags) {
        self.write(REG_IRQ_FLAGS, value.0)
    }
    pub fn with_irq_flags<F: FnOnce(IrqFlags) -> IrqFlags>(&self, f: F) {
        let tmp = IrqFlags(self.read(REG_IRQ_FLAGS));
        self.write(REG_IRQ_FLAGS, f(tmp).0)
    }

    pub fn rx_nb_bytes(&self) -> RxNbBytes {
        RxNbBytes(self.read(REG_RX_NB_BYTES))
    }
    pub fn set_rx_nb_bytes(&self, value: RxNbBytes) {
        self.write(REG_RX_NB_BYTES, value.0)
    }
    pub fn with_rx_nb_bytes<F: FnOnce(RxNbBytes) -> RxNbBytes>(&self, f: F) {
        let tmp = RxNbBytes(self.read(REG_RX_NB_BYTES));
        self.write(REG_RX_NB_BYTES, f(tmp).0)
    }

    pub fn rx_header_cnt_value_msb(&self) -> RxHeaderCntValueMsb {
        RxHeaderCntValueMsb(self.read(REG_RX_HEADER_CNT_VALUE_MSB))
    }
    pub fn set_rx_header_cnt_value_msb(&self, value: RxHeaderCntValueMsb) {
        self.write(REG_RX_HEADER_CNT_VALUE_MSB, value.0)
    }
    pub fn with_rx_header_cnt_value_msb<F: FnOnce(RxHeaderCntValueMsb) -> RxHeaderCntValueMsb>(&self, f: F) {
        let tmp = RxHeaderCntValueMsb(self.read(REG_RX_HEADER_CNT_VALUE_MSB));
        self.write(REG_RX_HEADER_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn rx_header_cnt_value_lsb(&self) -> RxHeaderCntValueLsb {
        RxHeaderCntValueLsb(self.read(REG_RX_HEADER_CNT_VALUE_LSB))
    }
    pub fn set_rx_header_cnt_value_lsb(&self, value: RxHeaderCntValueLsb) {
        self.write(REG_RX_HEADER_CNT_VALUE_LSB, value.0)
    }
    pub fn with_rx_header_cnt_value_lsb<F: FnOnce(RxHeaderCntValueLsb) -> RxHeaderCntValueLsb>(&self, f: F) {
        let tmp = RxHeaderCntValueLsb(self.read(REG_RX_HEADER_CNT_VALUE_LSB));
        self.write(REG_RX_HEADER_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn rx_packet_cnt_value_msb(&self) -> RxPacketCntValueMsb {
        RxPacketCntValueMsb(self.read(REG_RX_PACKET_CNT_VALUE_MSB))
    }
    pub fn set_rx_packet_cnt_value_msb(&self, value: RxPacketCntValueMsb) {
        self.write(REG_RX_PACKET_CNT_VALUE_MSB, value.0)
    }
    pub fn with_rx_packet_cnt_value_msb<F: FnOnce(RxPacketCntValueMsb) -> RxPacketCntValueMsb>(&self, f: F) {
        let tmp = RxPacketCntValueMsb(self.read(REG_RX_PACKET_CNT_VALUE_MSB));
        self.write(REG_RX_PACKET_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn rx_packet_cnt_value_lsb(&self) -> RxPacketCntValueLsb {
        RxPacketCntValueLsb(self.read(REG_RX_PACKET_CNT_VALUE_LSB))
    }
    pub fn set_rx_packet_cnt_value_lsb(&self, value: RxPacketCntValueLsb) {
        self.write(REG_RX_PACKET_CNT_VALUE_LSB, value.0)
    }
    pub fn with_rx_packet_cnt_value_lsb<F: FnOnce(RxPacketCntValueLsb) -> RxPacketCntValueLsb>(&self, f: F) {
        let tmp = RxPacketCntValueLsb(self.read(REG_RX_PACKET_CNT_VALUE_LSB));
        self.write(REG_RX_PACKET_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn modem_stat(&self) -> ModemStat {
        ModemStat(self.read(REG_MODEM_STAT))
    }
    pub fn set_modem_stat(&self, value: ModemStat) {
        self.write(REG_MODEM_STAT, value.0)
    }
    pub fn with_modem_stat<F: FnOnce(ModemStat) -> ModemStat>(&self, f: F) {
        let tmp = ModemStat(self.read(REG_MODEM_STAT));
        self.write(REG_MODEM_STAT, f(tmp).0)
    }

    pub fn pkt_snr_value(&self) -> PktSnrValue {
        PktSnrValue(self.read(REG_PKT_SNR_VALUE))
    }
    pub fn set_pkt_snr_value(&self, value: PktSnrValue) {
        self.write(REG_PKT_SNR_VALUE, value.0)
    }
    pub fn with_pkt_snr_value<F: FnOnce(PktSnrValue) -> PktSnrValue>(&self, f: F) {
        let tmp = PktSnrValue(self.read(REG_PKT_SNR_VALUE));
        self.write(REG_PKT_SNR_VALUE, f(tmp).0)
    }

    pub fn pkt_rssi_value(&self) -> PktRssiValue {
        PktRssiValue(self.read(REG_PKT_RSSI_VALUE))
    }
    pub fn set_pkt_rssi_value(&self, value: PktRssiValue) {
        self.write(REG_PKT_RSSI_VALUE, value.0)
    }
    pub fn with_pkt_rssi_value<F: FnOnce(PktRssiValue) -> PktRssiValue>(&self, f: F) {
        let tmp = PktRssiValue(self.read(REG_PKT_RSSI_VALUE));
        self.write(REG_PKT_RSSI_VALUE, f(tmp).0)
    }

    pub fn rssi_value(&self) -> RssiValue {
        RssiValue(self.read(REG_RSSI_VALUE))
    }
    pub fn set_rssi_value(&self, value: RssiValue) {
        self.write(REG_RSSI_VALUE, value.0)
    }
    pub fn with_rssi_value<F: FnOnce(RssiValue) -> RssiValue>(&self, f: F) {
        let tmp = RssiValue(self.read(REG_RSSI_VALUE));
        self.write(REG_RSSI_VALUE, f(tmp).0)
    }

    pub fn hop_channel(&self) -> HopChannel {
        HopChannel(self.read(REG_HOP_CHANNEL))
    }
    pub fn set_hop_channel(&self, value: HopChannel) {
        self.write(REG_HOP_CHANNEL, value.0)
    }
    pub fn with_hop_channel<F: FnOnce(HopChannel) -> HopChannel>(&self, f: F) {
        let tmp = HopChannel(self.read(REG_HOP_CHANNEL));
        self.write(REG_HOP_CHANNEL, f(tmp).0)
    }

    pub fn modem_config1(&self) -> ModemConfig1 {
        ModemConfig1(self.read(REG_MODEM_CONFIG1))
    }
    pub fn set_modem_config1(&self, value: ModemConfig1) {
        self.write(REG_MODEM_CONFIG1, value.0)
    }
    pub fn with_modem_config1<F: FnOnce(ModemConfig1) -> ModemConfig1>(&self, f: F) {
        let tmp = ModemConfig1(self.read(REG_MODEM_CONFIG1));
        self.write(REG_MODEM_CONFIG1, f(tmp).0)
    }

    pub fn modem_config2(&self) -> ModemConfig2 {
        ModemConfig2(self.read(REG_MODEM_CONFIG2))
    }
    pub fn set_modem_config2(&self, value: ModemConfig2) {
        self.write(REG_MODEM_CONFIG2, value.0)
    }
    pub fn with_modem_config2<F: FnOnce(ModemConfig2) -> ModemConfig2>(&self, f: F) {
        let tmp = ModemConfig2(self.read(REG_MODEM_CONFIG2));
        self.write(REG_MODEM_CONFIG2, f(tmp).0)
    }

    pub fn symb_timeout_lsb(&self) -> SymbTimeoutLsb {
        SymbTimeoutLsb(self.read(REG_SYMB_TIMEOUT_LSB))
    }
    pub fn set_symb_timeout_lsb(&self, value: SymbTimeoutLsb) {
        self.write(REG_SYMB_TIMEOUT_LSB, value.0)
    }
    pub fn with_symb_timeout_lsb<F: FnOnce(SymbTimeoutLsb) -> SymbTimeoutLsb>(&self, f: F) {
        let tmp = SymbTimeoutLsb(self.read(REG_SYMB_TIMEOUT_LSB));
        self.write(REG_SYMB_TIMEOUT_LSB, f(tmp).0)
    }

    pub fn preamble_msb(&self) -> PreambleMsb {
        PreambleMsb(self.read(REG_PREAMBLE_MSB))
    }
    pub fn set_preamble_msb(&self, value: PreambleMsb) {
        self.write(REG_PREAMBLE_MSB, value.0)
    }
    pub fn with_preamble_msb<F: FnOnce(PreambleMsb) -> PreambleMsb>(&self, f: F) {
        let tmp = PreambleMsb(self.read(REG_PREAMBLE_MSB));
        self.write(REG_PREAMBLE_MSB, f(tmp).0)
    }

    pub fn preamble_lsb(&self) -> PreambleLsb {
        PreambleLsb(self.read(REG_PREAMBLE_LSB))
    }
    pub fn set_preamble_lsb(&self, value: PreambleLsb) {
        self.write(REG_PREAMBLE_LSB, value.0)
    }
    pub fn with_preamble_lsb<F: FnOnce(PreambleLsb) -> PreambleLsb>(&self, f: F) {
        let tmp = PreambleLsb(self.read(REG_PREAMBLE_LSB));
        self.write(REG_PREAMBLE_LSB, f(tmp).0)
    }

    pub fn payload_length(&self) -> PayloadLength {
        PayloadLength(self.read(REG_PAYLOAD_LENGTH))
    }
    pub fn set_payload_length(&self, value: PayloadLength) {
        self.write(REG_PAYLOAD_LENGTH, value.0)
    }
    pub fn with_payload_length<F: FnOnce(PayloadLength) -> PayloadLength>(&self, f: F) {
        let tmp = PayloadLength(self.read(REG_PAYLOAD_LENGTH));
        self.write(REG_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn max_payload_length(&self) -> MaxPayloadLength {
        MaxPayloadLength(self.read(REG_MAX_PAYLOAD_LENGTH))
    }
    pub fn set_max_payload_length(&self, value: MaxPayloadLength) {
        self.write(REG_MAX_PAYLOAD_LENGTH, value.0)
    }
    pub fn with_max_payload_length<F: FnOnce(MaxPayloadLength) -> MaxPayloadLength>(&self, f: F) {
        let tmp = MaxPayloadLength(self.read(REG_MAX_PAYLOAD_LENGTH));
        self.write(REG_MAX_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn hop_period(&self) -> HopPeriod {
        HopPeriod(self.read(REG_HOP_PERIOD))
    }
    pub fn set_hop_period(&self, value: HopPeriod) {
        self.write(REG_HOP_PERIOD, value.0)
    }
    pub fn with_hop_period<F: FnOnce(HopPeriod) -> HopPeriod>(&self, f: F) {
        let tmp = HopPeriod(self.read(REG_HOP_PERIOD));
        self.write(REG_HOP_PERIOD, f(tmp).0)
    }

    pub fn fifo_rx_byte_addr(&self) -> FifoRxByteAddr {
        FifoRxByteAddr(self.read(REG_FIFO_RX_BYTE_ADDR))
    }
    pub fn set_fifo_rx_byte_addr(&self, value: FifoRxByteAddr) {
        self.write(REG_FIFO_RX_BYTE_ADDR, value.0)
    }
    pub fn with_fifo_rx_byte_addr<F: FnOnce(FifoRxByteAddr) -> FifoRxByteAddr>(&self, f: F) {
        let tmp = FifoRxByteAddr(self.read(REG_FIFO_RX_BYTE_ADDR));
        self.write(REG_FIFO_RX_BYTE_ADDR, f(tmp).0)
    }

    pub fn modem_config3(&self) -> ModemConfig3 {
        ModemConfig3(self.read(REG_MODEM_CONFIG3))
    }
    pub fn set_modem_config3(&self, value: ModemConfig3) {
        self.write(REG_MODEM_CONFIG3, value.0)
    }
    pub fn with_modem_config3<F: FnOnce(ModemConfig3) -> ModemConfig3>(&self, f: F) {
        let tmp = ModemConfig3(self.read(REG_MODEM_CONFIG3));
        self.write(REG_MODEM_CONFIG3, f(tmp).0)
    }

    pub fn ppm_correction(&self) -> PpmCorrection {
        PpmCorrection(self.read(REG_PPM_CORRECTION))
    }
    pub fn set_ppm_correction(&self, value: PpmCorrection) {
        self.write(REG_PPM_CORRECTION, value.0)
    }
    pub fn with_ppm_correction<F: FnOnce(PpmCorrection) -> PpmCorrection>(&self, f: F) {
        let tmp = PpmCorrection(self.read(REG_PPM_CORRECTION));
        self.write(REG_PPM_CORRECTION, f(tmp).0)
    }

    pub fn fei_msb(&self) -> FeiMsb {
        FeiMsb(self.read(REG_FEI_MSB))
    }
    pub fn set_fei_msb(&self, value: FeiMsb) {
        self.write(REG_FEI_MSB, value.0)
    }
    pub fn with_fei_msb<F: FnOnce(FeiMsb) -> FeiMsb>(&self, f: F) {
        let tmp = FeiMsb(self.read(REG_FEI_MSB));
        self.write(REG_FEI_MSB, f(tmp).0)
    }

    pub fn fei_mid(&self) -> FeiMid {
        FeiMid(self.read(REG_FEI_MID))
    }
    pub fn set_fei_mid(&self, value: FeiMid) {
        self.write(REG_FEI_MID, value.0)
    }
    pub fn with_fei_mid<F: FnOnce(FeiMid) -> FeiMid>(&self, f: F) {
        let tmp = FeiMid(self.read(REG_FEI_MID));
        self.write(REG_FEI_MID, f(tmp).0)
    }

    pub fn fei_lsb(&self) -> FeiLsb {
        FeiLsb(self.read(REG_FEI_LSB))
    }
    pub fn set_fei_lsb(&self, value: FeiLsb) {
        self.write(REG_FEI_LSB, value.0)
    }
    pub fn with_fei_lsb<F: FnOnce(FeiLsb) -> FeiLsb>(&self, f: F) {
        let tmp = FeiLsb(self.read(REG_FEI_LSB));
        self.write(REG_FEI_LSB, f(tmp).0)
    }

    pub fn rssi_wideband(&self) -> RssiWideband {
        RssiWideband(self.read(REG_RSSI_WIDEBAND))
    }
    pub fn set_rssi_wideband(&self, value: RssiWideband) {
        self.write(REG_RSSI_WIDEBAND, value.0)
    }
    pub fn with_rssi_wideband<F: FnOnce(RssiWideband) -> RssiWideband>(&self, f: F) {
        let tmp = RssiWideband(self.read(REG_RSSI_WIDEBAND));
        self.write(REG_RSSI_WIDEBAND, f(tmp).0)
    }

    pub fn detect_optimize(&self) -> DetectOptimize {
        DetectOptimize(self.read(REG_DETECT_OPTIMIZE))
    }
    pub fn set_detect_optimize(&self, value: DetectOptimize) {
        self.write(REG_DETECT_OPTIMIZE, value.0)
    }
    pub fn with_detect_optimize<F: FnOnce(DetectOptimize) -> DetectOptimize>(&self, f: F) {
        let tmp = DetectOptimize(self.read(REG_DETECT_OPTIMIZE));
        self.write(REG_DETECT_OPTIMIZE, f(tmp).0)
    }

    pub fn invert_iq(&self) -> InvertIq {
        InvertIq(self.read(REG_INVERT_IQ))
    }
    pub fn set_invert_iq(&self, value: InvertIq) {
        self.write(REG_INVERT_IQ, value.0)
    }
    pub fn with_invert_iq<F: FnOnce(InvertIq) -> InvertIq>(&self, f: F) {
        let tmp = InvertIq(self.read(REG_INVERT_IQ));
        self.write(REG_INVERT_IQ, f(tmp).0)
    }

    pub fn detection_threshold(&self) -> DetectionThreshold {
        DetectionThreshold(self.read(REG_DETECTION_THRESHOLD))
    }
    pub fn set_detection_threshold(&self, value: DetectionThreshold) {
        self.write(REG_DETECTION_THRESHOLD, value.0)
    }
    pub fn with_detection_threshold<F: FnOnce(DetectionThreshold) -> DetectionThreshold>(&self, f: F) {
        let tmp = DetectionThreshold(self.read(REG_DETECTION_THRESHOLD));
        self.write(REG_DETECTION_THRESHOLD, f(tmp).0)
    }

    pub fn sync_word(&self) -> SyncWord {
        SyncWord(self.read(REG_SYNC_WORD))
    }
    pub fn set_sync_word(&self, value: SyncWord) {
        self.write(REG_SYNC_WORD, value.0)
    }
    pub fn with_sync_word<F: FnOnce(SyncWord) -> SyncWord>(&self, f: F) {
        let tmp = SyncWord(self.read(REG_SYNC_WORD));
        self.write(REG_SYNC_WORD, f(tmp).0)
    }

    pub fn dio_mapping1(&self) -> DioMapping1 {
        DioMapping1(self.read(REG_DIO_MAPPING1))
    }
    pub fn set_dio_mapping1(&self, value: DioMapping1) {
        self.write(REG_DIO_MAPPING1, value.0)
    }
    pub fn with_dio_mapping1<F: FnOnce(DioMapping1) -> DioMapping1>(&self, f: F) {
        let tmp = DioMapping1(self.read(REG_DIO_MAPPING1));
        self.write(REG_DIO_MAPPING1, f(tmp).0)
    }

    pub fn dio_mapping2(&self) -> DioMapping2 {
        DioMapping2(self.read(REG_DIO_MAPPING2))
    }
    pub fn set_dio_mapping2(&self, value: DioMapping2) {
        self.write(REG_DIO_MAPPING2, value.0)
    }
    pub fn with_dio_mapping2<F: FnOnce(DioMapping2) -> DioMapping2>(&self, f: F) {
        let tmp = DioMapping2(self.read(REG_DIO_MAPPING2));
        self.write(REG_DIO_MAPPING2, f(tmp).0)
    }

    pub fn version(&self) -> Version {
        Version(self.read(REG_VERSION))
    }
    pub fn set_version(&self, value: Version) {
        self.write(REG_VERSION, value.0)
    }
    pub fn with_version<F: FnOnce(Version) -> Version>(&self, f: F) {
        let tmp = Version(self.read(REG_VERSION));
        self.write(REG_VERSION, f(tmp).0)
    }

    pub fn pll_hop(&self) -> PllHop {
        PllHop(self.read(REG_PLL_HOP))
    }
    pub fn set_pll_hop(&self, value: PllHop) {
        self.write(REG_PLL_HOP, value.0)
    }
    pub fn with_pll_hop<F: FnOnce(PllHop) -> PllHop>(&self, f: F) {
        let tmp = PllHop(self.read(REG_PLL_HOP));
        self.write(REG_PLL_HOP, f(tmp).0)
    }

    pub fn tcxo(&self) -> Tcxo {
        Tcxo(self.read(REG_TCXO))
    }
    pub fn set_tcxo(&self, value: Tcxo) {
        self.write(REG_TCXO, value.0)
    }
    pub fn with_tcxo<F: FnOnce(Tcxo) -> Tcxo>(&self, f: F) {
        let tmp = Tcxo(self.read(REG_TCXO));
        self.write(REG_TCXO, f(tmp).0)
    }

    pub fn pa_dac(&self) -> PaDac {
        PaDac(self.read(REG_PA_DAC))
    }
    pub fn set_pa_dac(&self, value: PaDac) {
        self.write(REG_PA_DAC, value.0)
    }
    pub fn with_pa_dac<F: FnOnce(PaDac) -> PaDac>(&self, f: F) {
        let tmp = PaDac(self.read(REG_PA_DAC));
        self.write(REG_PA_DAC, f(tmp).0)
    }

    pub fn former_temp(&self) -> FormerTemp {
        FormerTemp(self.read(REG_FORMER_TEMP))
    }
    pub fn set_former_temp(&self, value: FormerTemp) {
        self.write(REG_FORMER_TEMP, value.0)
    }
    pub fn with_former_temp<F: FnOnce(FormerTemp) -> FormerTemp>(&self, f: F) {
        let tmp = FormerTemp(self.read(REG_FORMER_TEMP));
        self.write(REG_FORMER_TEMP, f(tmp).0)
    }

    pub fn agc_ref(&self) -> AgcRef {
        AgcRef(self.read(REG_AGC_REF))
    }
    pub fn set_agc_ref(&self, value: AgcRef) {
        self.write(REG_AGC_REF, value.0)
    }
    pub fn with_agc_ref<F: FnOnce(AgcRef) -> AgcRef>(&self, f: F) {
        let tmp = AgcRef(self.read(REG_AGC_REF));
        self.write(REG_AGC_REF, f(tmp).0)
    }

    pub fn agc_thresh1(&self) -> AgcThresh1 {
        AgcThresh1(self.read(REG_AGC_THRESH1))
    }
    pub fn set_agc_thresh1(&self, value: AgcThresh1) {
        self.write(REG_AGC_THRESH1, value.0)
    }
    pub fn with_agc_thresh1<F: FnOnce(AgcThresh1) -> AgcThresh1>(&self, f: F) {
        let tmp = AgcThresh1(self.read(REG_AGC_THRESH1));
        self.write(REG_AGC_THRESH1, f(tmp).0)
    }

    pub fn agc_thresh2(&self) -> AgcThresh2 {
        AgcThresh2(self.read(REG_AGC_THRESH2))
    }
    pub fn set_agc_thresh2(&self, value: AgcThresh2) {
        self.write(REG_AGC_THRESH2, value.0)
    }
    pub fn with_agc_thresh2<F: FnOnce(AgcThresh2) -> AgcThresh2>(&self, f: F) {
        let tmp = AgcThresh2(self.read(REG_AGC_THRESH2));
        self.write(REG_AGC_THRESH2, f(tmp).0)
    }

    pub fn agc_thresh3(&self) -> AgcThresh3 {
        AgcThresh3(self.read(REG_AGC_THRESH3))
    }
    pub fn set_agc_thresh3(&self, value: AgcThresh3) {
        self.write(REG_AGC_THRESH3, value.0)
    }
    pub fn with_agc_thresh3<F: FnOnce(AgcThresh3) -> AgcThresh3>(&self, f: F) {
        let tmp = AgcThresh3(self.read(REG_AGC_THRESH3));
        self.write(REG_AGC_THRESH3, f(tmp).0)
    }

    pub fn pll_hf(&self) -> PllHf {
        PllHf(self.read(REG_PLL_HF))
    }
    pub fn set_pll_hf(&self, value: PllHf) {
        self.write(REG_PLL_HF, value.0)
    }
    pub fn with_pll_hf<F: FnOnce(PllHf) -> PllHf>(&self, f: F) {
        let tmp = PllHf(self.read(REG_PLL_HF));
        self.write(REG_PLL_HF, f(tmp).0)
    }

}

impl<RW: TryReadWrite> Rfm95<RW> {
    pub fn try_fifo(&self) -> Result<Fifo, RW::Error> {
        Ok(Fifo(self.try_read(REG_FIFO)?))
    }
    pub fn try_set_fifo(&self, value: Fifo) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO, value.0)
    }
    pub fn try_with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Fifo(self.try_read(REG_FIFO)?);
        self.try_write(REG_FIFO, f(tmp).0)
    }

    pub fn try_opmode(&self) -> Result<Opmode, RW::Error> {
        Ok(Opmode(self.try_read(REG_OPMODE)?))
    }
    pub fn try_set_opmode(&self, value: Opmode) -> Result<(), RW::Error> {
        self.try_write(REG_OPMODE, value.0)
    }
    pub fn try_with_opmode<F: FnOnce(Opmode) -> Opmode>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Opmode(self.try_read(REG_OPMODE)?);
        self.try_write(REG_OPMODE, f(tmp).0)
    }

    pub fn try_frf_msb(&self) -> Result<FrfMsb, RW::Error> {
        Ok(FrfMsb(self.try_read(REG_FRF_MSB)?))
    }
    pub fn try_set_frf_msb(&self, value: FrfMsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_MSB, value.0)
    }
    pub fn try_with_frf_msb<F: FnOnce(FrfMsb) -> FrfMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FrfMsb(self.try_read(REG_FRF_MSB)?);
        self.try_write(REG_FRF_MSB, f(tmp).0)
    }

    pub fn try_frf_mid(&self) -> Result<FrfMid, RW::Error> {
        Ok(FrfMid(self.try_read(REG_FRF_MID)?))
    }
    pub fn try_set_frf_mid(&self, value: FrfMid) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_MID, value.0)
    }
    pub fn try_with_frf_mid<F: FnOnce(FrfMid) -> FrfMid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FrfMid(self.try_read(REG_FRF_MID)?);
        self.try_write(REG_FRF_MID, f(tmp).0)
    }

    pub fn try_frf_lsb(&self) -> Result<FrfLsb, RW::Error> {
        Ok(FrfLsb(self.try_read(REG_FRF_LSB)?))
    }
    pub fn try_set_frf_lsb(&self, value: FrfLsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRF_LSB, value.0)
    }
    pub fn try_with_frf_lsb<F: FnOnce(FrfLsb) -> FrfLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FrfLsb(self.try_read(REG_FRF_LSB)?);
        self.try_write(REG_FRF_LSB, f(tmp).0)
    }

    pub fn try_pa_config(&self) -> Result<PaConfig, RW::Error> {
        Ok(PaConfig(self.try_read(REG_PA_CONFIG)?))
    }
    pub fn try_set_pa_config(&self, value: PaConfig) -> Result<(), RW::Error> {
        self.try_write(REG_PA_CONFIG, value.0)
    }
    pub fn try_with_pa_config<F: FnOnce(PaConfig) -> PaConfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PaConfig(self.try_read(REG_PA_CONFIG)?);
        self.try_write(REG_PA_CONFIG, f(tmp).0)
    }

    pub fn try_pa_ramp(&self) -> Result<PaRamp, RW::Error> {
        Ok(PaRamp(self.try_read(REG_PA_RAMP)?))
    }
    pub fn try_set_pa_ramp(&self, value: PaRamp) -> Result<(), RW::Error> {
        self.try_write(REG_PA_RAMP, value.0)
    }
    pub fn try_with_pa_ramp<F: FnOnce(PaRamp) -> PaRamp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PaRamp(self.try_read(REG_PA_RAMP)?);
        self.try_write(REG_PA_RAMP, f(tmp).0)
    }

    pub fn try_ocp(&self) -> Result<Ocp, RW::Error> {
        Ok(Ocp(self.try_read(REG_OCP)?))
    }
    pub fn try_set_ocp(&self, value: Ocp) -> Result<(), RW::Error> {
        self.try_write(REG_OCP, value.0)
    }
    pub fn try_with_ocp<F: FnOnce(Ocp) -> Ocp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Ocp(self.try_read(REG_OCP)?);
        self.try_write(REG_OCP, f(tmp).0)
    }

    pub fn try_lna(&self) -> Result<Lna, RW::Error> {
        Ok(Lna(self.try_read(REG_LNA)?))
    }
    pub fn try_set_lna(&self, value: Lna) -> Result<(), RW::Error> {
        self.try_write(REG_LNA, value.0)
    }
    pub fn try_with_lna<F: FnOnce(Lna) -> Lna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Lna(self.try_read(REG_LNA)?);
        self.try_write(REG_LNA, f(tmp).0)
    }

    pub fn try_fifo_addr_ptr(&self) -> Result<FifoAddrPtr, RW::Error> {
        Ok(FifoAddrPtr(self.try_read(REG_FIFO_ADDR_PTR)?))
    }
    pub fn try_set_fifo_addr_ptr(&self, value: FifoAddrPtr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_ADDR_PTR, value.0)
    }
    pub fn try_with_fifo_addr_ptr<F: FnOnce(FifoAddrPtr) -> FifoAddrPtr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FifoAddrPtr(self.try_read(REG_FIFO_ADDR_PTR)?);
        self.try_write(REG_FIFO_ADDR_PTR, f(tmp).0)
    }

    pub fn try_fifo_tx_base_addr(&self) -> Result<FifoTxBaseAddr, RW::Error> {
        Ok(FifoTxBaseAddr(self.try_read(REG_FIFO_TX_BASE_ADDR)?))
    }
    pub fn try_set_fifo_tx_base_addr(&self, value: FifoTxBaseAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_TX_BASE_ADDR, value.0)
    }
    pub fn try_with_fifo_tx_base_addr<F: FnOnce(FifoTxBaseAddr) -> FifoTxBaseAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FifoTxBaseAddr(self.try_read(REG_FIFO_TX_BASE_ADDR)?);
        self.try_write(REG_FIFO_TX_BASE_ADDR, f(tmp).0)
    }

    pub fn try_fifo_rx_base_addr(&self) -> Result<FifoRxBaseAddr, RW::Error> {
        Ok(FifoRxBaseAddr(self.try_read(REG_FIFO_RX_BASE_ADDR)?))
    }
    pub fn try_set_fifo_rx_base_addr(&self, value: FifoRxBaseAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_BASE_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_base_addr<F: FnOnce(FifoRxBaseAddr) -> FifoRxBaseAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FifoRxBaseAddr(self.try_read(REG_FIFO_RX_BASE_ADDR)?);
        self.try_write(REG_FIFO_RX_BASE_ADDR, f(tmp).0)
    }

    pub fn try_fifo_rx_current_addr(&self) -> Result<FifoRxCurrentAddr, RW::Error> {
        Ok(FifoRxCurrentAddr(self.try_read(REG_FIFO_RX_CURRENT_ADDR)?))
    }
    pub fn try_set_fifo_rx_current_addr(&self, value: FifoRxCurrentAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_CURRENT_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_current_addr<F: FnOnce(FifoRxCurrentAddr) -> FifoRxCurrentAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FifoRxCurrentAddr(self.try_read(REG_FIFO_RX_CURRENT_ADDR)?);
        self.try_write(REG_FIFO_RX_CURRENT_ADDR, f(tmp).0)
    }

    pub fn try_irq_flags_mask(&self) -> Result<IrqFlagsMask, RW::Error> {
        Ok(IrqFlagsMask(self.try_read(REG_IRQ_FLAGS_MASK)?))
    }
    pub fn try_set_irq_flags_mask(&self, value: IrqFlagsMask) -> Result<(), RW::Error> {
        self.try_write(REG_IRQ_FLAGS_MASK, value.0)
    }
    pub fn try_with_irq_flags_mask<F: FnOnce(IrqFlagsMask) -> IrqFlagsMask>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = IrqFlagsMask(self.try_read(REG_IRQ_FLAGS_MASK)?);
        self.try_write(REG_IRQ_FLAGS_MASK, f(tmp).0)
    }

    pub fn try_irq_flags(&self) -> Result<IrqFlags, RW::Error> {
        Ok(IrqFlags(self.try_read(REG_IRQ_FLAGS)?))
    }
    pub fn try_set_irq_flags(&self, value: IrqFlags) -> Result<(), RW::Error> {
        self.try_write(REG_IRQ_FLAGS, value.0)
    }
    pub fn try_with_irq_flags<F: FnOnce(IrqFlags) -> IrqFlags>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = IrqFlags(self.try_read(REG_IRQ_FLAGS)?);
        self.try_write(REG_IRQ_FLAGS, f(tmp).0)
    }

    pub fn try_rx_nb_bytes(&self) -> Result<RxNbBytes, RW::Error> {
        Ok(RxNbBytes(self.try_read(REG_RX_NB_BYTES)?))
    }
    pub fn try_set_rx_nb_bytes(&self, value: RxNbBytes) -> Result<(), RW::Error> {
        self.try_write(REG_RX_NB_BYTES, value.0)
    }
    pub fn try_with_rx_nb_bytes<F: FnOnce(RxNbBytes) -> RxNbBytes>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RxNbBytes(self.try_read(REG_RX_NB_BYTES)?);
        self.try_write(REG_RX_NB_BYTES, f(tmp).0)
    }

    pub fn try_rx_header_cnt_value_msb(&self) -> Result<RxHeaderCntValueMsb, RW::Error> {
        Ok(RxHeaderCntValueMsb(self.try_read(REG_RX_HEADER_CNT_VALUE_MSB)?))
    }
    pub fn try_set_rx_header_cnt_value_msb(&self, value: RxHeaderCntValueMsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_HEADER_CNT_VALUE_MSB, value.0)
    }
    pub fn try_with_rx_header_cnt_value_msb<F: FnOnce(RxHeaderCntValueMsb) -> RxHeaderCntValueMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RxHeaderCntValueMsb(self.try_read(REG_RX_HEADER_CNT_VALUE_MSB)?);
        self.try_write(REG_RX_HEADER_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn try_rx_header_cnt_value_lsb(&self) -> Result<RxHeaderCntValueLsb, RW::Error> {
        Ok(RxHeaderCntValueLsb(self.try_read(REG_RX_HEADER_CNT_VALUE_LSB)?))
    }
    pub fn try_set_rx_header_cnt_value_lsb(&self, value: RxHeaderCntValueLsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_HEADER_CNT_VALUE_LSB, value.0)
    }
    pub fn try_with_rx_header_cnt_value_lsb<F: FnOnce(RxHeaderCntValueLsb) -> RxHeaderCntValueLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RxHeaderCntValueLsb(self.try_read(REG_RX_HEADER_CNT_VALUE_LSB)?);
        self.try_write(REG_RX_HEADER_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn try_rx_packet_cnt_value_msb(&self) -> Result<RxPacketCntValueMsb, RW::Error> {
        Ok(RxPacketCntValueMsb(self.try_read(REG_RX_PACKET_CNT_VALUE_MSB)?))
    }
    pub fn try_set_rx_packet_cnt_value_msb(&self, value: RxPacketCntValueMsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_PACKET_CNT_VALUE_MSB, value.0)
    }
    pub fn try_with_rx_packet_cnt_value_msb<F: FnOnce(RxPacketCntValueMsb) -> RxPacketCntValueMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RxPacketCntValueMsb(self.try_read(REG_RX_PACKET_CNT_VALUE_MSB)?);
        self.try_write(REG_RX_PACKET_CNT_VALUE_MSB, f(tmp).0)
    }

    pub fn try_rx_packet_cnt_value_lsb(&self) -> Result<RxPacketCntValueLsb, RW::Error> {
        Ok(RxPacketCntValueLsb(self.try_read(REG_RX_PACKET_CNT_VALUE_LSB)?))
    }
    pub fn try_set_rx_packet_cnt_value_lsb(&self, value: RxPacketCntValueLsb) -> Result<(), RW::Error> {
        self.try_write(REG_RX_PACKET_CNT_VALUE_LSB, value.0)
    }
    pub fn try_with_rx_packet_cnt_value_lsb<F: FnOnce(RxPacketCntValueLsb) -> RxPacketCntValueLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RxPacketCntValueLsb(self.try_read(REG_RX_PACKET_CNT_VALUE_LSB)?);
        self.try_write(REG_RX_PACKET_CNT_VALUE_LSB, f(tmp).0)
    }

    pub fn try_modem_stat(&self) -> Result<ModemStat, RW::Error> {
        Ok(ModemStat(self.try_read(REG_MODEM_STAT)?))
    }
    pub fn try_set_modem_stat(&self, value: ModemStat) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_STAT, value.0)
    }
    pub fn try_with_modem_stat<F: FnOnce(ModemStat) -> ModemStat>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = ModemStat(self.try_read(REG_MODEM_STAT)?);
        self.try_write(REG_MODEM_STAT, f(tmp).0)
    }

    pub fn try_pkt_snr_value(&self) -> Result<PktSnrValue, RW::Error> {
        Ok(PktSnrValue(self.try_read(REG_PKT_SNR_VALUE)?))
    }
    pub fn try_set_pkt_snr_value(&self, value: PktSnrValue) -> Result<(), RW::Error> {
        self.try_write(REG_PKT_SNR_VALUE, value.0)
    }
    pub fn try_with_pkt_snr_value<F: FnOnce(PktSnrValue) -> PktSnrValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PktSnrValue(self.try_read(REG_PKT_SNR_VALUE)?);
        self.try_write(REG_PKT_SNR_VALUE, f(tmp).0)
    }

    pub fn try_pkt_rssi_value(&self) -> Result<PktRssiValue, RW::Error> {
        Ok(PktRssiValue(self.try_read(REG_PKT_RSSI_VALUE)?))
    }
    pub fn try_set_pkt_rssi_value(&self, value: PktRssiValue) -> Result<(), RW::Error> {
        self.try_write(REG_PKT_RSSI_VALUE, value.0)
    }
    pub fn try_with_pkt_rssi_value<F: FnOnce(PktRssiValue) -> PktRssiValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PktRssiValue(self.try_read(REG_PKT_RSSI_VALUE)?);
        self.try_write(REG_PKT_RSSI_VALUE, f(tmp).0)
    }

    pub fn try_rssi_value(&self) -> Result<RssiValue, RW::Error> {
        Ok(RssiValue(self.try_read(REG_RSSI_VALUE)?))
    }
    pub fn try_set_rssi_value(&self, value: RssiValue) -> Result<(), RW::Error> {
        self.try_write(REG_RSSI_VALUE, value.0)
    }
    pub fn try_with_rssi_value<F: FnOnce(RssiValue) -> RssiValue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RssiValue(self.try_read(REG_RSSI_VALUE)?);
        self.try_write(REG_RSSI_VALUE, f(tmp).0)
    }

    pub fn try_hop_channel(&self) -> Result<HopChannel, RW::Error> {
        Ok(HopChannel(self.try_read(REG_HOP_CHANNEL)?))
    }
    pub fn try_set_hop_channel(&self, value: HopChannel) -> Result<(), RW::Error> {
        self.try_write(REG_HOP_CHANNEL, value.0)
    }
    pub fn try_with_hop_channel<F: FnOnce(HopChannel) -> HopChannel>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = HopChannel(self.try_read(REG_HOP_CHANNEL)?);
        self.try_write(REG_HOP_CHANNEL, f(tmp).0)
    }

    pub fn try_modem_config1(&self) -> Result<ModemConfig1, RW::Error> {
        Ok(ModemConfig1(self.try_read(REG_MODEM_CONFIG1)?))
    }
    pub fn try_set_modem_config1(&self, value: ModemConfig1) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG1, value.0)
    }
    pub fn try_with_modem_config1<F: FnOnce(ModemConfig1) -> ModemConfig1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = ModemConfig1(self.try_read(REG_MODEM_CONFIG1)?);
        self.try_write(REG_MODEM_CONFIG1, f(tmp).0)
    }

    pub fn try_modem_config2(&self) -> Result<ModemConfig2, RW::Error> {
        Ok(ModemConfig2(self.try_read(REG_MODEM_CONFIG2)?))
    }
    pub fn try_set_modem_config2(&self, value: ModemConfig2) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG2, value.0)
    }
    pub fn try_with_modem_config2<F: FnOnce(ModemConfig2) -> ModemConfig2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = ModemConfig2(self.try_read(REG_MODEM_CONFIG2)?);
        self.try_write(REG_MODEM_CONFIG2, f(tmp).0)
    }

    pub fn try_symb_timeout_lsb(&self) -> Result<SymbTimeoutLsb, RW::Error> {
        Ok(SymbTimeoutLsb(self.try_read(REG_SYMB_TIMEOUT_LSB)?))
    }
    pub fn try_set_symb_timeout_lsb(&self, value: SymbTimeoutLsb) -> Result<(), RW::Error> {
        self.try_write(REG_SYMB_TIMEOUT_LSB, value.0)
    }
    pub fn try_with_symb_timeout_lsb<F: FnOnce(SymbTimeoutLsb) -> SymbTimeoutLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = SymbTimeoutLsb(self.try_read(REG_SYMB_TIMEOUT_LSB)?);
        self.try_write(REG_SYMB_TIMEOUT_LSB, f(tmp).0)
    }

    pub fn try_preamble_msb(&self) -> Result<PreambleMsb, RW::Error> {
        Ok(PreambleMsb(self.try_read(REG_PREAMBLE_MSB)?))
    }
    pub fn try_set_preamble_msb(&self, value: PreambleMsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLE_MSB, value.0)
    }
    pub fn try_with_preamble_msb<F: FnOnce(PreambleMsb) -> PreambleMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PreambleMsb(self.try_read(REG_PREAMBLE_MSB)?);
        self.try_write(REG_PREAMBLE_MSB, f(tmp).0)
    }

    pub fn try_preamble_lsb(&self) -> Result<PreambleLsb, RW::Error> {
        Ok(PreambleLsb(self.try_read(REG_PREAMBLE_LSB)?))
    }
    pub fn try_set_preamble_lsb(&self, value: PreambleLsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLE_LSB, value.0)
    }
    pub fn try_with_preamble_lsb<F: FnOnce(PreambleLsb) -> PreambleLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PreambleLsb(self.try_read(REG_PREAMBLE_LSB)?);
        self.try_write(REG_PREAMBLE_LSB, f(tmp).0)
    }

    pub fn try_payload_length(&self) -> Result<PayloadLength, RW::Error> {
        Ok(PayloadLength(self.try_read(REG_PAYLOAD_LENGTH)?))
    }
    pub fn try_set_payload_length(&self, value: PayloadLength) -> Result<(), RW::Error> {
        self.try_write(REG_PAYLOAD_LENGTH, value.0)
    }
    pub fn try_with_payload_length<F: FnOnce(PayloadLength) -> PayloadLength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PayloadLength(self.try_read(REG_PAYLOAD_LENGTH)?);
        self.try_write(REG_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn try_max_payload_length(&self) -> Result<MaxPayloadLength, RW::Error> {
        Ok(MaxPayloadLength(self.try_read(REG_MAX_PAYLOAD_LENGTH)?))
    }
    pub fn try_set_max_payload_length(&self, value: MaxPayloadLength) -> Result<(), RW::Error> {
        self.try_write(REG_MAX_PAYLOAD_LENGTH, value.0)
    }
    pub fn try_with_max_payload_length<F: FnOnce(MaxPayloadLength) -> MaxPayloadLength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = MaxPayloadLength(self.try_read(REG_MAX_PAYLOAD_LENGTH)?);
        self.try_write(REG_MAX_PAYLOAD_LENGTH, f(tmp).0)
    }

    pub fn try_hop_period(&self) -> Result<HopPeriod, RW::Error> {
        Ok(HopPeriod(self.try_read(REG_HOP_PERIOD)?))
    }
    pub fn try_set_hop_period(&self, value: HopPeriod) -> Result<(), RW::Error> {
        self.try_write(REG_HOP_PERIOD, value.0)
    }
    pub fn try_with_hop_period<F: FnOnce(HopPeriod) -> HopPeriod>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = HopPeriod(self.try_read(REG_HOP_PERIOD)?);
        self.try_write(REG_HOP_PERIOD, f(tmp).0)
    }

    pub fn try_fifo_rx_byte_addr(&self) -> Result<FifoRxByteAddr, RW::Error> {
        Ok(FifoRxByteAddr(self.try_read(REG_FIFO_RX_BYTE_ADDR)?))
    }
    pub fn try_set_fifo_rx_byte_addr(&self, value: FifoRxByteAddr) -> Result<(), RW::Error> {
        self.try_write(REG_FIFO_RX_BYTE_ADDR, value.0)
    }
    pub fn try_with_fifo_rx_byte_addr<F: FnOnce(FifoRxByteAddr) -> FifoRxByteAddr>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FifoRxByteAddr(self.try_read(REG_FIFO_RX_BYTE_ADDR)?);
        self.try_write(REG_FIFO_RX_BYTE_ADDR, f(tmp).0)
    }

    pub fn try_modem_config3(&self) -> Result<ModemConfig3, RW::Error> {
        Ok(ModemConfig3(self.try_read(REG_MODEM_CONFIG3)?))
    }
    pub fn try_set_modem_config3(&self, value: ModemConfig3) -> Result<(), RW::Error> {
        self.try_write(REG_MODEM_CONFIG3, value.0)
    }
    pub fn try_with_modem_config3<F: FnOnce(ModemConfig3) -> ModemConfig3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = ModemConfig3(self.try_read(REG_MODEM_CONFIG3)?);
        self.try_write(REG_MODEM_CONFIG3, f(tmp).0)
    }

    pub fn try_ppm_correction(&self) -> Result<PpmCorrection, RW::Error> {
        Ok(PpmCorrection(self.try_read(REG_PPM_CORRECTION)?))
    }
    pub fn try_set_ppm_correction(&self, value: PpmCorrection) -> Result<(), RW::Error> {
        self.try_write(REG_PPM_CORRECTION, value.0)
    }
    pub fn try_with_ppm_correction<F: FnOnce(PpmCorrection) -> PpmCorrection>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PpmCorrection(self.try_read(REG_PPM_CORRECTION)?);
        self.try_write(REG_PPM_CORRECTION, f(tmp).0)
    }

    pub fn try_fei_msb(&self) -> Result<FeiMsb, RW::Error> {
        Ok(FeiMsb(self.try_read(REG_FEI_MSB)?))
    }
    pub fn try_set_fei_msb(&self, value: FeiMsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_MSB, value.0)
    }
    pub fn try_with_fei_msb<F: FnOnce(FeiMsb) -> FeiMsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FeiMsb(self.try_read(REG_FEI_MSB)?);
        self.try_write(REG_FEI_MSB, f(tmp).0)
    }

    pub fn try_fei_mid(&self) -> Result<FeiMid, RW::Error> {
        Ok(FeiMid(self.try_read(REG_FEI_MID)?))
    }
    pub fn try_set_fei_mid(&self, value: FeiMid) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_MID, value.0)
    }
    pub fn try_with_fei_mid<F: FnOnce(FeiMid) -> FeiMid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FeiMid(self.try_read(REG_FEI_MID)?);
        self.try_write(REG_FEI_MID, f(tmp).0)
    }

    pub fn try_fei_lsb(&self) -> Result<FeiLsb, RW::Error> {
        Ok(FeiLsb(self.try_read(REG_FEI_LSB)?))
    }
    pub fn try_set_fei_lsb(&self, value: FeiLsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEI_LSB, value.0)
    }
    pub fn try_with_fei_lsb<F: FnOnce(FeiLsb) -> FeiLsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FeiLsb(self.try_read(REG_FEI_LSB)?);
        self.try_write(REG_FEI_LSB, f(tmp).0)
    }

    pub fn try_rssi_wideband(&self) -> Result<RssiWideband, RW::Error> {
        Ok(RssiWideband(self.try_read(REG_RSSI_WIDEBAND)?))
    }
    pub fn try_set_rssi_wideband(&self, value: RssiWideband) -> Result<(), RW::Error> {
        self.try_write(REG_RSSI_WIDEBAND, value.0)
    }
    pub fn try_with_rssi_wideband<F: FnOnce(RssiWideband) -> RssiWideband>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = RssiWideband(self.try_read(REG_RSSI_WIDEBAND)?);
        self.try_write(REG_RSSI_WIDEBAND, f(tmp).0)
    }

    pub fn try_detect_optimize(&self) -> Result<DetectOptimize, RW::Error> {
        Ok(DetectOptimize(self.try_read(REG_DETECT_OPTIMIZE)?))
    }
    pub fn try_set_detect_optimize(&self, value: DetectOptimize) -> Result<(), RW::Error> {
        self.try_write(REG_DETECT_OPTIMIZE, value.0)
    }
    pub fn try_with_detect_optimize<F: FnOnce(DetectOptimize) -> DetectOptimize>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = DetectOptimize(self.try_read(REG_DETECT_OPTIMIZE)?);
        self.try_write(REG_DETECT_OPTIMIZE, f(tmp).0)
    }

    pub fn try_invert_iq(&self) -> Result<InvertIq, RW::Error> {
        Ok(InvertIq(self.try_read(REG_INVERT_IQ)?))
    }
    pub fn try_set_invert_iq(&self, value: InvertIq) -> Result<(), RW::Error> {
        self.try_write(REG_INVERT_IQ, value.0)
    }
    pub fn try_with_invert_iq<F: FnOnce(InvertIq) -> InvertIq>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = InvertIq(self.try_read(REG_INVERT_IQ)?);
        self.try_write(REG_INVERT_IQ, f(tmp).0)
    }

    pub fn try_detection_threshold(&self) -> Result<DetectionThreshold, RW::Error> {
        Ok(DetectionThreshold(self.try_read(REG_DETECTION_THRESHOLD)?))
    }
    pub fn try_set_detection_threshold(&self, value: DetectionThreshold) -> Result<(), RW::Error> {
        self.try_write(REG_DETECTION_THRESHOLD, value.0)
    }
    pub fn try_with_detection_threshold<F: FnOnce(DetectionThreshold) -> DetectionThreshold>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = DetectionThreshold(self.try_read(REG_DETECTION_THRESHOLD)?);
        self.try_write(REG_DETECTION_THRESHOLD, f(tmp).0)
    }

    pub fn try_sync_word(&self) -> Result<SyncWord, RW::Error> {
        Ok(SyncWord(self.try_read(REG_SYNC_WORD)?))
    }
    pub fn try_set_sync_word(&self, value: SyncWord) -> Result<(), RW::Error> {
        self.try_write(REG_SYNC_WORD, value.0)
    }
    pub fn try_with_sync_word<F: FnOnce(SyncWord) -> SyncWord>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = SyncWord(self.try_read(REG_SYNC_WORD)?);
        self.try_write(REG_SYNC_WORD, f(tmp).0)
    }

    pub fn try_dio_mapping1(&self) -> Result<DioMapping1, RW::Error> {
        Ok(DioMapping1(self.try_read(REG_DIO_MAPPING1)?))
    }
    pub fn try_set_dio_mapping1(&self, value: DioMapping1) -> Result<(), RW::Error> {
        self.try_write(REG_DIO_MAPPING1, value.0)
    }
    pub fn try_with_dio_mapping1<F: FnOnce(DioMapping1) -> DioMapping1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = DioMapping1(self.try_read(REG_DIO_MAPPING1)?);
        self.try_write(REG_DIO_MAPPING1, f(tmp).0)
    }

    pub fn try_dio_mapping2(&self) -> Result<DioMapping2, RW::Error> {
        Ok(DioMapping2(self.try_read(REG_DIO_MAPPING2)?))
    }
    pub fn try_set_dio_mapping2(&self, value: DioMapping2) -> Result<(), RW::Error> {
        self.try_write(REG_DIO_MAPPING2, value.0)
    }
    pub fn try_with_dio_mapping2<F: FnOnce(DioMapping2) -> DioMapping2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = DioMapping2(self.try_read(REG_DIO_MAPPING2)?);
        self.try_write(REG_DIO_MAPPING2, f(tmp).0)
    }

    pub fn try_version(&self) -> Result<Version, RW::Error> {
        Ok(Version(self.try_read(REG_VERSION)?))
    }
    pub fn try_set_version(&self, value: Version) -> Result<(), RW::Error> {
        self.try_write(REG_VERSION, value.0)
    }
    pub fn try_with_version<F: FnOnce(Version) -> Version>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Version(self.try_read(REG_VERSION)?);
        self.try_write(REG_VERSION, f(tmp).0)
    }

    pub fn try_pll_hop(&self) -> Result<PllHop, RW::Error> {
        Ok(PllHop(self.try_read(REG_PLL_HOP)?))
    }
    pub fn try_set_pll_hop(&self, value: PllHop) -> Result<(), RW::Error> {
        self.try_write(REG_PLL_HOP, value.0)
    }
    pub fn try_with_pll_hop<F: FnOnce(PllHop) -> PllHop>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PllHop(self.try_read(REG_PLL_HOP)?);
        self.try_write(REG_PLL_HOP, f(tmp).0)
    }

    pub fn try_tcxo(&self) -> Result<Tcxo, RW::Error> {
        Ok(Tcxo(self.try_read(REG_TCXO)?))
    }
    pub fn try_set_tcxo(&self, value: Tcxo) -> Result<(), RW::Error> {
        self.try_write(REG_TCXO, value.0)
    }
    pub fn try_with_tcxo<F: FnOnce(Tcxo) -> Tcxo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Tcxo(self.try_read(REG_TCXO)?);
        self.try_write(REG_TCXO, f(tmp).0)
    }

    pub fn try_pa_dac(&self) -> Result<PaDac, RW::Error> {
        Ok(PaDac(self.try_read(REG_PA_DAC)?))
    }
    pub fn try_set_pa_dac(&self, value: PaDac) -> Result<(), RW::Error> {
        self.try_write(REG_PA_DAC, value.0)
    }
    pub fn try_with_pa_dac<F: FnOnce(PaDac) -> PaDac>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PaDac(self.try_read(REG_PA_DAC)?);
        self.try_write(REG_PA_DAC, f(tmp).0)
    }

    pub fn try_former_temp(&self) -> Result<FormerTemp, RW::Error> {
        Ok(FormerTemp(self.try_read(REG_FORMER_TEMP)?))
    }
    pub fn try_set_former_temp(&self, value: FormerTemp) -> Result<(), RW::Error> {
        self.try_write(REG_FORMER_TEMP, value.0)
    }
    pub fn try_with_former_temp<F: FnOnce(FormerTemp) -> FormerTemp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = FormerTemp(self.try_read(REG_FORMER_TEMP)?);
        self.try_write(REG_FORMER_TEMP, f(tmp).0)
    }

    pub fn try_agc_ref(&self) -> Result<AgcRef, RW::Error> {
        Ok(AgcRef(self.try_read(REG_AGC_REF)?))
    }
    pub fn try_set_agc_ref(&self, value: AgcRef) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_REF, value.0)
    }
    pub fn try_with_agc_ref<F: FnOnce(AgcRef) -> AgcRef>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = AgcRef(self.try_read(REG_AGC_REF)?);
        self.try_write(REG_AGC_REF, f(tmp).0)
    }

    pub fn try_agc_thresh1(&self) -> Result<AgcThresh1, RW::Error> {
        Ok(AgcThresh1(self.try_read(REG_AGC_THRESH1)?))
    }
    pub fn try_set_agc_thresh1(&self, value: AgcThresh1) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH1, value.0)
    }
    pub fn try_with_agc_thresh1<F: FnOnce(AgcThresh1) -> AgcThresh1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = AgcThresh1(self.try_read(REG_AGC_THRESH1)?);
        self.try_write(REG_AGC_THRESH1, f(tmp).0)
    }

    pub fn try_agc_thresh2(&self) -> Result<AgcThresh2, RW::Error> {
        Ok(AgcThresh2(self.try_read(REG_AGC_THRESH2)?))
    }
    pub fn try_set_agc_thresh2(&self, value: AgcThresh2) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH2, value.0)
    }
    pub fn try_with_agc_thresh2<F: FnOnce(AgcThresh2) -> AgcThresh2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = AgcThresh2(self.try_read(REG_AGC_THRESH2)?);
        self.try_write(REG_AGC_THRESH2, f(tmp).0)
    }

    pub fn try_agc_thresh3(&self) -> Result<AgcThresh3, RW::Error> {
        Ok(AgcThresh3(self.try_read(REG_AGC_THRESH3)?))
    }
    pub fn try_set_agc_thresh3(&self, value: AgcThresh3) -> Result<(), RW::Error> {
        self.try_write(REG_AGC_THRESH3, value.0)
    }
    pub fn try_with_agc_thresh3<F: FnOnce(AgcThresh3) -> AgcThresh3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = AgcThresh3(self.try_read(REG_AGC_THRESH3)?);
        self.try_write(REG_AGC_THRESH3, f(tmp).0)
    }

    pub fn try_pll_hf(&self) -> Result<PllHf, RW::Error> {
        Ok(PllHf(self.try_read(REG_PLL_HF)?))
    }
    pub fn try_set_pll_hf(&self, value: PllHf) -> Result<(), RW::Error> {
        self.try_write(REG_PLL_HF, value.0)
    }
    pub fn try_with_pll_hf<F: FnOnce(PllHf) -> PllHf>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = PllHf(self.try_read(REG_PLL_HF)?);
        self.try_write(REG_PLL_HF, f(tmp).0)
    }

}

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

