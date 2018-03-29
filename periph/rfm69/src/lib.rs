#![doc="The RFM69HCW is a transceiver module capable of operation over a wide frequency range, including the 315,433,868 and 915MHz license-free ISM (Industry Scientific and Medical) frequency bands."]
#![no_std]

extern crate bobbin_bits;

pub type Addr = u8;
pub type Value = u8;

pub mod addr {
    use super::Addr;
    pub const REG_FIFO: Addr = 0x00;
    pub const REG_OPMODE: Addr = 0x01;
    pub const REG_DATAMODUL: Addr = 0x02;
    pub const REG_BITRATEMSB: Addr = 0x03;
    pub const REG_BITRATELSB: Addr = 0x04;
    pub const REG_FDEVMSB: Addr = 0x05;
    pub const REG_FDEVLSB: Addr = 0x06;
    pub const REG_FRFMSB: Addr = 0x07;
    pub const REG_FRFMID: Addr = 0x08;
    pub const REG_FRFLSB: Addr = 0x09;
    pub const REG_OSC1: Addr = 0x0a;
    pub const REG_AFCCTRL: Addr = 0x0b;
    pub const REG_LISTEN1: Addr = 0x0d;
    pub const REG_LISTEN2: Addr = 0x0e;
    pub const REG_LISTEN3: Addr = 0x0f;
    pub const REG_VERSION: Addr = 0x10;
    pub const REG_PALEVEL: Addr = 0x11;
    pub const REG_PARAMP: Addr = 0x12;
    pub const REG_OCP: Addr = 0x13;
    pub const REG_LNA: Addr = 0x18;
    pub const REG_RXBW: Addr = 0x19;
    pub const REG_AFCBW: Addr = 0x1a;
    pub const REG_OOKPEAK: Addr = 0x1b;
    pub const REG_OOKAVG: Addr = 0x1c;
    pub const REG_OOKFIX: Addr = 0x1d;
    pub const REG_AFCFEI: Addr = 0x1e;
    pub const REG_AFCMSB: Addr = 0x1f;
    pub const REG_AFCLSB: Addr = 0x20;
    pub const REG_FEIMSB: Addr = 0x21;
    pub const REG_FEILSB: Addr = 0x22;
    pub const REG_RSSICONFIG: Addr = 0x23;
    pub const REG_RSSIVALUE: Addr = 0x24;
    pub const REG_DIOMAPPING1: Addr = 0x25;
    pub const REG_DIOMAPPING2: Addr = 0x26;
    pub const REG_IRQFLAGS1: Addr = 0x27;
    pub const REG_IRQFLAGS2: Addr = 0x28;
    pub const REG_RSSITHRESH: Addr = 0x29;
    pub const REG_RXTIMEOUT1: Addr = 0x2a;
    pub const REG_RXTIMEOUT2: Addr = 0x2b;
    pub const REG_PREAMBLEMSB: Addr = 0x2c;
    pub const REG_PREAMBLELSB: Addr = 0x2d;
    pub const REG_SYNCCONFIG: Addr = 0x2e;
    pub const REG_SYNCVALUE: Addr = 0x2f;
    pub const REG_SYNCVALUE1: Addr = 0x2f;
    pub const REG_SYNCVALUE2: Addr = 0x30;
    pub const REG_SYNCVALUE3: Addr = 0x31;
    pub const REG_SYNCVALUE4: Addr = 0x32;
    pub const REG_SYNCVALUE5: Addr = 0x33;
    pub const REG_SYNCVALUE6: Addr = 0x34;
    pub const REG_SYNCVALUE7: Addr = 0x35;
    pub const REG_SYNCVALUE8: Addr = 0x36;
    pub const REG_PACKETCONFIG1: Addr = 0x37;
    pub const REG_PAYLOADLENGTH: Addr = 0x38;
    pub const REG_NODEADRS: Addr = 0x39;
    pub const REG_BROADCASTADRS: Addr = 0x3a;
    pub const REG_AUTOMODES: Addr = 0x3b;
    pub const REG_FIFOTHRESH: Addr = 0x3c;
    pub const REG_PACKETCONFIG2: Addr = 0x3d;
    pub const REG_AESKEY: Addr = 0x3e;
    pub const REG_AESKEY1: Addr = 0x3e;
    pub const REG_AESKEY2: Addr = 0x3f;
    pub const REG_AESKEY3: Addr = 0x40;
    pub const REG_AESKEY4: Addr = 0x41;
    pub const REG_AESKEY5: Addr = 0x42;
    pub const REG_AESKEY6: Addr = 0x43;
    pub const REG_AESKEY7: Addr = 0x44;
    pub const REG_AESKEY8: Addr = 0x45;
    pub const REG_AESKEY9: Addr = 0x46;
    pub const REG_AESKEY10: Addr = 0x47;
    pub const REG_AESKEY11: Addr = 0x48;
    pub const REG_AESKEY12: Addr = 0x49;
    pub const REG_AESKEY13: Addr = 0x4a;
    pub const REG_AESKEY14: Addr = 0x4b;
    pub const REG_AESKEY15: Addr = 0x4c;
    pub const REG_AESKEY16: Addr = 0x4d;
    pub const REG_TEMP1: Addr = 0x4e;
    pub const REG_TEMP2: Addr = 0x4f;
    pub const REG_TESTLNA: Addr = 0x58;
    pub const REG_TESTPA1: Addr = 0x5a;
    pub const REG_TESTPA2: Addr = 0x5c;
    pub const REG_TESTDAGC: Addr = 0x6f;
    pub const REG_TESTAFC: Addr = 0x71;
}

pub struct Rfm69<RW> { rw: RW }

pub trait ReadWrite {
    fn read(&self, addr: Addr) -> Value;
    fn write(&self, addr: Addr, val: Value);
}

pub trait TryReadWrite {
    type Error;
    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error>;
    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error>;
}

impl<RW: ReadWrite> ReadWrite for Rfm69<RW> {
    fn read(&self, addr: Addr) -> Value { self.rw.read(addr) }
    fn write(&self, addr: Addr, val: Value) { self.rw.write(addr, val) }
}

impl<RW: TryReadWrite> TryReadWrite for Rfm69<RW> {
    type Error = RW::Error;
    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error> { self.rw.try_read(addr) }
    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error> { self.rw.try_write(addr, val) }
}

impl<RW> Rfm69<RW> {
    pub fn new(rw: RW) -> Self { Rfm69 { rw } }
}

impl<RW: ReadWrite> Rfm69<RW> {
    pub fn fifo(&self) -> types::Fifo {
        types::Fifo(self.read(addr::REG_FIFO))
    }
    pub fn set_fifo(&self, value: types::Fifo) {
        self.write(addr::REG_FIFO, value.0)
    }
    pub fn with_fifo<F: FnOnce(types::Fifo) -> types::Fifo>(&self, f: F) {
        let tmp = types::Fifo(self.read(addr::REG_FIFO));
        self.write(addr::REG_FIFO, f(tmp).0)
    }

    pub fn opmode(&self) -> types::Opmode {
        types::Opmode(self.read(addr::REG_OPMODE))
    }
    pub fn set_opmode(&self, value: types::Opmode) {
        self.write(addr::REG_OPMODE, value.0)
    }
    pub fn with_opmode<F: FnOnce(types::Opmode) -> types::Opmode>(&self, f: F) {
        let tmp = types::Opmode(self.read(addr::REG_OPMODE));
        self.write(addr::REG_OPMODE, f(tmp).0)
    }

    pub fn datamodul(&self) -> types::Datamodul {
        types::Datamodul(self.read(addr::REG_DATAMODUL))
    }
    pub fn set_datamodul(&self, value: types::Datamodul) {
        self.write(addr::REG_DATAMODUL, value.0)
    }
    pub fn with_datamodul<F: FnOnce(types::Datamodul) -> types::Datamodul>(&self, f: F) {
        let tmp = types::Datamodul(self.read(addr::REG_DATAMODUL));
        self.write(addr::REG_DATAMODUL, f(tmp).0)
    }

    pub fn bitratemsb(&self) -> types::Bitratemsb {
        types::Bitratemsb(self.read(addr::REG_BITRATEMSB))
    }
    pub fn set_bitratemsb(&self, value: types::Bitratemsb) {
        self.write(addr::REG_BITRATEMSB, value.0)
    }
    pub fn with_bitratemsb<F: FnOnce(types::Bitratemsb) -> types::Bitratemsb>(&self, f: F) {
        let tmp = types::Bitratemsb(self.read(addr::REG_BITRATEMSB));
        self.write(addr::REG_BITRATEMSB, f(tmp).0)
    }

    pub fn bitratelsb(&self) -> types::Bitratelsb {
        types::Bitratelsb(self.read(addr::REG_BITRATELSB))
    }
    pub fn set_bitratelsb(&self, value: types::Bitratelsb) {
        self.write(addr::REG_BITRATELSB, value.0)
    }
    pub fn with_bitratelsb<F: FnOnce(types::Bitratelsb) -> types::Bitratelsb>(&self, f: F) {
        let tmp = types::Bitratelsb(self.read(addr::REG_BITRATELSB));
        self.write(addr::REG_BITRATELSB, f(tmp).0)
    }

    pub fn fdevmsb(&self) -> types::Fdevmsb {
        types::Fdevmsb(self.read(addr::REG_FDEVMSB))
    }
    pub fn set_fdevmsb(&self, value: types::Fdevmsb) {
        self.write(addr::REG_FDEVMSB, value.0)
    }
    pub fn with_fdevmsb<F: FnOnce(types::Fdevmsb) -> types::Fdevmsb>(&self, f: F) {
        let tmp = types::Fdevmsb(self.read(addr::REG_FDEVMSB));
        self.write(addr::REG_FDEVMSB, f(tmp).0)
    }

    pub fn fdevlsb(&self) -> types::Fdevlsb {
        types::Fdevlsb(self.read(addr::REG_FDEVLSB))
    }
    pub fn set_fdevlsb(&self, value: types::Fdevlsb) {
        self.write(addr::REG_FDEVLSB, value.0)
    }
    pub fn with_fdevlsb<F: FnOnce(types::Fdevlsb) -> types::Fdevlsb>(&self, f: F) {
        let tmp = types::Fdevlsb(self.read(addr::REG_FDEVLSB));
        self.write(addr::REG_FDEVLSB, f(tmp).0)
    }

    pub fn frfmsb(&self) -> types::Frfmsb {
        types::Frfmsb(self.read(addr::REG_FRFMSB))
    }
    pub fn set_frfmsb(&self, value: types::Frfmsb) {
        self.write(addr::REG_FRFMSB, value.0)
    }
    pub fn with_frfmsb<F: FnOnce(types::Frfmsb) -> types::Frfmsb>(&self, f: F) {
        let tmp = types::Frfmsb(self.read(addr::REG_FRFMSB));
        self.write(addr::REG_FRFMSB, f(tmp).0)
    }

    pub fn frfmid(&self) -> types::Frfmid {
        types::Frfmid(self.read(addr::REG_FRFMID))
    }
    pub fn set_frfmid(&self, value: types::Frfmid) {
        self.write(addr::REG_FRFMID, value.0)
    }
    pub fn with_frfmid<F: FnOnce(types::Frfmid) -> types::Frfmid>(&self, f: F) {
        let tmp = types::Frfmid(self.read(addr::REG_FRFMID));
        self.write(addr::REG_FRFMID, f(tmp).0)
    }

    pub fn frflsb(&self) -> types::Frflsb {
        types::Frflsb(self.read(addr::REG_FRFLSB))
    }
    pub fn set_frflsb(&self, value: types::Frflsb) {
        self.write(addr::REG_FRFLSB, value.0)
    }
    pub fn with_frflsb<F: FnOnce(types::Frflsb) -> types::Frflsb>(&self, f: F) {
        let tmp = types::Frflsb(self.read(addr::REG_FRFLSB));
        self.write(addr::REG_FRFLSB, f(tmp).0)
    }

    pub fn osc1(&self) -> types::Osc1 {
        types::Osc1(self.read(addr::REG_OSC1))
    }
    pub fn set_osc1(&self, value: types::Osc1) {
        self.write(addr::REG_OSC1, value.0)
    }
    pub fn with_osc1<F: FnOnce(types::Osc1) -> types::Osc1>(&self, f: F) {
        let tmp = types::Osc1(self.read(addr::REG_OSC1));
        self.write(addr::REG_OSC1, f(tmp).0)
    }

    pub fn afcctrl(&self) -> types::Afcctrl {
        types::Afcctrl(self.read(addr::REG_AFCCTRL))
    }
    pub fn set_afcctrl(&self, value: types::Afcctrl) {
        self.write(addr::REG_AFCCTRL, value.0)
    }
    pub fn with_afcctrl<F: FnOnce(types::Afcctrl) -> types::Afcctrl>(&self, f: F) {
        let tmp = types::Afcctrl(self.read(addr::REG_AFCCTRL));
        self.write(addr::REG_AFCCTRL, f(tmp).0)
    }

    pub fn listen1(&self) -> types::Listen1 {
        types::Listen1(self.read(addr::REG_LISTEN1))
    }
    pub fn set_listen1(&self, value: types::Listen1) {
        self.write(addr::REG_LISTEN1, value.0)
    }
    pub fn with_listen1<F: FnOnce(types::Listen1) -> types::Listen1>(&self, f: F) {
        let tmp = types::Listen1(self.read(addr::REG_LISTEN1));
        self.write(addr::REG_LISTEN1, f(tmp).0)
    }

    pub fn listen2(&self) -> types::Listen2 {
        types::Listen2(self.read(addr::REG_LISTEN2))
    }
    pub fn set_listen2(&self, value: types::Listen2) {
        self.write(addr::REG_LISTEN2, value.0)
    }
    pub fn with_listen2<F: FnOnce(types::Listen2) -> types::Listen2>(&self, f: F) {
        let tmp = types::Listen2(self.read(addr::REG_LISTEN2));
        self.write(addr::REG_LISTEN2, f(tmp).0)
    }

    pub fn listen3(&self) -> types::Listen3 {
        types::Listen3(self.read(addr::REG_LISTEN3))
    }
    pub fn set_listen3(&self, value: types::Listen3) {
        self.write(addr::REG_LISTEN3, value.0)
    }
    pub fn with_listen3<F: FnOnce(types::Listen3) -> types::Listen3>(&self, f: F) {
        let tmp = types::Listen3(self.read(addr::REG_LISTEN3));
        self.write(addr::REG_LISTEN3, f(tmp).0)
    }

    pub fn version(&self) -> types::Version {
        types::Version(self.read(addr::REG_VERSION))
    }
    pub fn set_version(&self, value: types::Version) {
        self.write(addr::REG_VERSION, value.0)
    }
    pub fn with_version<F: FnOnce(types::Version) -> types::Version>(&self, f: F) {
        let tmp = types::Version(self.read(addr::REG_VERSION));
        self.write(addr::REG_VERSION, f(tmp).0)
    }

    pub fn palevel(&self) -> types::Palevel {
        types::Palevel(self.read(addr::REG_PALEVEL))
    }
    pub fn set_palevel(&self, value: types::Palevel) {
        self.write(addr::REG_PALEVEL, value.0)
    }
    pub fn with_palevel<F: FnOnce(types::Palevel) -> types::Palevel>(&self, f: F) {
        let tmp = types::Palevel(self.read(addr::REG_PALEVEL));
        self.write(addr::REG_PALEVEL, f(tmp).0)
    }

    pub fn paramp(&self) -> types::Paramp {
        types::Paramp(self.read(addr::REG_PARAMP))
    }
    pub fn set_paramp(&self, value: types::Paramp) {
        self.write(addr::REG_PARAMP, value.0)
    }
    pub fn with_paramp<F: FnOnce(types::Paramp) -> types::Paramp>(&self, f: F) {
        let tmp = types::Paramp(self.read(addr::REG_PARAMP));
        self.write(addr::REG_PARAMP, f(tmp).0)
    }

    pub fn ocp(&self) -> types::Ocp {
        types::Ocp(self.read(addr::REG_OCP))
    }
    pub fn set_ocp(&self, value: types::Ocp) {
        self.write(addr::REG_OCP, value.0)
    }
    pub fn with_ocp<F: FnOnce(types::Ocp) -> types::Ocp>(&self, f: F) {
        let tmp = types::Ocp(self.read(addr::REG_OCP));
        self.write(addr::REG_OCP, f(tmp).0)
    }

    pub fn lna(&self) -> types::Lna {
        types::Lna(self.read(addr::REG_LNA))
    }
    pub fn set_lna(&self, value: types::Lna) {
        self.write(addr::REG_LNA, value.0)
    }
    pub fn with_lna<F: FnOnce(types::Lna) -> types::Lna>(&self, f: F) {
        let tmp = types::Lna(self.read(addr::REG_LNA));
        self.write(addr::REG_LNA, f(tmp).0)
    }

    pub fn rxbw(&self) -> types::Rxbw {
        types::Rxbw(self.read(addr::REG_RXBW))
    }
    pub fn set_rxbw(&self, value: types::Rxbw) {
        self.write(addr::REG_RXBW, value.0)
    }
    pub fn with_rxbw<F: FnOnce(types::Rxbw) -> types::Rxbw>(&self, f: F) {
        let tmp = types::Rxbw(self.read(addr::REG_RXBW));
        self.write(addr::REG_RXBW, f(tmp).0)
    }

    pub fn afcbw(&self) -> types::Afcbw {
        types::Afcbw(self.read(addr::REG_AFCBW))
    }
    pub fn set_afcbw(&self, value: types::Afcbw) {
        self.write(addr::REG_AFCBW, value.0)
    }
    pub fn with_afcbw<F: FnOnce(types::Afcbw) -> types::Afcbw>(&self, f: F) {
        let tmp = types::Afcbw(self.read(addr::REG_AFCBW));
        self.write(addr::REG_AFCBW, f(tmp).0)
    }

    pub fn ookpeak(&self) -> types::Ookpeak {
        types::Ookpeak(self.read(addr::REG_OOKPEAK))
    }
    pub fn set_ookpeak(&self, value: types::Ookpeak) {
        self.write(addr::REG_OOKPEAK, value.0)
    }
    pub fn with_ookpeak<F: FnOnce(types::Ookpeak) -> types::Ookpeak>(&self, f: F) {
        let tmp = types::Ookpeak(self.read(addr::REG_OOKPEAK));
        self.write(addr::REG_OOKPEAK, f(tmp).0)
    }

    pub fn ookavg(&self) -> types::Ookavg {
        types::Ookavg(self.read(addr::REG_OOKAVG))
    }
    pub fn set_ookavg(&self, value: types::Ookavg) {
        self.write(addr::REG_OOKAVG, value.0)
    }
    pub fn with_ookavg<F: FnOnce(types::Ookavg) -> types::Ookavg>(&self, f: F) {
        let tmp = types::Ookavg(self.read(addr::REG_OOKAVG));
        self.write(addr::REG_OOKAVG, f(tmp).0)
    }

    pub fn ookfix(&self) -> types::Ookfix {
        types::Ookfix(self.read(addr::REG_OOKFIX))
    }
    pub fn set_ookfix(&self, value: types::Ookfix) {
        self.write(addr::REG_OOKFIX, value.0)
    }
    pub fn with_ookfix<F: FnOnce(types::Ookfix) -> types::Ookfix>(&self, f: F) {
        let tmp = types::Ookfix(self.read(addr::REG_OOKFIX));
        self.write(addr::REG_OOKFIX, f(tmp).0)
    }

    pub fn afcfei(&self) -> types::Afcfei {
        types::Afcfei(self.read(addr::REG_AFCFEI))
    }
    pub fn set_afcfei(&self, value: types::Afcfei) {
        self.write(addr::REG_AFCFEI, value.0)
    }
    pub fn with_afcfei<F: FnOnce(types::Afcfei) -> types::Afcfei>(&self, f: F) {
        let tmp = types::Afcfei(self.read(addr::REG_AFCFEI));
        self.write(addr::REG_AFCFEI, f(tmp).0)
    }

    pub fn afcmsb(&self) -> types::Afcmsb {
        types::Afcmsb(self.read(addr::REG_AFCMSB))
    }
    pub fn set_afcmsb(&self, value: types::Afcmsb) {
        self.write(addr::REG_AFCMSB, value.0)
    }
    pub fn with_afcmsb<F: FnOnce(types::Afcmsb) -> types::Afcmsb>(&self, f: F) {
        let tmp = types::Afcmsb(self.read(addr::REG_AFCMSB));
        self.write(addr::REG_AFCMSB, f(tmp).0)
    }

    pub fn afclsb(&self) -> types::Afclsb {
        types::Afclsb(self.read(addr::REG_AFCLSB))
    }
    pub fn set_afclsb(&self, value: types::Afclsb) {
        self.write(addr::REG_AFCLSB, value.0)
    }
    pub fn with_afclsb<F: FnOnce(types::Afclsb) -> types::Afclsb>(&self, f: F) {
        let tmp = types::Afclsb(self.read(addr::REG_AFCLSB));
        self.write(addr::REG_AFCLSB, f(tmp).0)
    }

    pub fn feimsb(&self) -> types::Feimsb {
        types::Feimsb(self.read(addr::REG_FEIMSB))
    }
    pub fn set_feimsb(&self, value: types::Feimsb) {
        self.write(addr::REG_FEIMSB, value.0)
    }
    pub fn with_feimsb<F: FnOnce(types::Feimsb) -> types::Feimsb>(&self, f: F) {
        let tmp = types::Feimsb(self.read(addr::REG_FEIMSB));
        self.write(addr::REG_FEIMSB, f(tmp).0)
    }

    pub fn feilsb(&self) -> types::Feilsb {
        types::Feilsb(self.read(addr::REG_FEILSB))
    }
    pub fn set_feilsb(&self, value: types::Feilsb) {
        self.write(addr::REG_FEILSB, value.0)
    }
    pub fn with_feilsb<F: FnOnce(types::Feilsb) -> types::Feilsb>(&self, f: F) {
        let tmp = types::Feilsb(self.read(addr::REG_FEILSB));
        self.write(addr::REG_FEILSB, f(tmp).0)
    }

    pub fn rssiconfig(&self) -> types::Rssiconfig {
        types::Rssiconfig(self.read(addr::REG_RSSICONFIG))
    }
    pub fn set_rssiconfig(&self, value: types::Rssiconfig) {
        self.write(addr::REG_RSSICONFIG, value.0)
    }
    pub fn with_rssiconfig<F: FnOnce(types::Rssiconfig) -> types::Rssiconfig>(&self, f: F) {
        let tmp = types::Rssiconfig(self.read(addr::REG_RSSICONFIG));
        self.write(addr::REG_RSSICONFIG, f(tmp).0)
    }

    pub fn rssivalue(&self) -> types::Rssivalue {
        types::Rssivalue(self.read(addr::REG_RSSIVALUE))
    }
    pub fn set_rssivalue(&self, value: types::Rssivalue) {
        self.write(addr::REG_RSSIVALUE, value.0)
    }
    pub fn with_rssivalue<F: FnOnce(types::Rssivalue) -> types::Rssivalue>(&self, f: F) {
        let tmp = types::Rssivalue(self.read(addr::REG_RSSIVALUE));
        self.write(addr::REG_RSSIVALUE, f(tmp).0)
    }

    pub fn diomapping1(&self) -> types::Diomapping1 {
        types::Diomapping1(self.read(addr::REG_DIOMAPPING1))
    }
    pub fn set_diomapping1(&self, value: types::Diomapping1) {
        self.write(addr::REG_DIOMAPPING1, value.0)
    }
    pub fn with_diomapping1<F: FnOnce(types::Diomapping1) -> types::Diomapping1>(&self, f: F) {
        let tmp = types::Diomapping1(self.read(addr::REG_DIOMAPPING1));
        self.write(addr::REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn diomapping2(&self) -> types::Diomapping2 {
        types::Diomapping2(self.read(addr::REG_DIOMAPPING2))
    }
    pub fn set_diomapping2(&self, value: types::Diomapping2) {
        self.write(addr::REG_DIOMAPPING2, value.0)
    }
    pub fn with_diomapping2<F: FnOnce(types::Diomapping2) -> types::Diomapping2>(&self, f: F) {
        let tmp = types::Diomapping2(self.read(addr::REG_DIOMAPPING2));
        self.write(addr::REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn irqflags1(&self) -> types::Irqflags1 {
        types::Irqflags1(self.read(addr::REG_IRQFLAGS1))
    }
    pub fn set_irqflags1(&self, value: types::Irqflags1) {
        self.write(addr::REG_IRQFLAGS1, value.0)
    }
    pub fn with_irqflags1<F: FnOnce(types::Irqflags1) -> types::Irqflags1>(&self, f: F) {
        let tmp = types::Irqflags1(self.read(addr::REG_IRQFLAGS1));
        self.write(addr::REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn irqflags2(&self) -> types::Irqflags2 {
        types::Irqflags2(self.read(addr::REG_IRQFLAGS2))
    }
    pub fn set_irqflags2(&self, value: types::Irqflags2) {
        self.write(addr::REG_IRQFLAGS2, value.0)
    }
    pub fn with_irqflags2<F: FnOnce(types::Irqflags2) -> types::Irqflags2>(&self, f: F) {
        let tmp = types::Irqflags2(self.read(addr::REG_IRQFLAGS2));
        self.write(addr::REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn rssithresh(&self) -> types::Rssithresh {
        types::Rssithresh(self.read(addr::REG_RSSITHRESH))
    }
    pub fn set_rssithresh(&self, value: types::Rssithresh) {
        self.write(addr::REG_RSSITHRESH, value.0)
    }
    pub fn with_rssithresh<F: FnOnce(types::Rssithresh) -> types::Rssithresh>(&self, f: F) {
        let tmp = types::Rssithresh(self.read(addr::REG_RSSITHRESH));
        self.write(addr::REG_RSSITHRESH, f(tmp).0)
    }

    pub fn rxtimeout1(&self) -> types::Rxtimeout1 {
        types::Rxtimeout1(self.read(addr::REG_RXTIMEOUT1))
    }
    pub fn set_rxtimeout1(&self, value: types::Rxtimeout1) {
        self.write(addr::REG_RXTIMEOUT1, value.0)
    }
    pub fn with_rxtimeout1<F: FnOnce(types::Rxtimeout1) -> types::Rxtimeout1>(&self, f: F) {
        let tmp = types::Rxtimeout1(self.read(addr::REG_RXTIMEOUT1));
        self.write(addr::REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn rxtimeout2(&self) -> types::Rxtimeout2 {
        types::Rxtimeout2(self.read(addr::REG_RXTIMEOUT2))
    }
    pub fn set_rxtimeout2(&self, value: types::Rxtimeout2) {
        self.write(addr::REG_RXTIMEOUT2, value.0)
    }
    pub fn with_rxtimeout2<F: FnOnce(types::Rxtimeout2) -> types::Rxtimeout2>(&self, f: F) {
        let tmp = types::Rxtimeout2(self.read(addr::REG_RXTIMEOUT2));
        self.write(addr::REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn preamblemsb(&self) -> types::Preamblemsb {
        types::Preamblemsb(self.read(addr::REG_PREAMBLEMSB))
    }
    pub fn set_preamblemsb(&self, value: types::Preamblemsb) {
        self.write(addr::REG_PREAMBLEMSB, value.0)
    }
    pub fn with_preamblemsb<F: FnOnce(types::Preamblemsb) -> types::Preamblemsb>(&self, f: F) {
        let tmp = types::Preamblemsb(self.read(addr::REG_PREAMBLEMSB));
        self.write(addr::REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn preamblelsb(&self) -> types::Preamblelsb {
        types::Preamblelsb(self.read(addr::REG_PREAMBLELSB))
    }
    pub fn set_preamblelsb(&self, value: types::Preamblelsb) {
        self.write(addr::REG_PREAMBLELSB, value.0)
    }
    pub fn with_preamblelsb<F: FnOnce(types::Preamblelsb) -> types::Preamblelsb>(&self, f: F) {
        let tmp = types::Preamblelsb(self.read(addr::REG_PREAMBLELSB));
        self.write(addr::REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn syncconfig(&self) -> types::Syncconfig {
        types::Syncconfig(self.read(addr::REG_SYNCCONFIG))
    }
    pub fn set_syncconfig(&self, value: types::Syncconfig) {
        self.write(addr::REG_SYNCCONFIG, value.0)
    }
    pub fn with_syncconfig<F: FnOnce(types::Syncconfig) -> types::Syncconfig>(&self, f: F) {
        let tmp = types::Syncconfig(self.read(addr::REG_SYNCCONFIG));
        self.write(addr::REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn syncvalue(&self, index: usize) -> types::Syncvalue {
        assert!(index < 8);
        types::Syncvalue(self.read(addr::REG_SYNCVALUE + index as u8))
    }
    pub fn set_syncvalue(&self, index: usize, value: types::Syncvalue) {
        assert!(index < 8);
        self.write(addr::REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn with_syncvalue<F: FnOnce(types::Syncvalue) -> types::Syncvalue>(&self, index: usize, f: F) {
        assert!(index < 8);
        let tmp = types::Syncvalue(self.read(addr::REG_SYNCVALUE + index as u8));
        self.write(addr::REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn packetconfig1(&self) -> types::Packetconfig1 {
        types::Packetconfig1(self.read(addr::REG_PACKETCONFIG1))
    }
    pub fn set_packetconfig1(&self, value: types::Packetconfig1) {
        self.write(addr::REG_PACKETCONFIG1, value.0)
    }
    pub fn with_packetconfig1<F: FnOnce(types::Packetconfig1) -> types::Packetconfig1>(&self, f: F) {
        let tmp = types::Packetconfig1(self.read(addr::REG_PACKETCONFIG1));
        self.write(addr::REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn payloadlength(&self) -> types::Payloadlength {
        types::Payloadlength(self.read(addr::REG_PAYLOADLENGTH))
    }
    pub fn set_payloadlength(&self, value: types::Payloadlength) {
        self.write(addr::REG_PAYLOADLENGTH, value.0)
    }
    pub fn with_payloadlength<F: FnOnce(types::Payloadlength) -> types::Payloadlength>(&self, f: F) {
        let tmp = types::Payloadlength(self.read(addr::REG_PAYLOADLENGTH));
        self.write(addr::REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn nodeadrs(&self) -> types::Nodeadrs {
        types::Nodeadrs(self.read(addr::REG_NODEADRS))
    }
    pub fn set_nodeadrs(&self, value: types::Nodeadrs) {
        self.write(addr::REG_NODEADRS, value.0)
    }
    pub fn with_nodeadrs<F: FnOnce(types::Nodeadrs) -> types::Nodeadrs>(&self, f: F) {
        let tmp = types::Nodeadrs(self.read(addr::REG_NODEADRS));
        self.write(addr::REG_NODEADRS, f(tmp).0)
    }

    pub fn broadcastadrs(&self) -> types::Broadcastadrs {
        types::Broadcastadrs(self.read(addr::REG_BROADCASTADRS))
    }
    pub fn set_broadcastadrs(&self, value: types::Broadcastadrs) {
        self.write(addr::REG_BROADCASTADRS, value.0)
    }
    pub fn with_broadcastadrs<F: FnOnce(types::Broadcastadrs) -> types::Broadcastadrs>(&self, f: F) {
        let tmp = types::Broadcastadrs(self.read(addr::REG_BROADCASTADRS));
        self.write(addr::REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn automodes(&self) -> types::Automodes {
        types::Automodes(self.read(addr::REG_AUTOMODES))
    }
    pub fn set_automodes(&self, value: types::Automodes) {
        self.write(addr::REG_AUTOMODES, value.0)
    }
    pub fn with_automodes<F: FnOnce(types::Automodes) -> types::Automodes>(&self, f: F) {
        let tmp = types::Automodes(self.read(addr::REG_AUTOMODES));
        self.write(addr::REG_AUTOMODES, f(tmp).0)
    }

    pub fn fifothresh(&self) -> types::Fifothresh {
        types::Fifothresh(self.read(addr::REG_FIFOTHRESH))
    }
    pub fn set_fifothresh(&self, value: types::Fifothresh) {
        self.write(addr::REG_FIFOTHRESH, value.0)
    }
    pub fn with_fifothresh<F: FnOnce(types::Fifothresh) -> types::Fifothresh>(&self, f: F) {
        let tmp = types::Fifothresh(self.read(addr::REG_FIFOTHRESH));
        self.write(addr::REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn packetconfig2(&self) -> types::Packetconfig2 {
        types::Packetconfig2(self.read(addr::REG_PACKETCONFIG2))
    }
    pub fn set_packetconfig2(&self, value: types::Packetconfig2) {
        self.write(addr::REG_PACKETCONFIG2, value.0)
    }
    pub fn with_packetconfig2<F: FnOnce(types::Packetconfig2) -> types::Packetconfig2>(&self, f: F) {
        let tmp = types::Packetconfig2(self.read(addr::REG_PACKETCONFIG2));
        self.write(addr::REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn aeskey(&self, index: usize) -> types::Aeskey {
        assert!(index < 16);
        types::Aeskey(self.read(addr::REG_AESKEY + index as u8))
    }
    pub fn set_aeskey(&self, index: usize, value: types::Aeskey) {
        assert!(index < 16);
        self.write(addr::REG_AESKEY + index as u8, value.0)
    }
    pub fn with_aeskey<F: FnOnce(types::Aeskey) -> types::Aeskey>(&self, index: usize, f: F) {
        assert!(index < 16);
        let tmp = types::Aeskey(self.read(addr::REG_AESKEY + index as u8));
        self.write(addr::REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn temp1(&self) -> types::Temp1 {
        types::Temp1(self.read(addr::REG_TEMP1))
    }
    pub fn set_temp1(&self, value: types::Temp1) {
        self.write(addr::REG_TEMP1, value.0)
    }
    pub fn with_temp1<F: FnOnce(types::Temp1) -> types::Temp1>(&self, f: F) {
        let tmp = types::Temp1(self.read(addr::REG_TEMP1));
        self.write(addr::REG_TEMP1, f(tmp).0)
    }

    pub fn temp2(&self) -> types::Temp2 {
        types::Temp2(self.read(addr::REG_TEMP2))
    }
    pub fn set_temp2(&self, value: types::Temp2) {
        self.write(addr::REG_TEMP2, value.0)
    }
    pub fn with_temp2<F: FnOnce(types::Temp2) -> types::Temp2>(&self, f: F) {
        let tmp = types::Temp2(self.read(addr::REG_TEMP2));
        self.write(addr::REG_TEMP2, f(tmp).0)
    }

    pub fn testlna(&self) -> types::Testlna {
        types::Testlna(self.read(addr::REG_TESTLNA))
    }
    pub fn set_testlna(&self, value: types::Testlna) {
        self.write(addr::REG_TESTLNA, value.0)
    }
    pub fn with_testlna<F: FnOnce(types::Testlna) -> types::Testlna>(&self, f: F) {
        let tmp = types::Testlna(self.read(addr::REG_TESTLNA));
        self.write(addr::REG_TESTLNA, f(tmp).0)
    }

    pub fn testpa1(&self) -> types::Testpa1 {
        types::Testpa1(self.read(addr::REG_TESTPA1))
    }
    pub fn set_testpa1(&self, value: types::Testpa1) {
        self.write(addr::REG_TESTPA1, value.0)
    }
    pub fn with_testpa1<F: FnOnce(types::Testpa1) -> types::Testpa1>(&self, f: F) {
        let tmp = types::Testpa1(self.read(addr::REG_TESTPA1));
        self.write(addr::REG_TESTPA1, f(tmp).0)
    }

    pub fn testpa2(&self) -> types::Testpa2 {
        types::Testpa2(self.read(addr::REG_TESTPA2))
    }
    pub fn set_testpa2(&self, value: types::Testpa2) {
        self.write(addr::REG_TESTPA2, value.0)
    }
    pub fn with_testpa2<F: FnOnce(types::Testpa2) -> types::Testpa2>(&self, f: F) {
        let tmp = types::Testpa2(self.read(addr::REG_TESTPA2));
        self.write(addr::REG_TESTPA2, f(tmp).0)
    }

    pub fn testdagc(&self) -> types::Testdagc {
        types::Testdagc(self.read(addr::REG_TESTDAGC))
    }
    pub fn set_testdagc(&self, value: types::Testdagc) {
        self.write(addr::REG_TESTDAGC, value.0)
    }
    pub fn with_testdagc<F: FnOnce(types::Testdagc) -> types::Testdagc>(&self, f: F) {
        let tmp = types::Testdagc(self.read(addr::REG_TESTDAGC));
        self.write(addr::REG_TESTDAGC, f(tmp).0)
    }

    pub fn testafc(&self) -> types::Testafc {
        types::Testafc(self.read(addr::REG_TESTAFC))
    }
    pub fn set_testafc(&self, value: types::Testafc) {
        self.write(addr::REG_TESTAFC, value.0)
    }
    pub fn with_testafc<F: FnOnce(types::Testafc) -> types::Testafc>(&self, f: F) {
        let tmp = types::Testafc(self.read(addr::REG_TESTAFC));
        self.write(addr::REG_TESTAFC, f(tmp).0)
    }

}

impl<RW: TryReadWrite> Rfm69<RW> {
    pub fn try_fifo(&self) -> Result<types::Fifo, RW::Error> {
        Ok(types::Fifo(self.try_read(addr::REG_FIFO)?))
    }
    pub fn try_set_fifo(&self, value: types::Fifo) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FIFO, value.0)
    }
    pub fn try_with_fifo<F: FnOnce(types::Fifo) -> types::Fifo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Fifo(self.try_read(addr::REG_FIFO)?);
        self.try_write(addr::REG_FIFO, f(tmp).0)
    }

    pub fn try_opmode(&self) -> Result<types::Opmode, RW::Error> {
        Ok(types::Opmode(self.try_read(addr::REG_OPMODE)?))
    }
    pub fn try_set_opmode(&self, value: types::Opmode) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OPMODE, value.0)
    }
    pub fn try_with_opmode<F: FnOnce(types::Opmode) -> types::Opmode>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Opmode(self.try_read(addr::REG_OPMODE)?);
        self.try_write(addr::REG_OPMODE, f(tmp).0)
    }

    pub fn try_datamodul(&self) -> Result<types::Datamodul, RW::Error> {
        Ok(types::Datamodul(self.try_read(addr::REG_DATAMODUL)?))
    }
    pub fn try_set_datamodul(&self, value: types::Datamodul) -> Result<(), RW::Error> {
        self.try_write(addr::REG_DATAMODUL, value.0)
    }
    pub fn try_with_datamodul<F: FnOnce(types::Datamodul) -> types::Datamodul>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Datamodul(self.try_read(addr::REG_DATAMODUL)?);
        self.try_write(addr::REG_DATAMODUL, f(tmp).0)
    }

    pub fn try_bitratemsb(&self) -> Result<types::Bitratemsb, RW::Error> {
        Ok(types::Bitratemsb(self.try_read(addr::REG_BITRATEMSB)?))
    }
    pub fn try_set_bitratemsb(&self, value: types::Bitratemsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_BITRATEMSB, value.0)
    }
    pub fn try_with_bitratemsb<F: FnOnce(types::Bitratemsb) -> types::Bitratemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Bitratemsb(self.try_read(addr::REG_BITRATEMSB)?);
        self.try_write(addr::REG_BITRATEMSB, f(tmp).0)
    }

    pub fn try_bitratelsb(&self) -> Result<types::Bitratelsb, RW::Error> {
        Ok(types::Bitratelsb(self.try_read(addr::REG_BITRATELSB)?))
    }
    pub fn try_set_bitratelsb(&self, value: types::Bitratelsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_BITRATELSB, value.0)
    }
    pub fn try_with_bitratelsb<F: FnOnce(types::Bitratelsb) -> types::Bitratelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Bitratelsb(self.try_read(addr::REG_BITRATELSB)?);
        self.try_write(addr::REG_BITRATELSB, f(tmp).0)
    }

    pub fn try_fdevmsb(&self) -> Result<types::Fdevmsb, RW::Error> {
        Ok(types::Fdevmsb(self.try_read(addr::REG_FDEVMSB)?))
    }
    pub fn try_set_fdevmsb(&self, value: types::Fdevmsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FDEVMSB, value.0)
    }
    pub fn try_with_fdevmsb<F: FnOnce(types::Fdevmsb) -> types::Fdevmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Fdevmsb(self.try_read(addr::REG_FDEVMSB)?);
        self.try_write(addr::REG_FDEVMSB, f(tmp).0)
    }

    pub fn try_fdevlsb(&self) -> Result<types::Fdevlsb, RW::Error> {
        Ok(types::Fdevlsb(self.try_read(addr::REG_FDEVLSB)?))
    }
    pub fn try_set_fdevlsb(&self, value: types::Fdevlsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FDEVLSB, value.0)
    }
    pub fn try_with_fdevlsb<F: FnOnce(types::Fdevlsb) -> types::Fdevlsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Fdevlsb(self.try_read(addr::REG_FDEVLSB)?);
        self.try_write(addr::REG_FDEVLSB, f(tmp).0)
    }

    pub fn try_frfmsb(&self) -> Result<types::Frfmsb, RW::Error> {
        Ok(types::Frfmsb(self.try_read(addr::REG_FRFMSB)?))
    }
    pub fn try_set_frfmsb(&self, value: types::Frfmsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FRFMSB, value.0)
    }
    pub fn try_with_frfmsb<F: FnOnce(types::Frfmsb) -> types::Frfmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Frfmsb(self.try_read(addr::REG_FRFMSB)?);
        self.try_write(addr::REG_FRFMSB, f(tmp).0)
    }

    pub fn try_frfmid(&self) -> Result<types::Frfmid, RW::Error> {
        Ok(types::Frfmid(self.try_read(addr::REG_FRFMID)?))
    }
    pub fn try_set_frfmid(&self, value: types::Frfmid) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FRFMID, value.0)
    }
    pub fn try_with_frfmid<F: FnOnce(types::Frfmid) -> types::Frfmid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Frfmid(self.try_read(addr::REG_FRFMID)?);
        self.try_write(addr::REG_FRFMID, f(tmp).0)
    }

    pub fn try_frflsb(&self) -> Result<types::Frflsb, RW::Error> {
        Ok(types::Frflsb(self.try_read(addr::REG_FRFLSB)?))
    }
    pub fn try_set_frflsb(&self, value: types::Frflsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FRFLSB, value.0)
    }
    pub fn try_with_frflsb<F: FnOnce(types::Frflsb) -> types::Frflsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Frflsb(self.try_read(addr::REG_FRFLSB)?);
        self.try_write(addr::REG_FRFLSB, f(tmp).0)
    }

    pub fn try_osc1(&self) -> Result<types::Osc1, RW::Error> {
        Ok(types::Osc1(self.try_read(addr::REG_OSC1)?))
    }
    pub fn try_set_osc1(&self, value: types::Osc1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OSC1, value.0)
    }
    pub fn try_with_osc1<F: FnOnce(types::Osc1) -> types::Osc1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Osc1(self.try_read(addr::REG_OSC1)?);
        self.try_write(addr::REG_OSC1, f(tmp).0)
    }

    pub fn try_afcctrl(&self) -> Result<types::Afcctrl, RW::Error> {
        Ok(types::Afcctrl(self.try_read(addr::REG_AFCCTRL)?))
    }
    pub fn try_set_afcctrl(&self, value: types::Afcctrl) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AFCCTRL, value.0)
    }
    pub fn try_with_afcctrl<F: FnOnce(types::Afcctrl) -> types::Afcctrl>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Afcctrl(self.try_read(addr::REG_AFCCTRL)?);
        self.try_write(addr::REG_AFCCTRL, f(tmp).0)
    }

    pub fn try_listen1(&self) -> Result<types::Listen1, RW::Error> {
        Ok(types::Listen1(self.try_read(addr::REG_LISTEN1)?))
    }
    pub fn try_set_listen1(&self, value: types::Listen1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_LISTEN1, value.0)
    }
    pub fn try_with_listen1<F: FnOnce(types::Listen1) -> types::Listen1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Listen1(self.try_read(addr::REG_LISTEN1)?);
        self.try_write(addr::REG_LISTEN1, f(tmp).0)
    }

    pub fn try_listen2(&self) -> Result<types::Listen2, RW::Error> {
        Ok(types::Listen2(self.try_read(addr::REG_LISTEN2)?))
    }
    pub fn try_set_listen2(&self, value: types::Listen2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_LISTEN2, value.0)
    }
    pub fn try_with_listen2<F: FnOnce(types::Listen2) -> types::Listen2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Listen2(self.try_read(addr::REG_LISTEN2)?);
        self.try_write(addr::REG_LISTEN2, f(tmp).0)
    }

    pub fn try_listen3(&self) -> Result<types::Listen3, RW::Error> {
        Ok(types::Listen3(self.try_read(addr::REG_LISTEN3)?))
    }
    pub fn try_set_listen3(&self, value: types::Listen3) -> Result<(), RW::Error> {
        self.try_write(addr::REG_LISTEN3, value.0)
    }
    pub fn try_with_listen3<F: FnOnce(types::Listen3) -> types::Listen3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Listen3(self.try_read(addr::REG_LISTEN3)?);
        self.try_write(addr::REG_LISTEN3, f(tmp).0)
    }

    pub fn try_version(&self) -> Result<types::Version, RW::Error> {
        Ok(types::Version(self.try_read(addr::REG_VERSION)?))
    }
    pub fn try_set_version(&self, value: types::Version) -> Result<(), RW::Error> {
        self.try_write(addr::REG_VERSION, value.0)
    }
    pub fn try_with_version<F: FnOnce(types::Version) -> types::Version>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Version(self.try_read(addr::REG_VERSION)?);
        self.try_write(addr::REG_VERSION, f(tmp).0)
    }

    pub fn try_palevel(&self) -> Result<types::Palevel, RW::Error> {
        Ok(types::Palevel(self.try_read(addr::REG_PALEVEL)?))
    }
    pub fn try_set_palevel(&self, value: types::Palevel) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PALEVEL, value.0)
    }
    pub fn try_with_palevel<F: FnOnce(types::Palevel) -> types::Palevel>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Palevel(self.try_read(addr::REG_PALEVEL)?);
        self.try_write(addr::REG_PALEVEL, f(tmp).0)
    }

    pub fn try_paramp(&self) -> Result<types::Paramp, RW::Error> {
        Ok(types::Paramp(self.try_read(addr::REG_PARAMP)?))
    }
    pub fn try_set_paramp(&self, value: types::Paramp) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PARAMP, value.0)
    }
    pub fn try_with_paramp<F: FnOnce(types::Paramp) -> types::Paramp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Paramp(self.try_read(addr::REG_PARAMP)?);
        self.try_write(addr::REG_PARAMP, f(tmp).0)
    }

    pub fn try_ocp(&self) -> Result<types::Ocp, RW::Error> {
        Ok(types::Ocp(self.try_read(addr::REG_OCP)?))
    }
    pub fn try_set_ocp(&self, value: types::Ocp) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OCP, value.0)
    }
    pub fn try_with_ocp<F: FnOnce(types::Ocp) -> types::Ocp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Ocp(self.try_read(addr::REG_OCP)?);
        self.try_write(addr::REG_OCP, f(tmp).0)
    }

    pub fn try_lna(&self) -> Result<types::Lna, RW::Error> {
        Ok(types::Lna(self.try_read(addr::REG_LNA)?))
    }
    pub fn try_set_lna(&self, value: types::Lna) -> Result<(), RW::Error> {
        self.try_write(addr::REG_LNA, value.0)
    }
    pub fn try_with_lna<F: FnOnce(types::Lna) -> types::Lna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Lna(self.try_read(addr::REG_LNA)?);
        self.try_write(addr::REG_LNA, f(tmp).0)
    }

    pub fn try_rxbw(&self) -> Result<types::Rxbw, RW::Error> {
        Ok(types::Rxbw(self.try_read(addr::REG_RXBW)?))
    }
    pub fn try_set_rxbw(&self, value: types::Rxbw) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RXBW, value.0)
    }
    pub fn try_with_rxbw<F: FnOnce(types::Rxbw) -> types::Rxbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rxbw(self.try_read(addr::REG_RXBW)?);
        self.try_write(addr::REG_RXBW, f(tmp).0)
    }

    pub fn try_afcbw(&self) -> Result<types::Afcbw, RW::Error> {
        Ok(types::Afcbw(self.try_read(addr::REG_AFCBW)?))
    }
    pub fn try_set_afcbw(&self, value: types::Afcbw) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AFCBW, value.0)
    }
    pub fn try_with_afcbw<F: FnOnce(types::Afcbw) -> types::Afcbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Afcbw(self.try_read(addr::REG_AFCBW)?);
        self.try_write(addr::REG_AFCBW, f(tmp).0)
    }

    pub fn try_ookpeak(&self) -> Result<types::Ookpeak, RW::Error> {
        Ok(types::Ookpeak(self.try_read(addr::REG_OOKPEAK)?))
    }
    pub fn try_set_ookpeak(&self, value: types::Ookpeak) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OOKPEAK, value.0)
    }
    pub fn try_with_ookpeak<F: FnOnce(types::Ookpeak) -> types::Ookpeak>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Ookpeak(self.try_read(addr::REG_OOKPEAK)?);
        self.try_write(addr::REG_OOKPEAK, f(tmp).0)
    }

    pub fn try_ookavg(&self) -> Result<types::Ookavg, RW::Error> {
        Ok(types::Ookavg(self.try_read(addr::REG_OOKAVG)?))
    }
    pub fn try_set_ookavg(&self, value: types::Ookavg) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OOKAVG, value.0)
    }
    pub fn try_with_ookavg<F: FnOnce(types::Ookavg) -> types::Ookavg>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Ookavg(self.try_read(addr::REG_OOKAVG)?);
        self.try_write(addr::REG_OOKAVG, f(tmp).0)
    }

    pub fn try_ookfix(&self) -> Result<types::Ookfix, RW::Error> {
        Ok(types::Ookfix(self.try_read(addr::REG_OOKFIX)?))
    }
    pub fn try_set_ookfix(&self, value: types::Ookfix) -> Result<(), RW::Error> {
        self.try_write(addr::REG_OOKFIX, value.0)
    }
    pub fn try_with_ookfix<F: FnOnce(types::Ookfix) -> types::Ookfix>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Ookfix(self.try_read(addr::REG_OOKFIX)?);
        self.try_write(addr::REG_OOKFIX, f(tmp).0)
    }

    pub fn try_afcfei(&self) -> Result<types::Afcfei, RW::Error> {
        Ok(types::Afcfei(self.try_read(addr::REG_AFCFEI)?))
    }
    pub fn try_set_afcfei(&self, value: types::Afcfei) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AFCFEI, value.0)
    }
    pub fn try_with_afcfei<F: FnOnce(types::Afcfei) -> types::Afcfei>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Afcfei(self.try_read(addr::REG_AFCFEI)?);
        self.try_write(addr::REG_AFCFEI, f(tmp).0)
    }

    pub fn try_afcmsb(&self) -> Result<types::Afcmsb, RW::Error> {
        Ok(types::Afcmsb(self.try_read(addr::REG_AFCMSB)?))
    }
    pub fn try_set_afcmsb(&self, value: types::Afcmsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AFCMSB, value.0)
    }
    pub fn try_with_afcmsb<F: FnOnce(types::Afcmsb) -> types::Afcmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Afcmsb(self.try_read(addr::REG_AFCMSB)?);
        self.try_write(addr::REG_AFCMSB, f(tmp).0)
    }

    pub fn try_afclsb(&self) -> Result<types::Afclsb, RW::Error> {
        Ok(types::Afclsb(self.try_read(addr::REG_AFCLSB)?))
    }
    pub fn try_set_afclsb(&self, value: types::Afclsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AFCLSB, value.0)
    }
    pub fn try_with_afclsb<F: FnOnce(types::Afclsb) -> types::Afclsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Afclsb(self.try_read(addr::REG_AFCLSB)?);
        self.try_write(addr::REG_AFCLSB, f(tmp).0)
    }

    pub fn try_feimsb(&self) -> Result<types::Feimsb, RW::Error> {
        Ok(types::Feimsb(self.try_read(addr::REG_FEIMSB)?))
    }
    pub fn try_set_feimsb(&self, value: types::Feimsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FEIMSB, value.0)
    }
    pub fn try_with_feimsb<F: FnOnce(types::Feimsb) -> types::Feimsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Feimsb(self.try_read(addr::REG_FEIMSB)?);
        self.try_write(addr::REG_FEIMSB, f(tmp).0)
    }

    pub fn try_feilsb(&self) -> Result<types::Feilsb, RW::Error> {
        Ok(types::Feilsb(self.try_read(addr::REG_FEILSB)?))
    }
    pub fn try_set_feilsb(&self, value: types::Feilsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FEILSB, value.0)
    }
    pub fn try_with_feilsb<F: FnOnce(types::Feilsb) -> types::Feilsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Feilsb(self.try_read(addr::REG_FEILSB)?);
        self.try_write(addr::REG_FEILSB, f(tmp).0)
    }

    pub fn try_rssiconfig(&self) -> Result<types::Rssiconfig, RW::Error> {
        Ok(types::Rssiconfig(self.try_read(addr::REG_RSSICONFIG)?))
    }
    pub fn try_set_rssiconfig(&self, value: types::Rssiconfig) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RSSICONFIG, value.0)
    }
    pub fn try_with_rssiconfig<F: FnOnce(types::Rssiconfig) -> types::Rssiconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rssiconfig(self.try_read(addr::REG_RSSICONFIG)?);
        self.try_write(addr::REG_RSSICONFIG, f(tmp).0)
    }

    pub fn try_rssivalue(&self) -> Result<types::Rssivalue, RW::Error> {
        Ok(types::Rssivalue(self.try_read(addr::REG_RSSIVALUE)?))
    }
    pub fn try_set_rssivalue(&self, value: types::Rssivalue) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RSSIVALUE, value.0)
    }
    pub fn try_with_rssivalue<F: FnOnce(types::Rssivalue) -> types::Rssivalue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rssivalue(self.try_read(addr::REG_RSSIVALUE)?);
        self.try_write(addr::REG_RSSIVALUE, f(tmp).0)
    }

    pub fn try_diomapping1(&self) -> Result<types::Diomapping1, RW::Error> {
        Ok(types::Diomapping1(self.try_read(addr::REG_DIOMAPPING1)?))
    }
    pub fn try_set_diomapping1(&self, value: types::Diomapping1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_DIOMAPPING1, value.0)
    }
    pub fn try_with_diomapping1<F: FnOnce(types::Diomapping1) -> types::Diomapping1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Diomapping1(self.try_read(addr::REG_DIOMAPPING1)?);
        self.try_write(addr::REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn try_diomapping2(&self) -> Result<types::Diomapping2, RW::Error> {
        Ok(types::Diomapping2(self.try_read(addr::REG_DIOMAPPING2)?))
    }
    pub fn try_set_diomapping2(&self, value: types::Diomapping2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_DIOMAPPING2, value.0)
    }
    pub fn try_with_diomapping2<F: FnOnce(types::Diomapping2) -> types::Diomapping2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Diomapping2(self.try_read(addr::REG_DIOMAPPING2)?);
        self.try_write(addr::REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn try_irqflags1(&self) -> Result<types::Irqflags1, RW::Error> {
        Ok(types::Irqflags1(self.try_read(addr::REG_IRQFLAGS1)?))
    }
    pub fn try_set_irqflags1(&self, value: types::Irqflags1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_IRQFLAGS1, value.0)
    }
    pub fn try_with_irqflags1<F: FnOnce(types::Irqflags1) -> types::Irqflags1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Irqflags1(self.try_read(addr::REG_IRQFLAGS1)?);
        self.try_write(addr::REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn try_irqflags2(&self) -> Result<types::Irqflags2, RW::Error> {
        Ok(types::Irqflags2(self.try_read(addr::REG_IRQFLAGS2)?))
    }
    pub fn try_set_irqflags2(&self, value: types::Irqflags2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_IRQFLAGS2, value.0)
    }
    pub fn try_with_irqflags2<F: FnOnce(types::Irqflags2) -> types::Irqflags2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Irqflags2(self.try_read(addr::REG_IRQFLAGS2)?);
        self.try_write(addr::REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn try_rssithresh(&self) -> Result<types::Rssithresh, RW::Error> {
        Ok(types::Rssithresh(self.try_read(addr::REG_RSSITHRESH)?))
    }
    pub fn try_set_rssithresh(&self, value: types::Rssithresh) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RSSITHRESH, value.0)
    }
    pub fn try_with_rssithresh<F: FnOnce(types::Rssithresh) -> types::Rssithresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rssithresh(self.try_read(addr::REG_RSSITHRESH)?);
        self.try_write(addr::REG_RSSITHRESH, f(tmp).0)
    }

    pub fn try_rxtimeout1(&self) -> Result<types::Rxtimeout1, RW::Error> {
        Ok(types::Rxtimeout1(self.try_read(addr::REG_RXTIMEOUT1)?))
    }
    pub fn try_set_rxtimeout1(&self, value: types::Rxtimeout1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RXTIMEOUT1, value.0)
    }
    pub fn try_with_rxtimeout1<F: FnOnce(types::Rxtimeout1) -> types::Rxtimeout1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rxtimeout1(self.try_read(addr::REG_RXTIMEOUT1)?);
        self.try_write(addr::REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn try_rxtimeout2(&self) -> Result<types::Rxtimeout2, RW::Error> {
        Ok(types::Rxtimeout2(self.try_read(addr::REG_RXTIMEOUT2)?))
    }
    pub fn try_set_rxtimeout2(&self, value: types::Rxtimeout2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_RXTIMEOUT2, value.0)
    }
    pub fn try_with_rxtimeout2<F: FnOnce(types::Rxtimeout2) -> types::Rxtimeout2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Rxtimeout2(self.try_read(addr::REG_RXTIMEOUT2)?);
        self.try_write(addr::REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn try_preamblemsb(&self) -> Result<types::Preamblemsb, RW::Error> {
        Ok(types::Preamblemsb(self.try_read(addr::REG_PREAMBLEMSB)?))
    }
    pub fn try_set_preamblemsb(&self, value: types::Preamblemsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PREAMBLEMSB, value.0)
    }
    pub fn try_with_preamblemsb<F: FnOnce(types::Preamblemsb) -> types::Preamblemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Preamblemsb(self.try_read(addr::REG_PREAMBLEMSB)?);
        self.try_write(addr::REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn try_preamblelsb(&self) -> Result<types::Preamblelsb, RW::Error> {
        Ok(types::Preamblelsb(self.try_read(addr::REG_PREAMBLELSB)?))
    }
    pub fn try_set_preamblelsb(&self, value: types::Preamblelsb) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PREAMBLELSB, value.0)
    }
    pub fn try_with_preamblelsb<F: FnOnce(types::Preamblelsb) -> types::Preamblelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Preamblelsb(self.try_read(addr::REG_PREAMBLELSB)?);
        self.try_write(addr::REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn try_syncconfig(&self) -> Result<types::Syncconfig, RW::Error> {
        Ok(types::Syncconfig(self.try_read(addr::REG_SYNCCONFIG)?))
    }
    pub fn try_set_syncconfig(&self, value: types::Syncconfig) -> Result<(), RW::Error> {
        self.try_write(addr::REG_SYNCCONFIG, value.0)
    }
    pub fn try_with_syncconfig<F: FnOnce(types::Syncconfig) -> types::Syncconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Syncconfig(self.try_read(addr::REG_SYNCCONFIG)?);
        self.try_write(addr::REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn try_syncvalue(&self, index: usize) -> Result<types::Syncvalue, RW::Error> {
        assert!(index < 8);
        Ok(types::Syncvalue(self.try_read(addr::REG_SYNCVALUE + index as u8)?))
    }
    pub fn try_set_syncvalue(&self, index: usize, value: types::Syncvalue) -> Result<(), RW::Error> {
        assert!(index < 8);
        self.try_write(addr::REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn try_with_syncvalue<F: FnOnce(types::Syncvalue) -> types::Syncvalue>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 8);
        let tmp = types::Syncvalue(self.try_read(addr::REG_SYNCVALUE + index as u8)?);
        self.try_write(addr::REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn try_packetconfig1(&self) -> Result<types::Packetconfig1, RW::Error> {
        Ok(types::Packetconfig1(self.try_read(addr::REG_PACKETCONFIG1)?))
    }
    pub fn try_set_packetconfig1(&self, value: types::Packetconfig1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PACKETCONFIG1, value.0)
    }
    pub fn try_with_packetconfig1<F: FnOnce(types::Packetconfig1) -> types::Packetconfig1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Packetconfig1(self.try_read(addr::REG_PACKETCONFIG1)?);
        self.try_write(addr::REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn try_payloadlength(&self) -> Result<types::Payloadlength, RW::Error> {
        Ok(types::Payloadlength(self.try_read(addr::REG_PAYLOADLENGTH)?))
    }
    pub fn try_set_payloadlength(&self, value: types::Payloadlength) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PAYLOADLENGTH, value.0)
    }
    pub fn try_with_payloadlength<F: FnOnce(types::Payloadlength) -> types::Payloadlength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Payloadlength(self.try_read(addr::REG_PAYLOADLENGTH)?);
        self.try_write(addr::REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn try_nodeadrs(&self) -> Result<types::Nodeadrs, RW::Error> {
        Ok(types::Nodeadrs(self.try_read(addr::REG_NODEADRS)?))
    }
    pub fn try_set_nodeadrs(&self, value: types::Nodeadrs) -> Result<(), RW::Error> {
        self.try_write(addr::REG_NODEADRS, value.0)
    }
    pub fn try_with_nodeadrs<F: FnOnce(types::Nodeadrs) -> types::Nodeadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Nodeadrs(self.try_read(addr::REG_NODEADRS)?);
        self.try_write(addr::REG_NODEADRS, f(tmp).0)
    }

    pub fn try_broadcastadrs(&self) -> Result<types::Broadcastadrs, RW::Error> {
        Ok(types::Broadcastadrs(self.try_read(addr::REG_BROADCASTADRS)?))
    }
    pub fn try_set_broadcastadrs(&self, value: types::Broadcastadrs) -> Result<(), RW::Error> {
        self.try_write(addr::REG_BROADCASTADRS, value.0)
    }
    pub fn try_with_broadcastadrs<F: FnOnce(types::Broadcastadrs) -> types::Broadcastadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Broadcastadrs(self.try_read(addr::REG_BROADCASTADRS)?);
        self.try_write(addr::REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn try_automodes(&self) -> Result<types::Automodes, RW::Error> {
        Ok(types::Automodes(self.try_read(addr::REG_AUTOMODES)?))
    }
    pub fn try_set_automodes(&self, value: types::Automodes) -> Result<(), RW::Error> {
        self.try_write(addr::REG_AUTOMODES, value.0)
    }
    pub fn try_with_automodes<F: FnOnce(types::Automodes) -> types::Automodes>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Automodes(self.try_read(addr::REG_AUTOMODES)?);
        self.try_write(addr::REG_AUTOMODES, f(tmp).0)
    }

    pub fn try_fifothresh(&self) -> Result<types::Fifothresh, RW::Error> {
        Ok(types::Fifothresh(self.try_read(addr::REG_FIFOTHRESH)?))
    }
    pub fn try_set_fifothresh(&self, value: types::Fifothresh) -> Result<(), RW::Error> {
        self.try_write(addr::REG_FIFOTHRESH, value.0)
    }
    pub fn try_with_fifothresh<F: FnOnce(types::Fifothresh) -> types::Fifothresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Fifothresh(self.try_read(addr::REG_FIFOTHRESH)?);
        self.try_write(addr::REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn try_packetconfig2(&self) -> Result<types::Packetconfig2, RW::Error> {
        Ok(types::Packetconfig2(self.try_read(addr::REG_PACKETCONFIG2)?))
    }
    pub fn try_set_packetconfig2(&self, value: types::Packetconfig2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_PACKETCONFIG2, value.0)
    }
    pub fn try_with_packetconfig2<F: FnOnce(types::Packetconfig2) -> types::Packetconfig2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Packetconfig2(self.try_read(addr::REG_PACKETCONFIG2)?);
        self.try_write(addr::REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn try_aeskey(&self, index: usize) -> Result<types::Aeskey, RW::Error> {
        assert!(index < 16);
        Ok(types::Aeskey(self.try_read(addr::REG_AESKEY + index as u8)?))
    }
    pub fn try_set_aeskey(&self, index: usize, value: types::Aeskey) -> Result<(), RW::Error> {
        assert!(index < 16);
        self.try_write(addr::REG_AESKEY + index as u8, value.0)
    }
    pub fn try_with_aeskey<F: FnOnce(types::Aeskey) -> types::Aeskey>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 16);
        let tmp = types::Aeskey(self.try_read(addr::REG_AESKEY + index as u8)?);
        self.try_write(addr::REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn try_temp1(&self) -> Result<types::Temp1, RW::Error> {
        Ok(types::Temp1(self.try_read(addr::REG_TEMP1)?))
    }
    pub fn try_set_temp1(&self, value: types::Temp1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TEMP1, value.0)
    }
    pub fn try_with_temp1<F: FnOnce(types::Temp1) -> types::Temp1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Temp1(self.try_read(addr::REG_TEMP1)?);
        self.try_write(addr::REG_TEMP1, f(tmp).0)
    }

    pub fn try_temp2(&self) -> Result<types::Temp2, RW::Error> {
        Ok(types::Temp2(self.try_read(addr::REG_TEMP2)?))
    }
    pub fn try_set_temp2(&self, value: types::Temp2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TEMP2, value.0)
    }
    pub fn try_with_temp2<F: FnOnce(types::Temp2) -> types::Temp2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Temp2(self.try_read(addr::REG_TEMP2)?);
        self.try_write(addr::REG_TEMP2, f(tmp).0)
    }

    pub fn try_testlna(&self) -> Result<types::Testlna, RW::Error> {
        Ok(types::Testlna(self.try_read(addr::REG_TESTLNA)?))
    }
    pub fn try_set_testlna(&self, value: types::Testlna) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TESTLNA, value.0)
    }
    pub fn try_with_testlna<F: FnOnce(types::Testlna) -> types::Testlna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Testlna(self.try_read(addr::REG_TESTLNA)?);
        self.try_write(addr::REG_TESTLNA, f(tmp).0)
    }

    pub fn try_testpa1(&self) -> Result<types::Testpa1, RW::Error> {
        Ok(types::Testpa1(self.try_read(addr::REG_TESTPA1)?))
    }
    pub fn try_set_testpa1(&self, value: types::Testpa1) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TESTPA1, value.0)
    }
    pub fn try_with_testpa1<F: FnOnce(types::Testpa1) -> types::Testpa1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Testpa1(self.try_read(addr::REG_TESTPA1)?);
        self.try_write(addr::REG_TESTPA1, f(tmp).0)
    }

    pub fn try_testpa2(&self) -> Result<types::Testpa2, RW::Error> {
        Ok(types::Testpa2(self.try_read(addr::REG_TESTPA2)?))
    }
    pub fn try_set_testpa2(&self, value: types::Testpa2) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TESTPA2, value.0)
    }
    pub fn try_with_testpa2<F: FnOnce(types::Testpa2) -> types::Testpa2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Testpa2(self.try_read(addr::REG_TESTPA2)?);
        self.try_write(addr::REG_TESTPA2, f(tmp).0)
    }

    pub fn try_testdagc(&self) -> Result<types::Testdagc, RW::Error> {
        Ok(types::Testdagc(self.try_read(addr::REG_TESTDAGC)?))
    }
    pub fn try_set_testdagc(&self, value: types::Testdagc) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TESTDAGC, value.0)
    }
    pub fn try_with_testdagc<F: FnOnce(types::Testdagc) -> types::Testdagc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Testdagc(self.try_read(addr::REG_TESTDAGC)?);
        self.try_write(addr::REG_TESTDAGC, f(tmp).0)
    }

    pub fn try_testafc(&self) -> Result<types::Testafc, RW::Error> {
        Ok(types::Testafc(self.try_read(addr::REG_TESTAFC)?))
    }
    pub fn try_set_testafc(&self, value: types::Testafc) -> Result<(), RW::Error> {
        self.try_write(addr::REG_TESTAFC, value.0)
    }
    pub fn try_with_testafc<F: FnOnce(types::Testafc) -> types::Testafc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = types::Testafc(self.try_read(addr::REG_TESTAFC)?);
        self.try_write(addr::REG_TESTAFC, f(tmp).0)
    }

}

pub mod types {
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

#[doc="Operating modes of the transceiver"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Opmode(pub u8);
impl Opmode {
    #[doc="Controls the automatic Sequencer (see section 4.2 ): 0 -> Operating mode as selected with Mode bits in RegOpMode is automatically reached with the Sequencer 1 -> Mode is forced by the user"]
    #[inline] pub fn sequencer_off(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SEQUENCER_OFF != 0"]
    #[inline] pub fn test_sequencer_off(&self) -> bool {
        self.sequencer_off() != 0
    }

    #[doc="Sets the SEQUENCER_OFF field."]
    #[inline] pub fn set_sequencer_off<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Enables Listen mode, should be enabled whilst in Standby mode: 0 → Off (see section 4.3) 1 → On"]
    #[inline] pub fn listen_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LISTEN_ON != 0"]
    #[inline] pub fn test_listen_on(&self) -> bool {
        self.listen_on() != 0
    }

    #[doc="Sets the LISTEN_ON field."]
    #[inline] pub fn set_listen_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Aborts Listen mode when set together with ListenOn=0 See section 4.3.4 for details Always reads 0."]
    #[inline] pub fn listen_abort(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LISTEN_ABORT != 0"]
    #[inline] pub fn test_listen_abort(&self) -> bool {
        self.listen_abort() != 0
    }

    #[doc="Sets the LISTEN_ABORT field."]
    #[inline] pub fn set_listen_abort<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transceiver’s operating modes: 000 → Sleep mode (SLEEP) 001 → Standby mode (STDBY) 010 → Frequency Synthesizer mode (FS) 011 → Transmitter mode (TX) 100 → Receiver mode (RX) others → reserved; Reads the value corresponding to the current module mode"]
    #[inline] pub fn mode(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
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
        if self.sequencer_off() != 0 { try!(write!(f, " sequencer_off"))}
        if self.listen_on() != 0 { try!(write!(f, " listen_on"))}
        if self.listen_abort() != 0 { try!(write!(f, " listen_abort"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data operation mode and Modulation settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datamodul(pub u8);
impl Datamodul {
    #[doc="Data processing mode: 00 → Packet mode 01 → reserved 10 → Continuous mode with bit synchronizer 11 → Continuous mode without bit synchronizer"]
    #[inline] pub fn data_mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if DATA_MODE != 0"]
    #[inline] pub fn test_data_mode(&self) -> bool {
        self.data_mode() != 0
    }

    #[doc="Sets the DATA_MODE field."]
    #[inline] pub fn set_data_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Modulation scheme: 00 → FSK 01 → OOK 10 - 11 → reserved"]
    #[inline] pub fn modulation_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if MODULATION_TYPE != 0"]
    #[inline] pub fn test_modulation_type(&self) -> bool {
        self.modulation_type() != 0
    }

    #[doc="Sets the MODULATION_TYPE field."]
    #[inline] pub fn set_modulation_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Gets the MODULATION_SHAPING field."]
    #[inline] pub fn modulation_shaping(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODULATION_SHAPING != 0"]
    #[inline] pub fn test_modulation_shaping(&self) -> bool {
        self.modulation_shaping() != 0
    }

    #[doc="Sets the MODULATION_SHAPING field."]
    #[inline] pub fn set_modulation_shaping<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datamodul {
    #[inline]
    fn from(other: u8) -> Self {
         Datamodul(other)
    }
}

impl ::core::fmt::Display for Datamodul {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datamodul {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data_mode() != 0 { try!(write!(f, " data_mode=0x{:x}", self.data_mode()))}
        if self.modulation_type() != 0 { try!(write!(f, " modulation_type=0x{:x}", self.modulation_type()))}
        if self.modulation_shaping() != 0 { try!(write!(f, " modulation_shaping=0x{:x}", self.modulation_shaping()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Bit Rate setting, Most Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitratemsb(pub u8);
impl Bitratemsb {
    #[inline] pub fn bitrate(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BITRATE != 0"]
    #[inline] pub fn test_bitrate(&self) -> bool {
        self.bitrate() != 0
    }

    #[doc="Sets the BITRATE field."]
    #[inline] pub fn set_bitrate<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bitratemsb {
    #[inline]
    fn from(other: u8) -> Self {
         Bitratemsb(other)
    }
}

impl ::core::fmt::Display for Bitratemsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bitratemsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bitrate() != 0 { try!(write!(f, " bitrate=0x{:x}", self.bitrate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Bit Rate setting, Least Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitratelsb(pub u8);
impl Bitratelsb {
    #[inline] pub fn bitrate(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BITRATE != 0"]
    #[inline] pub fn test_bitrate(&self) -> bool {
        self.bitrate() != 0
    }

    #[doc="Sets the BITRATE field."]
    #[inline] pub fn set_bitrate<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bitratelsb {
    #[inline]
    fn from(other: u8) -> Self {
         Bitratelsb(other)
    }
}

impl ::core::fmt::Display for Bitratelsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bitratelsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bitrate() != 0 { try!(write!(f, " bitrate=0x{:x}", self.bitrate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frequency Deviation setting, Most Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fdevmsb(pub u8);
impl Fdevmsb {
    #[inline] pub fn fdev(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FDEV != 0"]
    #[inline] pub fn test_fdev(&self) -> bool {
        self.fdev() != 0
    }

    #[doc="Sets the FDEV field."]
    #[inline] pub fn set_fdev<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fdevmsb {
    #[inline]
    fn from(other: u8) -> Self {
         Fdevmsb(other)
    }
}

impl ::core::fmt::Display for Fdevmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fdevmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fdev() != 0 { try!(write!(f, " fdev=0x{:x}", self.fdev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Frequency Deviation setting, Least Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fdevlsb(pub u8);
impl Fdevlsb {
    #[inline] pub fn fdev(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FDEV != 0"]
    #[inline] pub fn test_fdev(&self) -> bool {
        self.fdev() != 0
    }

    #[doc="Sets the FDEV field."]
    #[inline] pub fn set_fdev<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fdevlsb {
    #[inline]
    fn from(other: u8) -> Self {
         Fdevlsb(other)
    }
}

impl ::core::fmt::Display for Fdevlsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fdevlsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fdev() != 0 { try!(write!(f, " fdev=0x{:x}", self.fdev()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RF Carrier Frequency, Most Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frfmsb(pub u8);
impl Frfmsb {
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

impl From<u8> for Frfmsb {
    #[inline]
    fn from(other: u8) -> Self {
         Frfmsb(other)
    }
}

impl ::core::fmt::Display for Frfmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frfmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RF Carrier Frequency, Intermediate Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frfmid(pub u8);
impl Frfmid {
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

impl From<u8> for Frfmid {
    #[inline]
    fn from(other: u8) -> Self {
         Frfmid(other)
    }
}

impl ::core::fmt::Display for Frfmid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frfmid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RF Carrier Frequency, Least Significant Bits"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Frflsb(pub u8);
impl Frflsb {
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

impl From<u8> for Frflsb {
    #[inline]
    fn from(other: u8) -> Self {
         Frflsb(other)
    }
}

impl ::core::fmt::Display for Frflsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Frflsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RC Oscillators Settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Osc1(pub u8);
impl Osc1 {
    #[inline] pub fn rc_cal_start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RC_CAL_START != 0"]
    #[inline] pub fn test_rc_cal_start(&self) -> bool {
        self.rc_cal_start() != 0
    }

    #[doc="Sets the RC_CAL_START field."]
    #[inline] pub fn set_rc_cal_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn rc_cal_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RC_CAL_DONE != 0"]
    #[inline] pub fn test_rc_cal_done(&self) -> bool {
        self.rc_cal_done() != 0
    }

    #[doc="Sets the RC_CAL_DONE field."]
    #[inline] pub fn set_rc_cal_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Osc1 {
    #[inline]
    fn from(other: u8) -> Self {
         Osc1(other)
    }
}

impl ::core::fmt::Display for Osc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Osc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rc_cal_start() != 0 { try!(write!(f, " rc_cal_start"))}
        if self.rc_cal_done() != 0 { try!(write!(f, " rc_cal_done"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AFC control in low modulation index situations"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afcctrl(pub u8);
impl Afcctrl {
    #[inline] pub fn afc_low_beta_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFC_LOW_BETA_ON != 0"]
    #[inline] pub fn test_afc_low_beta_on(&self) -> bool {
        self.afc_low_beta_on() != 0
    }

    #[doc="Sets the AFC_LOW_BETA_ON field."]
    #[inline] pub fn set_afc_low_beta_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Afcctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Afcctrl(other)
    }
}

impl ::core::fmt::Display for Afcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afc_low_beta_on() != 0 { try!(write!(f, " afc_low_beta_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Listen Mode settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Listen1(pub u8);
impl Listen1 {
    #[inline] pub fn listen_resol_idle(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if LISTEN_RESOL_IDLE != 0"]
    #[inline] pub fn test_listen_resol_idle(&self) -> bool {
        self.listen_resol_idle() != 0
    }

    #[doc="Sets the LISTEN_RESOL_IDLE field."]
    #[inline] pub fn set_listen_resol_idle<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn listen_resol_rx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if LISTEN_RESOL_RX != 0"]
    #[inline] pub fn test_listen_resol_rx(&self) -> bool {
        self.listen_resol_rx() != 0
    }

    #[doc="Sets the LISTEN_RESOL_RX field."]
    #[inline] pub fn set_listen_resol_rx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn listen_criteria(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LISTEN_CRITERIA != 0"]
    #[inline] pub fn test_listen_criteria(&self) -> bool {
        self.listen_criteria() != 0
    }

    #[doc="Sets the LISTEN_CRITERIA field."]
    #[inline] pub fn set_listen_criteria<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn listen_end(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if LISTEN_END != 0"]
    #[inline] pub fn test_listen_end(&self) -> bool {
        self.listen_end() != 0
    }

    #[doc="Sets the LISTEN_END field."]
    #[inline] pub fn set_listen_end<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Listen1 {
    #[inline]
    fn from(other: u8) -> Self {
         Listen1(other)
    }
}

impl ::core::fmt::Display for Listen1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Listen1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.listen_resol_idle() != 0 { try!(write!(f, " listen_resol_idle=0x{:x}", self.listen_resol_idle()))}
        if self.listen_resol_rx() != 0 { try!(write!(f, " listen_resol_rx=0x{:x}", self.listen_resol_rx()))}
        if self.listen_criteria() != 0 { try!(write!(f, " listen_criteria"))}
        if self.listen_end() != 0 { try!(write!(f, " listen_end=0x{:x}", self.listen_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Listen Mode Idle duration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Listen2(pub u8);
impl Listen2 {
    #[inline] pub fn listen_coef_idle(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LISTEN_COEF_IDLE != 0"]
    #[inline] pub fn test_listen_coef_idle(&self) -> bool {
        self.listen_coef_idle() != 0
    }

    #[doc="Sets the LISTEN_COEF_IDLE field."]
    #[inline] pub fn set_listen_coef_idle<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Listen2 {
    #[inline]
    fn from(other: u8) -> Self {
         Listen2(other)
    }
}

impl ::core::fmt::Display for Listen2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Listen2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.listen_coef_idle() != 0 { try!(write!(f, " listen_coef_idle=0x{:x}", self.listen_coef_idle()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Listen Mode Rx duration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Listen3(pub u8);
impl Listen3 {
    #[inline] pub fn listen_coef_rx(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LISTEN_COEF_RX != 0"]
    #[inline] pub fn test_listen_coef_rx(&self) -> bool {
        self.listen_coef_rx() != 0
    }

    #[doc="Sets the LISTEN_COEF_RX field."]
    #[inline] pub fn set_listen_coef_rx<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Listen3 {
    #[inline]
    fn from(other: u8) -> Self {
         Listen3(other)
    }
}

impl ::core::fmt::Display for Listen3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Listen3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.listen_coef_rx() != 0 { try!(write!(f, " listen_coef_rx=0x{:x}", self.listen_coef_rx()))}
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

#[doc="PA selection and Output Power control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Palevel(pub u8);
impl Palevel {
    #[inline] pub fn pa0_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PA0_ON != 0"]
    #[inline] pub fn test_pa0_on(&self) -> bool {
        self.pa0_on() != 0
    }

    #[doc="Sets the PA0_ON field."]
    #[inline] pub fn set_pa0_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn pa1_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PA1_ON != 0"]
    #[inline] pub fn test_pa1_on(&self) -> bool {
        self.pa1_on() != 0
    }

    #[doc="Sets the PA1_ON field."]
    #[inline] pub fn set_pa1_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn pa2_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PA2_ON != 0"]
    #[inline] pub fn test_pa2_on(&self) -> bool {
        self.pa2_on() != 0
    }

    #[doc="Sets the PA2_ON field."]
    #[inline] pub fn set_pa2_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn output_power(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if OUTPUT_POWER != 0"]
    #[inline] pub fn test_output_power(&self) -> bool {
        self.output_power() != 0
    }

    #[doc="Sets the OUTPUT_POWER field."]
    #[inline] pub fn set_output_power<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Palevel {
    #[inline]
    fn from(other: u8) -> Self {
         Palevel(other)
    }
}

impl ::core::fmt::Display for Palevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Palevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa0_on() != 0 { try!(write!(f, " pa0_on"))}
        if self.pa1_on() != 0 { try!(write!(f, " pa1_on"))}
        if self.pa2_on() != 0 { try!(write!(f, " pa2_on"))}
        if self.output_power() != 0 { try!(write!(f, " output_power=0x{:x}", self.output_power()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control of the PA ramp time in FSK mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Paramp(pub u8);
impl Paramp {
    #[inline] pub fn pa_ramo(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PA_RAMO != 0"]
    #[inline] pub fn test_pa_ramo(&self) -> bool {
        self.pa_ramo() != 0
    }

    #[doc="Sets the PA_RAMO field."]
    #[inline] pub fn set_pa_ramo<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Paramp {
    #[inline]
    fn from(other: u8) -> Self {
         Paramp(other)
    }
}

impl ::core::fmt::Display for Paramp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Paramp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa_ramo() != 0 { try!(write!(f, " pa_ramo=0x{:x}", self.pa_ramo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Over Current Protection control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ocp(pub u8);
impl Ocp {
    #[inline] pub fn ocp_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OCP_ON != 0"]
    #[inline] pub fn test_ocp_on(&self) -> bool {
        self.ocp_on() != 0
    }

    #[doc="Sets the OCP_ON field."]
    #[inline] pub fn set_ocp_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn ocp_trim(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if OCP_TRIM != 0"]
    #[inline] pub fn test_ocp_trim(&self) -> bool {
        self.ocp_trim() != 0
    }

    #[doc="Sets the OCP_TRIM field."]
    #[inline] pub fn set_ocp_trim<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
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

#[doc="LNA settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lna(pub u8);
impl Lna {
    #[inline] pub fn lna_zin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LNA_ZIN != 0"]
    #[inline] pub fn test_lna_zin(&self) -> bool {
        self.lna_zin() != 0
    }

    #[doc="Sets the LNA_ZIN field."]
    #[inline] pub fn set_lna_zin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn lna_current_gain(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if LNA_CURRENT_GAIN != 0"]
    #[inline] pub fn test_lna_current_gain(&self) -> bool {
        self.lna_current_gain() != 0
    }

    #[doc="Sets the LNA_CURRENT_GAIN field."]
    #[inline] pub fn set_lna_current_gain<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn lna_gain_select(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LNA_GAIN_SELECT != 0"]
    #[inline] pub fn test_lna_gain_select(&self) -> bool {
        self.lna_gain_select() != 0
    }

    #[doc="Sets the LNA_GAIN_SELECT field."]
    #[inline] pub fn set_lna_gain_select<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
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
        if self.lna_zin() != 0 { try!(write!(f, " lna_zin"))}
        if self.lna_current_gain() != 0 { try!(write!(f, " lna_current_gain=0x{:x}", self.lna_current_gain()))}
        if self.lna_gain_select() != 0 { try!(write!(f, " lna_gain_select=0x{:x}", self.lna_gain_select()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Filter BW Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxbw(pub u8);
impl Rxbw {
    #[inline] pub fn dcc_freq(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if DCC_FREQ != 0"]
    #[inline] pub fn test_dcc_freq(&self) -> bool {
        self.dcc_freq() != 0
    }

    #[doc="Sets the DCC_FREQ field."]
    #[inline] pub fn set_dcc_freq<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn rx_bw_mant(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if RX_BW_MANT != 0"]
    #[inline] pub fn test_rx_bw_mant(&self) -> bool {
        self.rx_bw_mant() != 0
    }

    #[doc="Sets the RX_BW_MANT field."]
    #[inline] pub fn set_rx_bw_mant<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn rx_bw_exp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RX_BW_EXP != 0"]
    #[inline] pub fn test_rx_bw_exp(&self) -> bool {
        self.rx_bw_exp() != 0
    }

    #[doc="Sets the RX_BW_EXP field."]
    #[inline] pub fn set_rx_bw_exp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rxbw {
    #[inline]
    fn from(other: u8) -> Self {
         Rxbw(other)
    }
}

impl ::core::fmt::Display for Rxbw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxbw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcc_freq() != 0 { try!(write!(f, " dcc_freq=0x{:x}", self.dcc_freq()))}
        if self.rx_bw_mant() != 0 { try!(write!(f, " rx_bw_mant=0x{:x}", self.rx_bw_mant()))}
        if self.rx_bw_exp() != 0 { try!(write!(f, " rx_bw_exp=0x{:x}", self.rx_bw_exp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Filter BW control during the AFC routine"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afcbw(pub u8);
impl Afcbw {
    #[inline] pub fn dcc_freq_afc(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7f) as u8) } // [11:5]
    }

    #[doc="Returns true if DCC_FREQ_AFC != 0"]
    #[inline] pub fn test_dcc_freq_afc(&self) -> bool {
        self.dcc_freq_afc() != 0
    }

    #[doc="Sets the DCC_FREQ_AFC field."]
    #[inline] pub fn set_dcc_freq_afc<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn rx_bw_mant_afc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if RX_BW_MANT_AFC != 0"]
    #[inline] pub fn test_rx_bw_mant_afc(&self) -> bool {
        self.rx_bw_mant_afc() != 0
    }

    #[doc="Sets the RX_BW_MANT_AFC field."]
    #[inline] pub fn set_rx_bw_mant_afc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn rx_bw_exp_afc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if RX_BW_EXP_AFC != 0"]
    #[inline] pub fn test_rx_bw_exp_afc(&self) -> bool {
        self.rx_bw_exp_afc() != 0
    }

    #[doc="Sets the RX_BW_EXP_AFC field."]
    #[inline] pub fn set_rx_bw_exp_afc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Afcbw {
    #[inline]
    fn from(other: u8) -> Self {
         Afcbw(other)
    }
}

impl ::core::fmt::Display for Afcbw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afcbw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcc_freq_afc() != 0 { try!(write!(f, " dcc_freq_afc=0x{:x}", self.dcc_freq_afc()))}
        if self.rx_bw_mant_afc() != 0 { try!(write!(f, " rx_bw_mant_afc=0x{:x}", self.rx_bw_mant_afc()))}
        if self.rx_bw_exp_afc() != 0 { try!(write!(f, " rx_bw_exp_afc=0x{:x}", self.rx_bw_exp_afc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OOK demodulator selection and control in peak mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ookpeak(pub u8);
impl Ookpeak {
    #[inline] pub fn ook_thresh_type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if OOK_THRESH_TYPE != 0"]
    #[inline] pub fn test_ook_thresh_type(&self) -> bool {
        self.ook_thresh_type() != 0
    }

    #[doc="Sets the OOK_THRESH_TYPE field."]
    #[inline] pub fn set_ook_thresh_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn ook_peak_thresh_step(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if OOK_PEAK_THRESH_STEP != 0"]
    #[inline] pub fn test_ook_peak_thresh_step(&self) -> bool {
        self.ook_peak_thresh_step() != 0
    }

    #[doc="Sets the OOK_PEAK_THRESH_STEP field."]
    #[inline] pub fn set_ook_peak_thresh_step<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn ook_peak_thresh_dec(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if OOK_PEAK_THRESH_DEC != 0"]
    #[inline] pub fn test_ook_peak_thresh_dec(&self) -> bool {
        self.ook_peak_thresh_dec() != 0
    }

    #[doc="Sets the OOK_PEAK_THRESH_DEC field."]
    #[inline] pub fn set_ook_peak_thresh_dec<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ookpeak {
    #[inline]
    fn from(other: u8) -> Self {
         Ookpeak(other)
    }
}

impl ::core::fmt::Display for Ookpeak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ookpeak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ook_thresh_type() != 0 { try!(write!(f, " ook_thresh_type=0x{:x}", self.ook_thresh_type()))}
        if self.ook_peak_thresh_step() != 0 { try!(write!(f, " ook_peak_thresh_step=0x{:x}", self.ook_peak_thresh_step()))}
        if self.ook_peak_thresh_dec() != 0 { try!(write!(f, " ook_peak_thresh_dec=0x{:x}", self.ook_peak_thresh_dec()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Average threshold control of the OOK demodulator"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ookavg(pub u8);
impl Ookavg {
    #[inline] pub fn ook_average_thresh_filt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if OOK_AVERAGE_THRESH_FILT != 0"]
    #[inline] pub fn test_ook_average_thresh_filt(&self) -> bool {
        self.ook_average_thresh_filt() != 0
    }

    #[doc="Sets the OOK_AVERAGE_THRESH_FILT field."]
    #[inline] pub fn set_ook_average_thresh_filt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ookavg {
    #[inline]
    fn from(other: u8) -> Self {
         Ookavg(other)
    }
}

impl ::core::fmt::Display for Ookavg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ookavg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ook_average_thresh_filt() != 0 { try!(write!(f, " ook_average_thresh_filt=0x{:x}", self.ook_average_thresh_filt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fixed threshold control of the OOK demodulator"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ookfix(pub u8);
impl Ookfix {
    #[inline] pub fn ook_fixed_thresh(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if OOK_FIXED_THRESH != 0"]
    #[inline] pub fn test_ook_fixed_thresh(&self) -> bool {
        self.ook_fixed_thresh() != 0
    }

    #[doc="Sets the OOK_FIXED_THRESH field."]
    #[inline] pub fn set_ook_fixed_thresh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ookfix {
    #[inline]
    fn from(other: u8) -> Self {
         Ookfix(other)
    }
}

impl ::core::fmt::Display for Ookfix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ookfix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ook_fixed_thresh() != 0 { try!(write!(f, " ook_fixed_thresh=0x{:x}", self.ook_fixed_thresh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AFC and FEI control and status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afcfei(pub u8);
impl Afcfei {
    #[inline] pub fn fei_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FEI_DONE != 0"]
    #[inline] pub fn test_fei_done(&self) -> bool {
        self.fei_done() != 0
    }

    #[doc="Sets the FEI_DONE field."]
    #[inline] pub fn set_fei_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn fei_start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FEI_START != 0"]
    #[inline] pub fn test_fei_start(&self) -> bool {
        self.fei_start() != 0
    }

    #[doc="Sets the FEI_START field."]
    #[inline] pub fn set_fei_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn afc_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AFC_DONE != 0"]
    #[inline] pub fn test_afc_done(&self) -> bool {
        self.afc_done() != 0
    }

    #[doc="Sets the AFC_DONE field."]
    #[inline] pub fn set_afc_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn afc_autoclear_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if AFC_AUTOCLEAR_ON != 0"]
    #[inline] pub fn test_afc_autoclear_on(&self) -> bool {
        self.afc_autoclear_on() != 0
    }

    #[doc="Sets the AFC_AUTOCLEAR_ON field."]
    #[inline] pub fn set_afc_autoclear_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn afc_auto_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AFC_AUTO_ON != 0"]
    #[inline] pub fn test_afc_auto_on(&self) -> bool {
        self.afc_auto_on() != 0
    }

    #[doc="Sets the AFC_AUTO_ON field."]
    #[inline] pub fn set_afc_auto_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn afc_clear(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AFC_CLEAR != 0"]
    #[inline] pub fn test_afc_clear(&self) -> bool {
        self.afc_clear() != 0
    }

    #[doc="Sets the AFC_CLEAR field."]
    #[inline] pub fn set_afc_clear<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn afc_start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AFC_START != 0"]
    #[inline] pub fn test_afc_start(&self) -> bool {
        self.afc_start() != 0
    }

    #[doc="Sets the AFC_START field."]
    #[inline] pub fn set_afc_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Afcfei {
    #[inline]
    fn from(other: u8) -> Self {
         Afcfei(other)
    }
}

impl ::core::fmt::Display for Afcfei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afcfei {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fei_done() != 0 { try!(write!(f, " fei_done"))}
        if self.fei_start() != 0 { try!(write!(f, " fei_start"))}
        if self.afc_done() != 0 { try!(write!(f, " afc_done"))}
        if self.afc_autoclear_on() != 0 { try!(write!(f, " afc_autoclear_on"))}
        if self.afc_auto_on() != 0 { try!(write!(f, " afc_auto_on"))}
        if self.afc_clear() != 0 { try!(write!(f, " afc_clear"))}
        if self.afc_start() != 0 { try!(write!(f, " afc_start"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MSB of the frequency correction of the AFC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afcmsb(pub u8);
impl Afcmsb {
    #[inline] pub fn afc_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AFC_VALUE != 0"]
    #[inline] pub fn test_afc_value(&self) -> bool {
        self.afc_value() != 0
    }

    #[doc="Sets the AFC_VALUE field."]
    #[inline] pub fn set_afc_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Afcmsb {
    #[inline]
    fn from(other: u8) -> Self {
         Afcmsb(other)
    }
}

impl ::core::fmt::Display for Afcmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afcmsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afc_value() != 0 { try!(write!(f, " afc_value=0x{:x}", self.afc_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LSB of the frequency correction of the AFC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afclsb(pub u8);
impl Afclsb {
    #[inline] pub fn afc_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AFC_VALUE != 0"]
    #[inline] pub fn test_afc_value(&self) -> bool {
        self.afc_value() != 0
    }

    #[doc="Sets the AFC_VALUE field."]
    #[inline] pub fn set_afc_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Afclsb {
    #[inline]
    fn from(other: u8) -> Self {
         Afclsb(other)
    }
}

impl ::core::fmt::Display for Afclsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afclsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afc_value() != 0 { try!(write!(f, " afc_value=0x{:x}", self.afc_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MSB of the calculated frequency error"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Feimsb(pub u8);
impl Feimsb {
    #[inline] pub fn fei_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FEI_VALUE != 0"]
    #[inline] pub fn test_fei_value(&self) -> bool {
        self.fei_value() != 0
    }

    #[doc="Sets the FEI_VALUE field."]
    #[inline] pub fn set_fei_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Feimsb {
    #[inline]
    fn from(other: u8) -> Self {
         Feimsb(other)
    }
}

impl ::core::fmt::Display for Feimsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Feimsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fei_value() != 0 { try!(write!(f, " fei_value=0x{:x}", self.fei_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LSB of the calculated frequency error"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Feilsb(pub u8);
impl Feilsb {
    #[inline] pub fn fei_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FEI_VALUE != 0"]
    #[inline] pub fn test_fei_value(&self) -> bool {
        self.fei_value() != 0
    }

    #[doc="Sets the FEI_VALUE field."]
    #[inline] pub fn set_fei_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Feilsb {
    #[inline]
    fn from(other: u8) -> Self {
         Feilsb(other)
    }
}

impl ::core::fmt::Display for Feilsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Feilsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fei_value() != 0 { try!(write!(f, " fei_value=0x{:x}", self.fei_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RSSI-related settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rssiconfig(pub u8);
impl Rssiconfig {
    #[inline] pub fn rssi_done(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RSSI_DONE != 0"]
    #[inline] pub fn test_rssi_done(&self) -> bool {
        self.rssi_done() != 0
    }

    #[doc="Sets the RSSI_DONE field."]
    #[inline] pub fn set_rssi_done<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn rssi_start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RSSI_START != 0"]
    #[inline] pub fn test_rssi_start(&self) -> bool {
        self.rssi_start() != 0
    }

    #[doc="Sets the RSSI_START field."]
    #[inline] pub fn set_rssi_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rssiconfig {
    #[inline]
    fn from(other: u8) -> Self {
         Rssiconfig(other)
    }
}

impl ::core::fmt::Display for Rssiconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rssiconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rssi_done() != 0 { try!(write!(f, " rssi_done"))}
        if self.rssi_start() != 0 { try!(write!(f, " rssi_start"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RSSI value in dBm"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rssivalue(pub u8);
impl Rssivalue {
    #[inline] pub fn rssi_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSSI_VALUE != 0"]
    #[inline] pub fn test_rssi_value(&self) -> bool {
        self.rssi_value() != 0
    }

    #[doc="Sets the RSSI_VALUE field."]
    #[inline] pub fn set_rssi_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rssivalue {
    #[inline]
    fn from(other: u8) -> Self {
         Rssivalue(other)
    }
}

impl ::core::fmt::Display for Rssivalue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rssivalue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rssi_value() != 0 { try!(write!(f, " rssi_value=0x{:x}", self.rssi_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Mapping of pins DIO0 to DIO3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diomapping1(pub u8);
impl Diomapping1 {
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

impl From<u8> for Diomapping1 {
    #[inline]
    fn from(other: u8) -> Self {
         Diomapping1(other)
    }
}

impl ::core::fmt::Display for Diomapping1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diomapping1 {
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

#[doc="Mapping of pins DIO4 and DIO5, ClkOut frequency"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diomapping2(pub u8);
impl Diomapping2 {
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

    #[inline] pub fn clk_out(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CLK_OUT != 0"]
    #[inline] pub fn test_clk_out(&self) -> bool {
        self.clk_out() != 0
    }

    #[doc="Sets the CLK_OUT field."]
    #[inline] pub fn set_clk_out<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Diomapping2 {
    #[inline]
    fn from(other: u8) -> Self {
         Diomapping2(other)
    }
}

impl ::core::fmt::Display for Diomapping2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diomapping2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dio4_mapping() != 0 { try!(write!(f, " dio4_mapping=0x{:x}", self.dio4_mapping()))}
        if self.dio5_mapping() != 0 { try!(write!(f, " dio5_mapping=0x{:x}", self.dio5_mapping()))}
        if self.clk_out() != 0 { try!(write!(f, " clk_out=0x{:x}", self.clk_out()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register: PLL Lock state, Timeout, RSSI > Threshold..."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Irqflags1(pub u8);
impl Irqflags1 {
    #[inline] pub fn mode_ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MODE_READY != 0"]
    #[inline] pub fn test_mode_ready(&self) -> bool {
        self.mode_ready() != 0
    }

    #[doc="Sets the MODE_READY field."]
    #[inline] pub fn set_mode_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn rx_ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RX_READY != 0"]
    #[inline] pub fn test_rx_ready(&self) -> bool {
        self.rx_ready() != 0
    }

    #[doc="Sets the RX_READY field."]
    #[inline] pub fn set_rx_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn tx_ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TX_READY != 0"]
    #[inline] pub fn test_tx_ready(&self) -> bool {
        self.tx_ready() != 0
    }

    #[doc="Sets the TX_READY field."]
    #[inline] pub fn set_tx_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn pll_lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLL_LOCK != 0"]
    #[inline] pub fn test_pll_lock(&self) -> bool {
        self.pll_lock() != 0
    }

    #[doc="Sets the PLL_LOCK field."]
    #[inline] pub fn set_pll_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn rssi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RSSI != 0"]
    #[inline] pub fn test_rssi(&self) -> bool {
        self.rssi() != 0
    }

    #[doc="Sets the RSSI field."]
    #[inline] pub fn set_rssi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn timeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIMEOUT != 0"]
    #[inline] pub fn test_timeout(&self) -> bool {
        self.timeout() != 0
    }

    #[doc="Sets the TIMEOUT field."]
    #[inline] pub fn set_timeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn auto_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AUTO_MODE != 0"]
    #[inline] pub fn test_auto_mode(&self) -> bool {
        self.auto_mode() != 0
    }

    #[doc="Sets the AUTO_MODE field."]
    #[inline] pub fn set_auto_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn sync_address_match(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYNC_ADDRESS_MATCH != 0"]
    #[inline] pub fn test_sync_address_match(&self) -> bool {
        self.sync_address_match() != 0
    }

    #[doc="Sets the SYNC_ADDRESS_MATCH field."]
    #[inline] pub fn set_sync_address_match<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Irqflags1 {
    #[inline]
    fn from(other: u8) -> Self {
         Irqflags1(other)
    }
}

impl ::core::fmt::Display for Irqflags1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Irqflags1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode_ready() != 0 { try!(write!(f, " mode_ready"))}
        if self.rx_ready() != 0 { try!(write!(f, " rx_ready"))}
        if self.tx_ready() != 0 { try!(write!(f, " tx_ready"))}
        if self.pll_lock() != 0 { try!(write!(f, " pll_lock"))}
        if self.rssi() != 0 { try!(write!(f, " rssi"))}
        if self.timeout() != 0 { try!(write!(f, " timeout"))}
        if self.auto_mode() != 0 { try!(write!(f, " auto_mode"))}
        if self.sync_address_match() != 0 { try!(write!(f, " sync_address_match"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register: FIFO handling flags..."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Irqflags2(pub u8);
impl Irqflags2 {
    #[inline] pub fn fifo_full(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FIFO_FULL != 0"]
    #[inline] pub fn test_fifo_full(&self) -> bool {
        self.fifo_full() != 0
    }

    #[doc="Sets the FIFO_FULL field."]
    #[inline] pub fn set_fifo_full<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn fifo_not_empty(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FIFO_NOT_EMPTY != 0"]
    #[inline] pub fn test_fifo_not_empty(&self) -> bool {
        self.fifo_not_empty() != 0
    }

    #[doc="Sets the FIFO_NOT_EMPTY field."]
    #[inline] pub fn set_fifo_not_empty<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn fifo_level(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FIFO_LEVEL != 0"]
    #[inline] pub fn test_fifo_level(&self) -> bool {
        self.fifo_level() != 0
    }

    #[doc="Sets the FIFO_LEVEL field."]
    #[inline] pub fn set_fifo_level<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn fifo_overrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FIFO_OVERRUN != 0"]
    #[inline] pub fn test_fifo_overrun(&self) -> bool {
        self.fifo_overrun() != 0
    }

    #[doc="Sets the FIFO_OVERRUN field."]
    #[inline] pub fn set_fifo_overrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn packet_sent(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PACKET_SENT != 0"]
    #[inline] pub fn test_packet_sent(&self) -> bool {
        self.packet_sent() != 0
    }

    #[doc="Sets the PACKET_SENT field."]
    #[inline] pub fn set_packet_sent<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn payload_ready(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PAYLOAD_READY != 0"]
    #[inline] pub fn test_payload_ready(&self) -> bool {
        self.payload_ready() != 0
    }

    #[doc="Sets the PAYLOAD_READY field."]
    #[inline] pub fn set_payload_ready<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn crc_ok(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRC_OK != 0"]
    #[inline] pub fn test_crc_ok(&self) -> bool {
        self.crc_ok() != 0
    }

    #[doc="Sets the CRC_OK field."]
    #[inline] pub fn set_crc_ok<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Irqflags2 {
    #[inline]
    fn from(other: u8) -> Self {
         Irqflags2(other)
    }
}

impl ::core::fmt::Display for Irqflags2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Irqflags2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifo_full() != 0 { try!(write!(f, " fifo_full"))}
        if self.fifo_not_empty() != 0 { try!(write!(f, " fifo_not_empty"))}
        if self.fifo_level() != 0 { try!(write!(f, " fifo_level"))}
        if self.fifo_overrun() != 0 { try!(write!(f, " fifo_overrun"))}
        if self.packet_sent() != 0 { try!(write!(f, " packet_sent"))}
        if self.payload_ready() != 0 { try!(write!(f, " payload_ready"))}
        if self.crc_ok() != 0 { try!(write!(f, " crc_ok"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RSSI Threshold control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rssithresh(pub u8);
impl Rssithresh {
    #[inline] pub fn rssi_threshold(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RSSI_THRESHOLD != 0"]
    #[inline] pub fn test_rssi_threshold(&self) -> bool {
        self.rssi_threshold() != 0
    }

    #[doc="Sets the RSSI_THRESHOLD field."]
    #[inline] pub fn set_rssi_threshold<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rssithresh {
    #[inline]
    fn from(other: u8) -> Self {
         Rssithresh(other)
    }
}

impl ::core::fmt::Display for Rssithresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rssithresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rssi_threshold() != 0 { try!(write!(f, " rssi_threshold=0x{:x}", self.rssi_threshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timeout duration between Rx request and RSSI detection"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxtimeout1(pub u8);
impl Rxtimeout1 {
    #[inline] pub fn timeout_rx_start(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TIMEOUT_RX_START != 0"]
    #[inline] pub fn test_timeout_rx_start(&self) -> bool {
        self.timeout_rx_start() != 0
    }

    #[doc="Sets the TIMEOUT_RX_START field."]
    #[inline] pub fn set_timeout_rx_start<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rxtimeout1 {
    #[inline]
    fn from(other: u8) -> Self {
         Rxtimeout1(other)
    }
}

impl ::core::fmt::Display for Rxtimeout1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxtimeout1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timeout_rx_start() != 0 { try!(write!(f, " timeout_rx_start=0x{:x}", self.timeout_rx_start()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Timeout duration between RSSI detection and PayloadReady"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxtimeout2(pub u8);
impl Rxtimeout2 {
    #[inline] pub fn timeout_rssi_thresh(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TIMEOUT_RSSI_THRESH != 0"]
    #[inline] pub fn test_timeout_rssi_thresh(&self) -> bool {
        self.timeout_rssi_thresh() != 0
    }

    #[doc="Sets the TIMEOUT_RSSI_THRESH field."]
    #[inline] pub fn set_timeout_rssi_thresh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rxtimeout2 {
    #[inline]
    fn from(other: u8) -> Self {
         Rxtimeout2(other)
    }
}

impl ::core::fmt::Display for Rxtimeout2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxtimeout2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.timeout_rssi_thresh() != 0 { try!(write!(f, " timeout_rssi_thresh=0x{:x}", self.timeout_rssi_thresh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Preamble length, MSB"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Preamblemsb(pub u8);
impl Preamblemsb {
    #[inline] pub fn preamble_size(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLE_SIZE != 0"]
    #[inline] pub fn test_preamble_size(&self) -> bool {
        self.preamble_size() != 0
    }

    #[doc="Sets the PREAMBLE_SIZE field."]
    #[inline] pub fn set_preamble_size<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Preamblemsb {
    #[inline]
    fn from(other: u8) -> Self {
         Preamblemsb(other)
    }
}

impl ::core::fmt::Display for Preamblemsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Preamblemsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preamble_size() != 0 { try!(write!(f, " preamble_size=0x{:x}", self.preamble_size()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Preamble length, LSB"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Preamblelsb(pub u8);
impl Preamblelsb {
    #[inline] pub fn preamble_size(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PREAMBLE_SIZE != 0"]
    #[inline] pub fn test_preamble_size(&self) -> bool {
        self.preamble_size() != 0
    }

    #[doc="Sets the PREAMBLE_SIZE field."]
    #[inline] pub fn set_preamble_size<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Preamblelsb {
    #[inline]
    fn from(other: u8) -> Self {
         Preamblelsb(other)
    }
}

impl ::core::fmt::Display for Preamblelsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Preamblelsb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.preamble_size() != 0 { try!(write!(f, " preamble_size=0x{:x}", self.preamble_size()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sync Word Recognition control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncconfig(pub u8);
impl Syncconfig {
    #[inline] pub fn sync_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNC_ON != 0"]
    #[inline] pub fn test_sync_on(&self) -> bool {
        self.sync_on() != 0
    }

    #[doc="Sets the SYNC_ON field."]
    #[inline] pub fn set_sync_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn fifo_fill_condition(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FIFO_FILL_CONDITION != 0"]
    #[inline] pub fn test_fifo_fill_condition(&self) -> bool {
        self.fifo_fill_condition() != 0
    }

    #[doc="Sets the FIFO_FILL_CONDITION field."]
    #[inline] pub fn set_fifo_fill_condition<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[inline] pub fn sync_size(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if SYNC_SIZE != 0"]
    #[inline] pub fn test_sync_size(&self) -> bool {
        self.sync_size() != 0
    }

    #[doc="Sets the SYNC_SIZE field."]
    #[inline] pub fn set_sync_size<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn sync_tol(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SYNC_TOL != 0"]
    #[inline] pub fn test_sync_tol(&self) -> bool {
        self.sync_tol() != 0
    }

    #[doc="Sets the SYNC_TOL field."]
    #[inline] pub fn set_sync_tol<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Syncconfig {
    #[inline]
    fn from(other: u8) -> Self {
         Syncconfig(other)
    }
}

impl ::core::fmt::Display for Syncconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_on() != 0 { try!(write!(f, " sync_on"))}
        if self.fifo_fill_condition() != 0 { try!(write!(f, " fifo_fill_condition"))}
        if self.sync_size() != 0 { try!(write!(f, " sync_size=0x{:x}", self.sync_size()))}
        if self.sync_tol() != 0 { try!(write!(f, " sync_tol=0x{:x}", self.sync_tol()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sync Word bytes, 1 through 8"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncvalue(pub u8);
impl Syncvalue {
    #[inline] pub fn sync_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SYNC_VALUE != 0"]
    #[inline] pub fn test_sync_value(&self) -> bool {
        self.sync_value() != 0
    }

    #[doc="Sets the SYNC_VALUE field."]
    #[inline] pub fn set_sync_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Syncvalue {
    #[inline]
    fn from(other: u8) -> Self {
         Syncvalue(other)
    }
}

impl ::core::fmt::Display for Syncvalue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncvalue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sync_value() != 0 { try!(write!(f, " sync_value=0x{:x}", self.sync_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Packet mode settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Packetconfig1(pub u8);
impl Packetconfig1 {
    #[inline] pub fn packet_format(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PACKET_FORMAT != 0"]
    #[inline] pub fn test_packet_format(&self) -> bool {
        self.packet_format() != 0
    }

    #[doc="Sets the PACKET_FORMAT field."]
    #[inline] pub fn set_packet_format<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn dc_free(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if DC_FREE != 0"]
    #[inline] pub fn test_dc_free(&self) -> bool {
        self.dc_free() != 0
    }

    #[doc="Sets the DC_FREE field."]
    #[inline] pub fn set_dc_free<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn crc_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CRC_ON != 0"]
    #[inline] pub fn test_crc_on(&self) -> bool {
        self.crc_on() != 0
    }

    #[doc="Sets the CRC_ON field."]
    #[inline] pub fn set_crc_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn crc_auto_clear_off(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CRC_AUTO_CLEAR_OFF != 0"]
    #[inline] pub fn test_crc_auto_clear_off(&self) -> bool {
        self.crc_auto_clear_off() != 0
    }

    #[doc="Sets the CRC_AUTO_CLEAR_OFF field."]
    #[inline] pub fn set_crc_auto_clear_off<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn address_filtering(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if ADDRESS_FILTERING != 0"]
    #[inline] pub fn test_address_filtering(&self) -> bool {
        self.address_filtering() != 0
    }

    #[doc="Sets the ADDRESS_FILTERING field."]
    #[inline] pub fn set_address_filtering<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Packetconfig1 {
    #[inline]
    fn from(other: u8) -> Self {
         Packetconfig1(other)
    }
}

impl ::core::fmt::Display for Packetconfig1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Packetconfig1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.packet_format() != 0 { try!(write!(f, " packet_format"))}
        if self.dc_free() != 0 { try!(write!(f, " dc_free=0x{:x}", self.dc_free()))}
        if self.crc_on() != 0 { try!(write!(f, " crc_on"))}
        if self.crc_auto_clear_off() != 0 { try!(write!(f, " crc_auto_clear_off"))}
        if self.address_filtering() != 0 { try!(write!(f, " address_filtering=0x{:x}", self.address_filtering()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Payload length setting"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Payloadlength(pub u8);
impl Payloadlength {
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

impl From<u8> for Payloadlength {
    #[inline]
    fn from(other: u8) -> Self {
         Payloadlength(other)
    }
}

impl ::core::fmt::Display for Payloadlength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Payloadlength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.payload_length() != 0 { try!(write!(f, " payload_length=0x{:x}", self.payload_length()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Node address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nodeadrs(pub u8);
impl Nodeadrs {
    #[inline] pub fn node_address(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if NODE_ADDRESS != 0"]
    #[inline] pub fn test_node_address(&self) -> bool {
        self.node_address() != 0
    }

    #[doc="Sets the NODE_ADDRESS field."]
    #[inline] pub fn set_node_address<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Nodeadrs {
    #[inline]
    fn from(other: u8) -> Self {
         Nodeadrs(other)
    }
}

impl ::core::fmt::Display for Nodeadrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Nodeadrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.node_address() != 0 { try!(write!(f, " node_address=0x{:x}", self.node_address()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Broadcast address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Broadcastadrs(pub u8);
impl Broadcastadrs {
    #[inline] pub fn broadcast_address(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BROADCAST_ADDRESS != 0"]
    #[inline] pub fn test_broadcast_address(&self) -> bool {
        self.broadcast_address() != 0
    }

    #[doc="Sets the BROADCAST_ADDRESS field."]
    #[inline] pub fn set_broadcast_address<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Broadcastadrs {
    #[inline]
    fn from(other: u8) -> Self {
         Broadcastadrs(other)
    }
}

impl ::core::fmt::Display for Broadcastadrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Broadcastadrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.broadcast_address() != 0 { try!(write!(f, " broadcast_address=0x{:x}", self.broadcast_address()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Auto modes settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Automodes(pub u8);
impl Automodes {
    #[inline] pub fn enter_condition(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if ENTER_CONDITION != 0"]
    #[inline] pub fn test_enter_condition(&self) -> bool {
        self.enter_condition() != 0
    }

    #[doc="Sets the ENTER_CONDITION field."]
    #[inline] pub fn set_enter_condition<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[inline] pub fn exit_condition(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x7) as u8) } // [4:2]
    }

    #[doc="Returns true if EXIT_CONDITION != 0"]
    #[inline] pub fn test_exit_condition(&self) -> bool {
        self.exit_condition() != 0
    }

    #[doc="Sets the EXIT_CONDITION field."]
    #[inline] pub fn set_exit_condition<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn intermediate_mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if INTERMEDIATE_MODE != 0"]
    #[inline] pub fn test_intermediate_mode(&self) -> bool {
        self.intermediate_mode() != 0
    }

    #[doc="Sets the INTERMEDIATE_MODE field."]
    #[inline] pub fn set_intermediate_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Automodes {
    #[inline]
    fn from(other: u8) -> Self {
         Automodes(other)
    }
}

impl ::core::fmt::Display for Automodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Automodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enter_condition() != 0 { try!(write!(f, " enter_condition=0x{:x}", self.enter_condition()))}
        if self.exit_condition() != 0 { try!(write!(f, " exit_condition=0x{:x}", self.exit_condition()))}
        if self.intermediate_mode() != 0 { try!(write!(f, " intermediate_mode=0x{:x}", self.intermediate_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fifo threshold, Tx start condition"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifothresh(pub u8);
impl Fifothresh {
    #[inline] pub fn tx_start_condition(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TX_START_CONDITION != 0"]
    #[inline] pub fn test_tx_start_condition(&self) -> bool {
        self.tx_start_condition() != 0
    }

    #[doc="Sets the TX_START_CONDITION field."]
    #[inline] pub fn set_tx_start_condition<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[inline] pub fn fifo_threshold(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if FIFO_THRESHOLD != 0"]
    #[inline] pub fn test_fifo_threshold(&self) -> bool {
        self.fifo_threshold() != 0
    }

    #[doc="Sets the FIFO_THRESHOLD field."]
    #[inline] pub fn set_fifo_threshold<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fifothresh {
    #[inline]
    fn from(other: u8) -> Self {
         Fifothresh(other)
    }
}

impl ::core::fmt::Display for Fifothresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifothresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx_start_condition() != 0 { try!(write!(f, " tx_start_condition"))}
        if self.fifo_threshold() != 0 { try!(write!(f, " fifo_threshold=0x{:x}", self.fifo_threshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Packet mode settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Packetconfig2(pub u8);
impl Packetconfig2 {
    #[inline] pub fn inter_packet_rx_delay(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if INTER_PACKET_RX_DELAY != 0"]
    #[inline] pub fn test_inter_packet_rx_delay(&self) -> bool {
        self.inter_packet_rx_delay() != 0
    }

    #[doc="Sets the INTER_PACKET_RX_DELAY field."]
    #[inline] pub fn set_inter_packet_rx_delay<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[inline] pub fn restart_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RESTART_RX != 0"]
    #[inline] pub fn test_restart_rx(&self) -> bool {
        self.restart_rx() != 0
    }

    #[doc="Sets the RESTART_RX field."]
    #[inline] pub fn set_restart_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[inline] pub fn auto_rx_restart_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if AUTO_RX_RESTART_ON != 0"]
    #[inline] pub fn test_auto_rx_restart_on(&self) -> bool {
        self.auto_rx_restart_on() != 0
    }

    #[doc="Sets the AUTO_RX_RESTART_ON field."]
    #[inline] pub fn set_auto_rx_restart_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[inline] pub fn aes_on(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if AES_ON != 0"]
    #[inline] pub fn test_aes_on(&self) -> bool {
        self.aes_on() != 0
    }

    #[doc="Sets the AES_ON field."]
    #[inline] pub fn set_aes_on<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Packetconfig2 {
    #[inline]
    fn from(other: u8) -> Self {
         Packetconfig2(other)
    }
}

impl ::core::fmt::Display for Packetconfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Packetconfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inter_packet_rx_delay() != 0 { try!(write!(f, " inter_packet_rx_delay=0x{:x}", self.inter_packet_rx_delay()))}
        if self.restart_rx() != 0 { try!(write!(f, " restart_rx"))}
        if self.auto_rx_restart_on() != 0 { try!(write!(f, " auto_rx_restart_on"))}
        if self.aes_on() != 0 { try!(write!(f, " aes_on"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="16 bytes of the cypher key"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aeskey(pub u8);
impl Aeskey {
    #[inline] pub fn aes_key(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AES_KEY != 0"]
    #[inline] pub fn test_aes_key(&self) -> bool {
        self.aes_key() != 0
    }

    #[doc="Sets the AES_KEY field."]
    #[inline] pub fn set_aes_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Aeskey {
    #[inline]
    fn from(other: u8) -> Self {
         Aeskey(other)
    }
}

impl ::core::fmt::Display for Aeskey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aeskey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aes_key() != 0 { try!(write!(f, " aes_key=0x{:x}", self.aes_key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Temperature Sensor control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Temp1(pub u8);
impl Temp1 {
    #[inline] pub fn temp_meas_start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TEMP_MEAS_START != 0"]
    #[inline] pub fn test_temp_meas_start(&self) -> bool {
        self.temp_meas_start() != 0
    }

    #[doc="Sets the TEMP_MEAS_START field."]
    #[inline] pub fn set_temp_meas_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[inline] pub fn temp_meas_running(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TEMP_MEAS_RUNNING != 0"]
    #[inline] pub fn test_temp_meas_running(&self) -> bool {
        self.temp_meas_running() != 0
    }

    #[doc="Sets the TEMP_MEAS_RUNNING field."]
    #[inline] pub fn set_temp_meas_running<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Temp1 {
    #[inline]
    fn from(other: u8) -> Self {
         Temp1(other)
    }
}

impl ::core::fmt::Display for Temp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Temp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.temp_meas_start() != 0 { try!(write!(f, " temp_meas_start"))}
        if self.temp_meas_running() != 0 { try!(write!(f, " temp_meas_running"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Temperature readout"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Temp2(pub u8);
impl Temp2 {
    #[inline] pub fn temp_value(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TEMP_VALUE != 0"]
    #[inline] pub fn test_temp_value(&self) -> bool {
        self.temp_value() != 0
    }

    #[doc="Sets the TEMP_VALUE field."]
    #[inline] pub fn set_temp_value<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Temp2 {
    #[inline]
    fn from(other: u8) -> Self {
         Temp2(other)
    }
}

impl ::core::fmt::Display for Temp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Temp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.temp_value() != 0 { try!(write!(f, " temp_value=0x{:x}", self.temp_value()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sensitivity boost"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Testlna(pub u8);
impl Testlna {
    #[inline] pub fn sensitivity_boost(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SENSITIVITY_BOOST != 0"]
    #[inline] pub fn test_sensitivity_boost(&self) -> bool {
        self.sensitivity_boost() != 0
    }

    #[doc="Sets the SENSITIVITY_BOOST field."]
    #[inline] pub fn set_sensitivity_boost<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Testlna {
    #[inline]
    fn from(other: u8) -> Self {
         Testlna(other)
    }
}

impl ::core::fmt::Display for Testlna {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Testlna {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sensitivity_boost() != 0 { try!(write!(f, " sensitivity_boost=0x{:x}", self.sensitivity_boost()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Power PA settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Testpa1(pub u8);
impl Testpa1 {
    #[inline] pub fn pa20dbm1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PA20DBM1 != 0"]
    #[inline] pub fn test_pa20dbm1(&self) -> bool {
        self.pa20dbm1() != 0
    }

    #[doc="Sets the PA20DBM1 field."]
    #[inline] pub fn set_pa20dbm1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Testpa1 {
    #[inline]
    fn from(other: u8) -> Self {
         Testpa1(other)
    }
}

impl ::core::fmt::Display for Testpa1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Testpa1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa20dbm1() != 0 { try!(write!(f, " pa20dbm1=0x{:x}", self.pa20dbm1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="High Power PA settings"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Testpa2(pub u8);
impl Testpa2 {
    #[inline] pub fn pa20dbm2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PA20DBM2 != 0"]
    #[inline] pub fn test_pa20dbm2(&self) -> bool {
        self.pa20dbm2() != 0
    }

    #[doc="Sets the PA20DBM2 field."]
    #[inline] pub fn set_pa20dbm2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Testpa2 {
    #[inline]
    fn from(other: u8) -> Self {
         Testpa2(other)
    }
}

impl ::core::fmt::Display for Testpa2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Testpa2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pa20dbm2() != 0 { try!(write!(f, " pa20dbm2=0x{:x}", self.pa20dbm2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fading Margin Improvement"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Testdagc(pub u8);
impl Testdagc {
    #[inline] pub fn continuous_dagc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CONTINUOUS_DAGC != 0"]
    #[inline] pub fn test_continuous_dagc(&self) -> bool {
        self.continuous_dagc() != 0
    }

    #[doc="Sets the CONTINUOUS_DAGC field."]
    #[inline] pub fn set_continuous_dagc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Testdagc {
    #[inline]
    fn from(other: u8) -> Self {
         Testdagc(other)
    }
}

impl ::core::fmt::Display for Testdagc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Testdagc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.continuous_dagc() != 0 { try!(write!(f, " continuous_dagc=0x{:x}", self.continuous_dagc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AFC offset for low modulation index AFC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Testafc(pub u8);
impl Testafc {
    #[inline] pub fn low_beta_afc_offset(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LOW_BETA_AFC_OFFSET != 0"]
    #[inline] pub fn test_low_beta_afc_offset(&self) -> bool {
        self.low_beta_afc_offset() != 0
    }

    #[doc="Sets the LOW_BETA_AFC_OFFSET field."]
    #[inline] pub fn set_low_beta_afc_offset<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Testafc {
    #[inline]
    fn from(other: u8) -> Self {
         Testafc(other)
    }
}

impl ::core::fmt::Display for Testafc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Testafc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.low_beta_afc_offset() != 0 { try!(write!(f, " low_beta_afc_offset=0x{:x}", self.low_beta_afc_offset()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


}

pub mod ext;
pub use ext::*;
