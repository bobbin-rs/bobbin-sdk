#![no_std]

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

    pub fn datamodul(&self) -> reg::Datamodul {
        reg::Datamodul(self.read(addr::REG_DATAMODUL))
    }
    pub fn set_datamodul(&self, value: reg::Datamodul) {
        self.write(addr::REG_DATAMODUL, value.0)
    }
    pub fn with_datamodul<F: FnOnce(reg::Datamodul) -> reg::Datamodul>(&self, f: F) {
        let tmp = reg::Datamodul(self.read(addr::REG_DATAMODUL));
        self.write(addr::REG_DATAMODUL, f(tmp).0)
    }

    pub fn bitratemsb(&self) -> reg::Bitratemsb {
        reg::Bitratemsb(self.read(addr::REG_BITRATEMSB))
    }
    pub fn set_bitratemsb(&self, value: reg::Bitratemsb) {
        self.write(addr::REG_BITRATEMSB, value.0)
    }
    pub fn with_bitratemsb<F: FnOnce(reg::Bitratemsb) -> reg::Bitratemsb>(&self, f: F) {
        let tmp = reg::Bitratemsb(self.read(addr::REG_BITRATEMSB));
        self.write(addr::REG_BITRATEMSB, f(tmp).0)
    }

    pub fn bitratelsb(&self) -> reg::Bitratelsb {
        reg::Bitratelsb(self.read(addr::REG_BITRATELSB))
    }
    pub fn set_bitratelsb(&self, value: reg::Bitratelsb) {
        self.write(addr::REG_BITRATELSB, value.0)
    }
    pub fn with_bitratelsb<F: FnOnce(reg::Bitratelsb) -> reg::Bitratelsb>(&self, f: F) {
        let tmp = reg::Bitratelsb(self.read(addr::REG_BITRATELSB));
        self.write(addr::REG_BITRATELSB, f(tmp).0)
    }

    pub fn fdevmsb(&self) -> reg::Fdevmsb {
        reg::Fdevmsb(self.read(addr::REG_FDEVMSB))
    }
    pub fn set_fdevmsb(&self, value: reg::Fdevmsb) {
        self.write(addr::REG_FDEVMSB, value.0)
    }
    pub fn with_fdevmsb<F: FnOnce(reg::Fdevmsb) -> reg::Fdevmsb>(&self, f: F) {
        let tmp = reg::Fdevmsb(self.read(addr::REG_FDEVMSB));
        self.write(addr::REG_FDEVMSB, f(tmp).0)
    }

    pub fn fdevlsb(&self) -> reg::Fdevlsb {
        reg::Fdevlsb(self.read(addr::REG_FDEVLSB))
    }
    pub fn set_fdevlsb(&self, value: reg::Fdevlsb) {
        self.write(addr::REG_FDEVLSB, value.0)
    }
    pub fn with_fdevlsb<F: FnOnce(reg::Fdevlsb) -> reg::Fdevlsb>(&self, f: F) {
        let tmp = reg::Fdevlsb(self.read(addr::REG_FDEVLSB));
        self.write(addr::REG_FDEVLSB, f(tmp).0)
    }

    pub fn frfmsb(&self) -> reg::Frfmsb {
        reg::Frfmsb(self.read(addr::REG_FRFMSB))
    }
    pub fn set_frfmsb(&self, value: reg::Frfmsb) {
        self.write(addr::REG_FRFMSB, value.0)
    }
    pub fn with_frfmsb<F: FnOnce(reg::Frfmsb) -> reg::Frfmsb>(&self, f: F) {
        let tmp = reg::Frfmsb(self.read(addr::REG_FRFMSB));
        self.write(addr::REG_FRFMSB, f(tmp).0)
    }

    pub fn frfmid(&self) -> reg::Frfmid {
        reg::Frfmid(self.read(addr::REG_FRFMID))
    }
    pub fn set_frfmid(&self, value: reg::Frfmid) {
        self.write(addr::REG_FRFMID, value.0)
    }
    pub fn with_frfmid<F: FnOnce(reg::Frfmid) -> reg::Frfmid>(&self, f: F) {
        let tmp = reg::Frfmid(self.read(addr::REG_FRFMID));
        self.write(addr::REG_FRFMID, f(tmp).0)
    }

    pub fn frflsb(&self) -> reg::Frflsb {
        reg::Frflsb(self.read(addr::REG_FRFLSB))
    }
    pub fn set_frflsb(&self, value: reg::Frflsb) {
        self.write(addr::REG_FRFLSB, value.0)
    }
    pub fn with_frflsb<F: FnOnce(reg::Frflsb) -> reg::Frflsb>(&self, f: F) {
        let tmp = reg::Frflsb(self.read(addr::REG_FRFLSB));
        self.write(addr::REG_FRFLSB, f(tmp).0)
    }

    pub fn osc1(&self) -> reg::Osc1 {
        reg::Osc1(self.read(addr::REG_OSC1))
    }
    pub fn set_osc1(&self, value: reg::Osc1) {
        self.write(addr::REG_OSC1, value.0)
    }
    pub fn with_osc1<F: FnOnce(reg::Osc1) -> reg::Osc1>(&self, f: F) {
        let tmp = reg::Osc1(self.read(addr::REG_OSC1));
        self.write(addr::REG_OSC1, f(tmp).0)
    }

    pub fn afcctrl(&self) -> reg::Afcctrl {
        reg::Afcctrl(self.read(addr::REG_AFCCTRL))
    }
    pub fn set_afcctrl(&self, value: reg::Afcctrl) {
        self.write(addr::REG_AFCCTRL, value.0)
    }
    pub fn with_afcctrl<F: FnOnce(reg::Afcctrl) -> reg::Afcctrl>(&self, f: F) {
        let tmp = reg::Afcctrl(self.read(addr::REG_AFCCTRL));
        self.write(addr::REG_AFCCTRL, f(tmp).0)
    }

    pub fn listen1(&self) -> reg::Listen1 {
        reg::Listen1(self.read(addr::REG_LISTEN1))
    }
    pub fn set_listen1(&self, value: reg::Listen1) {
        self.write(addr::REG_LISTEN1, value.0)
    }
    pub fn with_listen1<F: FnOnce(reg::Listen1) -> reg::Listen1>(&self, f: F) {
        let tmp = reg::Listen1(self.read(addr::REG_LISTEN1));
        self.write(addr::REG_LISTEN1, f(tmp).0)
    }

    pub fn listen2(&self) -> reg::Listen2 {
        reg::Listen2(self.read(addr::REG_LISTEN2))
    }
    pub fn set_listen2(&self, value: reg::Listen2) {
        self.write(addr::REG_LISTEN2, value.0)
    }
    pub fn with_listen2<F: FnOnce(reg::Listen2) -> reg::Listen2>(&self, f: F) {
        let tmp = reg::Listen2(self.read(addr::REG_LISTEN2));
        self.write(addr::REG_LISTEN2, f(tmp).0)
    }

    pub fn listen3(&self) -> reg::Listen3 {
        reg::Listen3(self.read(addr::REG_LISTEN3))
    }
    pub fn set_listen3(&self, value: reg::Listen3) {
        self.write(addr::REG_LISTEN3, value.0)
    }
    pub fn with_listen3<F: FnOnce(reg::Listen3) -> reg::Listen3>(&self, f: F) {
        let tmp = reg::Listen3(self.read(addr::REG_LISTEN3));
        self.write(addr::REG_LISTEN3, f(tmp).0)
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

    pub fn palevel(&self) -> reg::Palevel {
        reg::Palevel(self.read(addr::REG_PALEVEL))
    }
    pub fn set_palevel(&self, value: reg::Palevel) {
        self.write(addr::REG_PALEVEL, value.0)
    }
    pub fn with_palevel<F: FnOnce(reg::Palevel) -> reg::Palevel>(&self, f: F) {
        let tmp = reg::Palevel(self.read(addr::REG_PALEVEL));
        self.write(addr::REG_PALEVEL, f(tmp).0)
    }

    pub fn paramp(&self) -> reg::Paramp {
        reg::Paramp(self.read(addr::REG_PARAMP))
    }
    pub fn set_paramp(&self, value: reg::Paramp) {
        self.write(addr::REG_PARAMP, value.0)
    }
    pub fn with_paramp<F: FnOnce(reg::Paramp) -> reg::Paramp>(&self, f: F) {
        let tmp = reg::Paramp(self.read(addr::REG_PARAMP));
        self.write(addr::REG_PARAMP, f(tmp).0)
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

    pub fn rxbw(&self) -> reg::Rxbw {
        reg::Rxbw(self.read(addr::REG_RXBW))
    }
    pub fn set_rxbw(&self, value: reg::Rxbw) {
        self.write(addr::REG_RXBW, value.0)
    }
    pub fn with_rxbw<F: FnOnce(reg::Rxbw) -> reg::Rxbw>(&self, f: F) {
        let tmp = reg::Rxbw(self.read(addr::REG_RXBW));
        self.write(addr::REG_RXBW, f(tmp).0)
    }

    pub fn afcbw(&self) -> reg::Afcbw {
        reg::Afcbw(self.read(addr::REG_AFCBW))
    }
    pub fn set_afcbw(&self, value: reg::Afcbw) {
        self.write(addr::REG_AFCBW, value.0)
    }
    pub fn with_afcbw<F: FnOnce(reg::Afcbw) -> reg::Afcbw>(&self, f: F) {
        let tmp = reg::Afcbw(self.read(addr::REG_AFCBW));
        self.write(addr::REG_AFCBW, f(tmp).0)
    }

    pub fn ookpeak(&self) -> reg::Ookpeak {
        reg::Ookpeak(self.read(addr::REG_OOKPEAK))
    }
    pub fn set_ookpeak(&self, value: reg::Ookpeak) {
        self.write(addr::REG_OOKPEAK, value.0)
    }
    pub fn with_ookpeak<F: FnOnce(reg::Ookpeak) -> reg::Ookpeak>(&self, f: F) {
        let tmp = reg::Ookpeak(self.read(addr::REG_OOKPEAK));
        self.write(addr::REG_OOKPEAK, f(tmp).0)
    }

    pub fn ookavg(&self) -> reg::Ookavg {
        reg::Ookavg(self.read(addr::REG_OOKAVG))
    }
    pub fn set_ookavg(&self, value: reg::Ookavg) {
        self.write(addr::REG_OOKAVG, value.0)
    }
    pub fn with_ookavg<F: FnOnce(reg::Ookavg) -> reg::Ookavg>(&self, f: F) {
        let tmp = reg::Ookavg(self.read(addr::REG_OOKAVG));
        self.write(addr::REG_OOKAVG, f(tmp).0)
    }

    pub fn ookfix(&self) -> reg::Ookfix {
        reg::Ookfix(self.read(addr::REG_OOKFIX))
    }
    pub fn set_ookfix(&self, value: reg::Ookfix) {
        self.write(addr::REG_OOKFIX, value.0)
    }
    pub fn with_ookfix<F: FnOnce(reg::Ookfix) -> reg::Ookfix>(&self, f: F) {
        let tmp = reg::Ookfix(self.read(addr::REG_OOKFIX));
        self.write(addr::REG_OOKFIX, f(tmp).0)
    }

    pub fn afcfei(&self) -> reg::Afcfei {
        reg::Afcfei(self.read(addr::REG_AFCFEI))
    }
    pub fn set_afcfei(&self, value: reg::Afcfei) {
        self.write(addr::REG_AFCFEI, value.0)
    }
    pub fn with_afcfei<F: FnOnce(reg::Afcfei) -> reg::Afcfei>(&self, f: F) {
        let tmp = reg::Afcfei(self.read(addr::REG_AFCFEI));
        self.write(addr::REG_AFCFEI, f(tmp).0)
    }

    pub fn afcmsb(&self) -> reg::Afcmsb {
        reg::Afcmsb(self.read(addr::REG_AFCMSB))
    }
    pub fn set_afcmsb(&self, value: reg::Afcmsb) {
        self.write(addr::REG_AFCMSB, value.0)
    }
    pub fn with_afcmsb<F: FnOnce(reg::Afcmsb) -> reg::Afcmsb>(&self, f: F) {
        let tmp = reg::Afcmsb(self.read(addr::REG_AFCMSB));
        self.write(addr::REG_AFCMSB, f(tmp).0)
    }

    pub fn afclsb(&self) -> reg::Afclsb {
        reg::Afclsb(self.read(addr::REG_AFCLSB))
    }
    pub fn set_afclsb(&self, value: reg::Afclsb) {
        self.write(addr::REG_AFCLSB, value.0)
    }
    pub fn with_afclsb<F: FnOnce(reg::Afclsb) -> reg::Afclsb>(&self, f: F) {
        let tmp = reg::Afclsb(self.read(addr::REG_AFCLSB));
        self.write(addr::REG_AFCLSB, f(tmp).0)
    }

    pub fn feimsb(&self) -> reg::Feimsb {
        reg::Feimsb(self.read(addr::REG_FEIMSB))
    }
    pub fn set_feimsb(&self, value: reg::Feimsb) {
        self.write(addr::REG_FEIMSB, value.0)
    }
    pub fn with_feimsb<F: FnOnce(reg::Feimsb) -> reg::Feimsb>(&self, f: F) {
        let tmp = reg::Feimsb(self.read(addr::REG_FEIMSB));
        self.write(addr::REG_FEIMSB, f(tmp).0)
    }

    pub fn feilsb(&self) -> reg::Feilsb {
        reg::Feilsb(self.read(addr::REG_FEILSB))
    }
    pub fn set_feilsb(&self, value: reg::Feilsb) {
        self.write(addr::REG_FEILSB, value.0)
    }
    pub fn with_feilsb<F: FnOnce(reg::Feilsb) -> reg::Feilsb>(&self, f: F) {
        let tmp = reg::Feilsb(self.read(addr::REG_FEILSB));
        self.write(addr::REG_FEILSB, f(tmp).0)
    }

    pub fn rssiconfig(&self) -> reg::Rssiconfig {
        reg::Rssiconfig(self.read(addr::REG_RSSICONFIG))
    }
    pub fn set_rssiconfig(&self, value: reg::Rssiconfig) {
        self.write(addr::REG_RSSICONFIG, value.0)
    }
    pub fn with_rssiconfig<F: FnOnce(reg::Rssiconfig) -> reg::Rssiconfig>(&self, f: F) {
        let tmp = reg::Rssiconfig(self.read(addr::REG_RSSICONFIG));
        self.write(addr::REG_RSSICONFIG, f(tmp).0)
    }

    pub fn rssivalue(&self) -> reg::Rssivalue {
        reg::Rssivalue(self.read(addr::REG_RSSIVALUE))
    }
    pub fn set_rssivalue(&self, value: reg::Rssivalue) {
        self.write(addr::REG_RSSIVALUE, value.0)
    }
    pub fn with_rssivalue<F: FnOnce(reg::Rssivalue) -> reg::Rssivalue>(&self, f: F) {
        let tmp = reg::Rssivalue(self.read(addr::REG_RSSIVALUE));
        self.write(addr::REG_RSSIVALUE, f(tmp).0)
    }

    pub fn diomapping1(&self) -> reg::Diomapping1 {
        reg::Diomapping1(self.read(addr::REG_DIOMAPPING1))
    }
    pub fn set_diomapping1(&self, value: reg::Diomapping1) {
        self.write(addr::REG_DIOMAPPING1, value.0)
    }
    pub fn with_diomapping1<F: FnOnce(reg::Diomapping1) -> reg::Diomapping1>(&self, f: F) {
        let tmp = reg::Diomapping1(self.read(addr::REG_DIOMAPPING1));
        self.write(addr::REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn diomapping2(&self) -> reg::Diomapping2 {
        reg::Diomapping2(self.read(addr::REG_DIOMAPPING2))
    }
    pub fn set_diomapping2(&self, value: reg::Diomapping2) {
        self.write(addr::REG_DIOMAPPING2, value.0)
    }
    pub fn with_diomapping2<F: FnOnce(reg::Diomapping2) -> reg::Diomapping2>(&self, f: F) {
        let tmp = reg::Diomapping2(self.read(addr::REG_DIOMAPPING2));
        self.write(addr::REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn irqflags1(&self) -> reg::Irqflags1 {
        reg::Irqflags1(self.read(addr::REG_IRQFLAGS1))
    }
    pub fn set_irqflags1(&self, value: reg::Irqflags1) {
        self.write(addr::REG_IRQFLAGS1, value.0)
    }
    pub fn with_irqflags1<F: FnOnce(reg::Irqflags1) -> reg::Irqflags1>(&self, f: F) {
        let tmp = reg::Irqflags1(self.read(addr::REG_IRQFLAGS1));
        self.write(addr::REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn irqflags2(&self) -> reg::Irqflags2 {
        reg::Irqflags2(self.read(addr::REG_IRQFLAGS2))
    }
    pub fn set_irqflags2(&self, value: reg::Irqflags2) {
        self.write(addr::REG_IRQFLAGS2, value.0)
    }
    pub fn with_irqflags2<F: FnOnce(reg::Irqflags2) -> reg::Irqflags2>(&self, f: F) {
        let tmp = reg::Irqflags2(self.read(addr::REG_IRQFLAGS2));
        self.write(addr::REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn rssithresh(&self) -> reg::Rssithresh {
        reg::Rssithresh(self.read(addr::REG_RSSITHRESH))
    }
    pub fn set_rssithresh(&self, value: reg::Rssithresh) {
        self.write(addr::REG_RSSITHRESH, value.0)
    }
    pub fn with_rssithresh<F: FnOnce(reg::Rssithresh) -> reg::Rssithresh>(&self, f: F) {
        let tmp = reg::Rssithresh(self.read(addr::REG_RSSITHRESH));
        self.write(addr::REG_RSSITHRESH, f(tmp).0)
    }

    pub fn rxtimeout1(&self) -> reg::Rxtimeout1 {
        reg::Rxtimeout1(self.read(addr::REG_RXTIMEOUT1))
    }
    pub fn set_rxtimeout1(&self, value: reg::Rxtimeout1) {
        self.write(addr::REG_RXTIMEOUT1, value.0)
    }
    pub fn with_rxtimeout1<F: FnOnce(reg::Rxtimeout1) -> reg::Rxtimeout1>(&self, f: F) {
        let tmp = reg::Rxtimeout1(self.read(addr::REG_RXTIMEOUT1));
        self.write(addr::REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn rxtimeout2(&self) -> reg::Rxtimeout2 {
        reg::Rxtimeout2(self.read(addr::REG_RXTIMEOUT2))
    }
    pub fn set_rxtimeout2(&self, value: reg::Rxtimeout2) {
        self.write(addr::REG_RXTIMEOUT2, value.0)
    }
    pub fn with_rxtimeout2<F: FnOnce(reg::Rxtimeout2) -> reg::Rxtimeout2>(&self, f: F) {
        let tmp = reg::Rxtimeout2(self.read(addr::REG_RXTIMEOUT2));
        self.write(addr::REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn preamblemsb(&self) -> reg::Preamblemsb {
        reg::Preamblemsb(self.read(addr::REG_PREAMBLEMSB))
    }
    pub fn set_preamblemsb(&self, value: reg::Preamblemsb) {
        self.write(addr::REG_PREAMBLEMSB, value.0)
    }
    pub fn with_preamblemsb<F: FnOnce(reg::Preamblemsb) -> reg::Preamblemsb>(&self, f: F) {
        let tmp = reg::Preamblemsb(self.read(addr::REG_PREAMBLEMSB));
        self.write(addr::REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn preamblelsb(&self) -> reg::Preamblelsb {
        reg::Preamblelsb(self.read(addr::REG_PREAMBLELSB))
    }
    pub fn set_preamblelsb(&self, value: reg::Preamblelsb) {
        self.write(addr::REG_PREAMBLELSB, value.0)
    }
    pub fn with_preamblelsb<F: FnOnce(reg::Preamblelsb) -> reg::Preamblelsb>(&self, f: F) {
        let tmp = reg::Preamblelsb(self.read(addr::REG_PREAMBLELSB));
        self.write(addr::REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn syncconfig(&self) -> reg::Syncconfig {
        reg::Syncconfig(self.read(addr::REG_SYNCCONFIG))
    }
    pub fn set_syncconfig(&self, value: reg::Syncconfig) {
        self.write(addr::REG_SYNCCONFIG, value.0)
    }
    pub fn with_syncconfig<F: FnOnce(reg::Syncconfig) -> reg::Syncconfig>(&self, f: F) {
        let tmp = reg::Syncconfig(self.read(addr::REG_SYNCCONFIG));
        self.write(addr::REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn syncvalue(&self, index: usize) -> reg::Syncvalue {
        assert!(index < 8);
        reg::Syncvalue(self.read(addr::REG_SYNCVALUE + index as u8))
    }
    pub fn set_syncvalue(&self, index: usize, value: reg::Syncvalue) {
        assert!(index < 8);
        self.write(addr::REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn with_syncvalue<F: FnOnce(reg::Syncvalue) -> reg::Syncvalue>(&self, index: usize, f: F) {
        assert!(index < 8);
        let tmp = reg::Syncvalue(self.read(addr::REG_SYNCVALUE + index as u8));
        self.write(addr::REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn packetconfig1(&self) -> reg::Packetconfig1 {
        reg::Packetconfig1(self.read(addr::REG_PACKETCONFIG1))
    }
    pub fn set_packetconfig1(&self, value: reg::Packetconfig1) {
        self.write(addr::REG_PACKETCONFIG1, value.0)
    }
    pub fn with_packetconfig1<F: FnOnce(reg::Packetconfig1) -> reg::Packetconfig1>(&self, f: F) {
        let tmp = reg::Packetconfig1(self.read(addr::REG_PACKETCONFIG1));
        self.write(addr::REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn payloadlength(&self) -> reg::Payloadlength {
        reg::Payloadlength(self.read(addr::REG_PAYLOADLENGTH))
    }
    pub fn set_payloadlength(&self, value: reg::Payloadlength) {
        self.write(addr::REG_PAYLOADLENGTH, value.0)
    }
    pub fn with_payloadlength<F: FnOnce(reg::Payloadlength) -> reg::Payloadlength>(&self, f: F) {
        let tmp = reg::Payloadlength(self.read(addr::REG_PAYLOADLENGTH));
        self.write(addr::REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn nodeadrs(&self) -> reg::Nodeadrs {
        reg::Nodeadrs(self.read(addr::REG_NODEADRS))
    }
    pub fn set_nodeadrs(&self, value: reg::Nodeadrs) {
        self.write(addr::REG_NODEADRS, value.0)
    }
    pub fn with_nodeadrs<F: FnOnce(reg::Nodeadrs) -> reg::Nodeadrs>(&self, f: F) {
        let tmp = reg::Nodeadrs(self.read(addr::REG_NODEADRS));
        self.write(addr::REG_NODEADRS, f(tmp).0)
    }

    pub fn broadcastadrs(&self) -> reg::Broadcastadrs {
        reg::Broadcastadrs(self.read(addr::REG_BROADCASTADRS))
    }
    pub fn set_broadcastadrs(&self, value: reg::Broadcastadrs) {
        self.write(addr::REG_BROADCASTADRS, value.0)
    }
    pub fn with_broadcastadrs<F: FnOnce(reg::Broadcastadrs) -> reg::Broadcastadrs>(&self, f: F) {
        let tmp = reg::Broadcastadrs(self.read(addr::REG_BROADCASTADRS));
        self.write(addr::REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn automodes(&self) -> reg::Automodes {
        reg::Automodes(self.read(addr::REG_AUTOMODES))
    }
    pub fn set_automodes(&self, value: reg::Automodes) {
        self.write(addr::REG_AUTOMODES, value.0)
    }
    pub fn with_automodes<F: FnOnce(reg::Automodes) -> reg::Automodes>(&self, f: F) {
        let tmp = reg::Automodes(self.read(addr::REG_AUTOMODES));
        self.write(addr::REG_AUTOMODES, f(tmp).0)
    }

    pub fn fifothresh(&self) -> reg::Fifothresh {
        reg::Fifothresh(self.read(addr::REG_FIFOTHRESH))
    }
    pub fn set_fifothresh(&self, value: reg::Fifothresh) {
        self.write(addr::REG_FIFOTHRESH, value.0)
    }
    pub fn with_fifothresh<F: FnOnce(reg::Fifothresh) -> reg::Fifothresh>(&self, f: F) {
        let tmp = reg::Fifothresh(self.read(addr::REG_FIFOTHRESH));
        self.write(addr::REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn packetconfig2(&self) -> reg::Packetconfig2 {
        reg::Packetconfig2(self.read(addr::REG_PACKETCONFIG2))
    }
    pub fn set_packetconfig2(&self, value: reg::Packetconfig2) {
        self.write(addr::REG_PACKETCONFIG2, value.0)
    }
    pub fn with_packetconfig2<F: FnOnce(reg::Packetconfig2) -> reg::Packetconfig2>(&self, f: F) {
        let tmp = reg::Packetconfig2(self.read(addr::REG_PACKETCONFIG2));
        self.write(addr::REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn aeskey(&self, index: usize) -> reg::Aeskey {
        assert!(index < 16);
        reg::Aeskey(self.read(addr::REG_AESKEY + index as u8))
    }
    pub fn set_aeskey(&self, index: usize, value: reg::Aeskey) {
        assert!(index < 16);
        self.write(addr::REG_AESKEY + index as u8, value.0)
    }
    pub fn with_aeskey<F: FnOnce(reg::Aeskey) -> reg::Aeskey>(&self, index: usize, f: F) {
        assert!(index < 16);
        let tmp = reg::Aeskey(self.read(addr::REG_AESKEY + index as u8));
        self.write(addr::REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn temp1(&self) -> reg::Temp1 {
        reg::Temp1(self.read(addr::REG_TEMP1))
    }
    pub fn set_temp1(&self, value: reg::Temp1) {
        self.write(addr::REG_TEMP1, value.0)
    }
    pub fn with_temp1<F: FnOnce(reg::Temp1) -> reg::Temp1>(&self, f: F) {
        let tmp = reg::Temp1(self.read(addr::REG_TEMP1));
        self.write(addr::REG_TEMP1, f(tmp).0)
    }

    pub fn temp2(&self) -> reg::Temp2 {
        reg::Temp2(self.read(addr::REG_TEMP2))
    }
    pub fn set_temp2(&self, value: reg::Temp2) {
        self.write(addr::REG_TEMP2, value.0)
    }
    pub fn with_temp2<F: FnOnce(reg::Temp2) -> reg::Temp2>(&self, f: F) {
        let tmp = reg::Temp2(self.read(addr::REG_TEMP2));
        self.write(addr::REG_TEMP2, f(tmp).0)
    }

    pub fn testlna(&self) -> reg::Testlna {
        reg::Testlna(self.read(addr::REG_TESTLNA))
    }
    pub fn set_testlna(&self, value: reg::Testlna) {
        self.write(addr::REG_TESTLNA, value.0)
    }
    pub fn with_testlna<F: FnOnce(reg::Testlna) -> reg::Testlna>(&self, f: F) {
        let tmp = reg::Testlna(self.read(addr::REG_TESTLNA));
        self.write(addr::REG_TESTLNA, f(tmp).0)
    }

    pub fn testpa1(&self) -> reg::Testpa1 {
        reg::Testpa1(self.read(addr::REG_TESTPA1))
    }
    pub fn set_testpa1(&self, value: reg::Testpa1) {
        self.write(addr::REG_TESTPA1, value.0)
    }
    pub fn with_testpa1<F: FnOnce(reg::Testpa1) -> reg::Testpa1>(&self, f: F) {
        let tmp = reg::Testpa1(self.read(addr::REG_TESTPA1));
        self.write(addr::REG_TESTPA1, f(tmp).0)
    }

    pub fn testpa2(&self) -> reg::Testpa2 {
        reg::Testpa2(self.read(addr::REG_TESTPA2))
    }
    pub fn set_testpa2(&self, value: reg::Testpa2) {
        self.write(addr::REG_TESTPA2, value.0)
    }
    pub fn with_testpa2<F: FnOnce(reg::Testpa2) -> reg::Testpa2>(&self, f: F) {
        let tmp = reg::Testpa2(self.read(addr::REG_TESTPA2));
        self.write(addr::REG_TESTPA2, f(tmp).0)
    }

    pub fn testdagc(&self) -> reg::Testdagc {
        reg::Testdagc(self.read(addr::REG_TESTDAGC))
    }
    pub fn set_testdagc(&self, value: reg::Testdagc) {
        self.write(addr::REG_TESTDAGC, value.0)
    }
    pub fn with_testdagc<F: FnOnce(reg::Testdagc) -> reg::Testdagc>(&self, f: F) {
        let tmp = reg::Testdagc(self.read(addr::REG_TESTDAGC));
        self.write(addr::REG_TESTDAGC, f(tmp).0)
    }

    pub fn testafc(&self) -> reg::Testafc {
        reg::Testafc(self.read(addr::REG_TESTAFC))
    }
    pub fn set_testafc(&self, value: reg::Testafc) {
        self.write(addr::REG_TESTAFC, value.0)
    }
    pub fn with_testafc<F: FnOnce(reg::Testafc) -> reg::Testafc>(&self, f: F) {
        let tmp = reg::Testafc(self.read(addr::REG_TESTAFC));
        self.write(addr::REG_TESTAFC, f(tmp).0)
    }

}

impl<RW: TryReadWrite> Rfm69<RW> {
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

    pub fn try_datamodul(&self) -> Result<reg::Datamodul, RW::Error> {
        Ok(reg::Datamodul(self.try_read(REG_DATAMODUL)?))
    }
    pub fn try_set_datamodul(&self, value: reg::Datamodul) -> Result<(), RW::Error> {
        self.try_write(REG_DATAMODUL, value.0)
    }
    pub fn try_with_datamodul<F: FnOnce(reg::Datamodul) -> reg::Datamodul>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Datamodul(self.try_read(REG_DATAMODUL)?);
        self.try_write(REG_DATAMODUL, f(tmp).0)
    }

    pub fn try_bitratemsb(&self) -> Result<reg::Bitratemsb, RW::Error> {
        Ok(reg::Bitratemsb(self.try_read(REG_BITRATEMSB)?))
    }
    pub fn try_set_bitratemsb(&self, value: reg::Bitratemsb) -> Result<(), RW::Error> {
        self.try_write(REG_BITRATEMSB, value.0)
    }
    pub fn try_with_bitratemsb<F: FnOnce(reg::Bitratemsb) -> reg::Bitratemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Bitratemsb(self.try_read(REG_BITRATEMSB)?);
        self.try_write(REG_BITRATEMSB, f(tmp).0)
    }

    pub fn try_bitratelsb(&self) -> Result<reg::Bitratelsb, RW::Error> {
        Ok(reg::Bitratelsb(self.try_read(REG_BITRATELSB)?))
    }
    pub fn try_set_bitratelsb(&self, value: reg::Bitratelsb) -> Result<(), RW::Error> {
        self.try_write(REG_BITRATELSB, value.0)
    }
    pub fn try_with_bitratelsb<F: FnOnce(reg::Bitratelsb) -> reg::Bitratelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Bitratelsb(self.try_read(REG_BITRATELSB)?);
        self.try_write(REG_BITRATELSB, f(tmp).0)
    }

    pub fn try_fdevmsb(&self) -> Result<reg::Fdevmsb, RW::Error> {
        Ok(reg::Fdevmsb(self.try_read(REG_FDEVMSB)?))
    }
    pub fn try_set_fdevmsb(&self, value: reg::Fdevmsb) -> Result<(), RW::Error> {
        self.try_write(REG_FDEVMSB, value.0)
    }
    pub fn try_with_fdevmsb<F: FnOnce(reg::Fdevmsb) -> reg::Fdevmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Fdevmsb(self.try_read(REG_FDEVMSB)?);
        self.try_write(REG_FDEVMSB, f(tmp).0)
    }

    pub fn try_fdevlsb(&self) -> Result<reg::Fdevlsb, RW::Error> {
        Ok(reg::Fdevlsb(self.try_read(REG_FDEVLSB)?))
    }
    pub fn try_set_fdevlsb(&self, value: reg::Fdevlsb) -> Result<(), RW::Error> {
        self.try_write(REG_FDEVLSB, value.0)
    }
    pub fn try_with_fdevlsb<F: FnOnce(reg::Fdevlsb) -> reg::Fdevlsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Fdevlsb(self.try_read(REG_FDEVLSB)?);
        self.try_write(REG_FDEVLSB, f(tmp).0)
    }

    pub fn try_frfmsb(&self) -> Result<reg::Frfmsb, RW::Error> {
        Ok(reg::Frfmsb(self.try_read(REG_FRFMSB)?))
    }
    pub fn try_set_frfmsb(&self, value: reg::Frfmsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRFMSB, value.0)
    }
    pub fn try_with_frfmsb<F: FnOnce(reg::Frfmsb) -> reg::Frfmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Frfmsb(self.try_read(REG_FRFMSB)?);
        self.try_write(REG_FRFMSB, f(tmp).0)
    }

    pub fn try_frfmid(&self) -> Result<reg::Frfmid, RW::Error> {
        Ok(reg::Frfmid(self.try_read(REG_FRFMID)?))
    }
    pub fn try_set_frfmid(&self, value: reg::Frfmid) -> Result<(), RW::Error> {
        self.try_write(REG_FRFMID, value.0)
    }
    pub fn try_with_frfmid<F: FnOnce(reg::Frfmid) -> reg::Frfmid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Frfmid(self.try_read(REG_FRFMID)?);
        self.try_write(REG_FRFMID, f(tmp).0)
    }

    pub fn try_frflsb(&self) -> Result<reg::Frflsb, RW::Error> {
        Ok(reg::Frflsb(self.try_read(REG_FRFLSB)?))
    }
    pub fn try_set_frflsb(&self, value: reg::Frflsb) -> Result<(), RW::Error> {
        self.try_write(REG_FRFLSB, value.0)
    }
    pub fn try_with_frflsb<F: FnOnce(reg::Frflsb) -> reg::Frflsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Frflsb(self.try_read(REG_FRFLSB)?);
        self.try_write(REG_FRFLSB, f(tmp).0)
    }

    pub fn try_osc1(&self) -> Result<reg::Osc1, RW::Error> {
        Ok(reg::Osc1(self.try_read(REG_OSC1)?))
    }
    pub fn try_set_osc1(&self, value: reg::Osc1) -> Result<(), RW::Error> {
        self.try_write(REG_OSC1, value.0)
    }
    pub fn try_with_osc1<F: FnOnce(reg::Osc1) -> reg::Osc1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Osc1(self.try_read(REG_OSC1)?);
        self.try_write(REG_OSC1, f(tmp).0)
    }

    pub fn try_afcctrl(&self) -> Result<reg::Afcctrl, RW::Error> {
        Ok(reg::Afcctrl(self.try_read(REG_AFCCTRL)?))
    }
    pub fn try_set_afcctrl(&self, value: reg::Afcctrl) -> Result<(), RW::Error> {
        self.try_write(REG_AFCCTRL, value.0)
    }
    pub fn try_with_afcctrl<F: FnOnce(reg::Afcctrl) -> reg::Afcctrl>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Afcctrl(self.try_read(REG_AFCCTRL)?);
        self.try_write(REG_AFCCTRL, f(tmp).0)
    }

    pub fn try_listen1(&self) -> Result<reg::Listen1, RW::Error> {
        Ok(reg::Listen1(self.try_read(REG_LISTEN1)?))
    }
    pub fn try_set_listen1(&self, value: reg::Listen1) -> Result<(), RW::Error> {
        self.try_write(REG_LISTEN1, value.0)
    }
    pub fn try_with_listen1<F: FnOnce(reg::Listen1) -> reg::Listen1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Listen1(self.try_read(REG_LISTEN1)?);
        self.try_write(REG_LISTEN1, f(tmp).0)
    }

    pub fn try_listen2(&self) -> Result<reg::Listen2, RW::Error> {
        Ok(reg::Listen2(self.try_read(REG_LISTEN2)?))
    }
    pub fn try_set_listen2(&self, value: reg::Listen2) -> Result<(), RW::Error> {
        self.try_write(REG_LISTEN2, value.0)
    }
    pub fn try_with_listen2<F: FnOnce(reg::Listen2) -> reg::Listen2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Listen2(self.try_read(REG_LISTEN2)?);
        self.try_write(REG_LISTEN2, f(tmp).0)
    }

    pub fn try_listen3(&self) -> Result<reg::Listen3, RW::Error> {
        Ok(reg::Listen3(self.try_read(REG_LISTEN3)?))
    }
    pub fn try_set_listen3(&self, value: reg::Listen3) -> Result<(), RW::Error> {
        self.try_write(REG_LISTEN3, value.0)
    }
    pub fn try_with_listen3<F: FnOnce(reg::Listen3) -> reg::Listen3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Listen3(self.try_read(REG_LISTEN3)?);
        self.try_write(REG_LISTEN3, f(tmp).0)
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

    pub fn try_palevel(&self) -> Result<reg::Palevel, RW::Error> {
        Ok(reg::Palevel(self.try_read(REG_PALEVEL)?))
    }
    pub fn try_set_palevel(&self, value: reg::Palevel) -> Result<(), RW::Error> {
        self.try_write(REG_PALEVEL, value.0)
    }
    pub fn try_with_palevel<F: FnOnce(reg::Palevel) -> reg::Palevel>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Palevel(self.try_read(REG_PALEVEL)?);
        self.try_write(REG_PALEVEL, f(tmp).0)
    }

    pub fn try_paramp(&self) -> Result<reg::Paramp, RW::Error> {
        Ok(reg::Paramp(self.try_read(REG_PARAMP)?))
    }
    pub fn try_set_paramp(&self, value: reg::Paramp) -> Result<(), RW::Error> {
        self.try_write(REG_PARAMP, value.0)
    }
    pub fn try_with_paramp<F: FnOnce(reg::Paramp) -> reg::Paramp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Paramp(self.try_read(REG_PARAMP)?);
        self.try_write(REG_PARAMP, f(tmp).0)
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

    pub fn try_rxbw(&self) -> Result<reg::Rxbw, RW::Error> {
        Ok(reg::Rxbw(self.try_read(REG_RXBW)?))
    }
    pub fn try_set_rxbw(&self, value: reg::Rxbw) -> Result<(), RW::Error> {
        self.try_write(REG_RXBW, value.0)
    }
    pub fn try_with_rxbw<F: FnOnce(reg::Rxbw) -> reg::Rxbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rxbw(self.try_read(REG_RXBW)?);
        self.try_write(REG_RXBW, f(tmp).0)
    }

    pub fn try_afcbw(&self) -> Result<reg::Afcbw, RW::Error> {
        Ok(reg::Afcbw(self.try_read(REG_AFCBW)?))
    }
    pub fn try_set_afcbw(&self, value: reg::Afcbw) -> Result<(), RW::Error> {
        self.try_write(REG_AFCBW, value.0)
    }
    pub fn try_with_afcbw<F: FnOnce(reg::Afcbw) -> reg::Afcbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Afcbw(self.try_read(REG_AFCBW)?);
        self.try_write(REG_AFCBW, f(tmp).0)
    }

    pub fn try_ookpeak(&self) -> Result<reg::Ookpeak, RW::Error> {
        Ok(reg::Ookpeak(self.try_read(REG_OOKPEAK)?))
    }
    pub fn try_set_ookpeak(&self, value: reg::Ookpeak) -> Result<(), RW::Error> {
        self.try_write(REG_OOKPEAK, value.0)
    }
    pub fn try_with_ookpeak<F: FnOnce(reg::Ookpeak) -> reg::Ookpeak>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Ookpeak(self.try_read(REG_OOKPEAK)?);
        self.try_write(REG_OOKPEAK, f(tmp).0)
    }

    pub fn try_ookavg(&self) -> Result<reg::Ookavg, RW::Error> {
        Ok(reg::Ookavg(self.try_read(REG_OOKAVG)?))
    }
    pub fn try_set_ookavg(&self, value: reg::Ookavg) -> Result<(), RW::Error> {
        self.try_write(REG_OOKAVG, value.0)
    }
    pub fn try_with_ookavg<F: FnOnce(reg::Ookavg) -> reg::Ookavg>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Ookavg(self.try_read(REG_OOKAVG)?);
        self.try_write(REG_OOKAVG, f(tmp).0)
    }

    pub fn try_ookfix(&self) -> Result<reg::Ookfix, RW::Error> {
        Ok(reg::Ookfix(self.try_read(REG_OOKFIX)?))
    }
    pub fn try_set_ookfix(&self, value: reg::Ookfix) -> Result<(), RW::Error> {
        self.try_write(REG_OOKFIX, value.0)
    }
    pub fn try_with_ookfix<F: FnOnce(reg::Ookfix) -> reg::Ookfix>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Ookfix(self.try_read(REG_OOKFIX)?);
        self.try_write(REG_OOKFIX, f(tmp).0)
    }

    pub fn try_afcfei(&self) -> Result<reg::Afcfei, RW::Error> {
        Ok(reg::Afcfei(self.try_read(REG_AFCFEI)?))
    }
    pub fn try_set_afcfei(&self, value: reg::Afcfei) -> Result<(), RW::Error> {
        self.try_write(REG_AFCFEI, value.0)
    }
    pub fn try_with_afcfei<F: FnOnce(reg::Afcfei) -> reg::Afcfei>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Afcfei(self.try_read(REG_AFCFEI)?);
        self.try_write(REG_AFCFEI, f(tmp).0)
    }

    pub fn try_afcmsb(&self) -> Result<reg::Afcmsb, RW::Error> {
        Ok(reg::Afcmsb(self.try_read(REG_AFCMSB)?))
    }
    pub fn try_set_afcmsb(&self, value: reg::Afcmsb) -> Result<(), RW::Error> {
        self.try_write(REG_AFCMSB, value.0)
    }
    pub fn try_with_afcmsb<F: FnOnce(reg::Afcmsb) -> reg::Afcmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Afcmsb(self.try_read(REG_AFCMSB)?);
        self.try_write(REG_AFCMSB, f(tmp).0)
    }

    pub fn try_afclsb(&self) -> Result<reg::Afclsb, RW::Error> {
        Ok(reg::Afclsb(self.try_read(REG_AFCLSB)?))
    }
    pub fn try_set_afclsb(&self, value: reg::Afclsb) -> Result<(), RW::Error> {
        self.try_write(REG_AFCLSB, value.0)
    }
    pub fn try_with_afclsb<F: FnOnce(reg::Afclsb) -> reg::Afclsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Afclsb(self.try_read(REG_AFCLSB)?);
        self.try_write(REG_AFCLSB, f(tmp).0)
    }

    pub fn try_feimsb(&self) -> Result<reg::Feimsb, RW::Error> {
        Ok(reg::Feimsb(self.try_read(REG_FEIMSB)?))
    }
    pub fn try_set_feimsb(&self, value: reg::Feimsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEIMSB, value.0)
    }
    pub fn try_with_feimsb<F: FnOnce(reg::Feimsb) -> reg::Feimsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Feimsb(self.try_read(REG_FEIMSB)?);
        self.try_write(REG_FEIMSB, f(tmp).0)
    }

    pub fn try_feilsb(&self) -> Result<reg::Feilsb, RW::Error> {
        Ok(reg::Feilsb(self.try_read(REG_FEILSB)?))
    }
    pub fn try_set_feilsb(&self, value: reg::Feilsb) -> Result<(), RW::Error> {
        self.try_write(REG_FEILSB, value.0)
    }
    pub fn try_with_feilsb<F: FnOnce(reg::Feilsb) -> reg::Feilsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Feilsb(self.try_read(REG_FEILSB)?);
        self.try_write(REG_FEILSB, f(tmp).0)
    }

    pub fn try_rssiconfig(&self) -> Result<reg::Rssiconfig, RW::Error> {
        Ok(reg::Rssiconfig(self.try_read(REG_RSSICONFIG)?))
    }
    pub fn try_set_rssiconfig(&self, value: reg::Rssiconfig) -> Result<(), RW::Error> {
        self.try_write(REG_RSSICONFIG, value.0)
    }
    pub fn try_with_rssiconfig<F: FnOnce(reg::Rssiconfig) -> reg::Rssiconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rssiconfig(self.try_read(REG_RSSICONFIG)?);
        self.try_write(REG_RSSICONFIG, f(tmp).0)
    }

    pub fn try_rssivalue(&self) -> Result<reg::Rssivalue, RW::Error> {
        Ok(reg::Rssivalue(self.try_read(REG_RSSIVALUE)?))
    }
    pub fn try_set_rssivalue(&self, value: reg::Rssivalue) -> Result<(), RW::Error> {
        self.try_write(REG_RSSIVALUE, value.0)
    }
    pub fn try_with_rssivalue<F: FnOnce(reg::Rssivalue) -> reg::Rssivalue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rssivalue(self.try_read(REG_RSSIVALUE)?);
        self.try_write(REG_RSSIVALUE, f(tmp).0)
    }

    pub fn try_diomapping1(&self) -> Result<reg::Diomapping1, RW::Error> {
        Ok(reg::Diomapping1(self.try_read(REG_DIOMAPPING1)?))
    }
    pub fn try_set_diomapping1(&self, value: reg::Diomapping1) -> Result<(), RW::Error> {
        self.try_write(REG_DIOMAPPING1, value.0)
    }
    pub fn try_with_diomapping1<F: FnOnce(reg::Diomapping1) -> reg::Diomapping1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Diomapping1(self.try_read(REG_DIOMAPPING1)?);
        self.try_write(REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn try_diomapping2(&self) -> Result<reg::Diomapping2, RW::Error> {
        Ok(reg::Diomapping2(self.try_read(REG_DIOMAPPING2)?))
    }
    pub fn try_set_diomapping2(&self, value: reg::Diomapping2) -> Result<(), RW::Error> {
        self.try_write(REG_DIOMAPPING2, value.0)
    }
    pub fn try_with_diomapping2<F: FnOnce(reg::Diomapping2) -> reg::Diomapping2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Diomapping2(self.try_read(REG_DIOMAPPING2)?);
        self.try_write(REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn try_irqflags1(&self) -> Result<reg::Irqflags1, RW::Error> {
        Ok(reg::Irqflags1(self.try_read(REG_IRQFLAGS1)?))
    }
    pub fn try_set_irqflags1(&self, value: reg::Irqflags1) -> Result<(), RW::Error> {
        self.try_write(REG_IRQFLAGS1, value.0)
    }
    pub fn try_with_irqflags1<F: FnOnce(reg::Irqflags1) -> reg::Irqflags1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Irqflags1(self.try_read(REG_IRQFLAGS1)?);
        self.try_write(REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn try_irqflags2(&self) -> Result<reg::Irqflags2, RW::Error> {
        Ok(reg::Irqflags2(self.try_read(REG_IRQFLAGS2)?))
    }
    pub fn try_set_irqflags2(&self, value: reg::Irqflags2) -> Result<(), RW::Error> {
        self.try_write(REG_IRQFLAGS2, value.0)
    }
    pub fn try_with_irqflags2<F: FnOnce(reg::Irqflags2) -> reg::Irqflags2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Irqflags2(self.try_read(REG_IRQFLAGS2)?);
        self.try_write(REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn try_rssithresh(&self) -> Result<reg::Rssithresh, RW::Error> {
        Ok(reg::Rssithresh(self.try_read(REG_RSSITHRESH)?))
    }
    pub fn try_set_rssithresh(&self, value: reg::Rssithresh) -> Result<(), RW::Error> {
        self.try_write(REG_RSSITHRESH, value.0)
    }
    pub fn try_with_rssithresh<F: FnOnce(reg::Rssithresh) -> reg::Rssithresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rssithresh(self.try_read(REG_RSSITHRESH)?);
        self.try_write(REG_RSSITHRESH, f(tmp).0)
    }

    pub fn try_rxtimeout1(&self) -> Result<reg::Rxtimeout1, RW::Error> {
        Ok(reg::Rxtimeout1(self.try_read(REG_RXTIMEOUT1)?))
    }
    pub fn try_set_rxtimeout1(&self, value: reg::Rxtimeout1) -> Result<(), RW::Error> {
        self.try_write(REG_RXTIMEOUT1, value.0)
    }
    pub fn try_with_rxtimeout1<F: FnOnce(reg::Rxtimeout1) -> reg::Rxtimeout1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rxtimeout1(self.try_read(REG_RXTIMEOUT1)?);
        self.try_write(REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn try_rxtimeout2(&self) -> Result<reg::Rxtimeout2, RW::Error> {
        Ok(reg::Rxtimeout2(self.try_read(REG_RXTIMEOUT2)?))
    }
    pub fn try_set_rxtimeout2(&self, value: reg::Rxtimeout2) -> Result<(), RW::Error> {
        self.try_write(REG_RXTIMEOUT2, value.0)
    }
    pub fn try_with_rxtimeout2<F: FnOnce(reg::Rxtimeout2) -> reg::Rxtimeout2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Rxtimeout2(self.try_read(REG_RXTIMEOUT2)?);
        self.try_write(REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn try_preamblemsb(&self) -> Result<reg::Preamblemsb, RW::Error> {
        Ok(reg::Preamblemsb(self.try_read(REG_PREAMBLEMSB)?))
    }
    pub fn try_set_preamblemsb(&self, value: reg::Preamblemsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLEMSB, value.0)
    }
    pub fn try_with_preamblemsb<F: FnOnce(reg::Preamblemsb) -> reg::Preamblemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Preamblemsb(self.try_read(REG_PREAMBLEMSB)?);
        self.try_write(REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn try_preamblelsb(&self) -> Result<reg::Preamblelsb, RW::Error> {
        Ok(reg::Preamblelsb(self.try_read(REG_PREAMBLELSB)?))
    }
    pub fn try_set_preamblelsb(&self, value: reg::Preamblelsb) -> Result<(), RW::Error> {
        self.try_write(REG_PREAMBLELSB, value.0)
    }
    pub fn try_with_preamblelsb<F: FnOnce(reg::Preamblelsb) -> reg::Preamblelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Preamblelsb(self.try_read(REG_PREAMBLELSB)?);
        self.try_write(REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn try_syncconfig(&self) -> Result<reg::Syncconfig, RW::Error> {
        Ok(reg::Syncconfig(self.try_read(REG_SYNCCONFIG)?))
    }
    pub fn try_set_syncconfig(&self, value: reg::Syncconfig) -> Result<(), RW::Error> {
        self.try_write(REG_SYNCCONFIG, value.0)
    }
    pub fn try_with_syncconfig<F: FnOnce(reg::Syncconfig) -> reg::Syncconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Syncconfig(self.try_read(REG_SYNCCONFIG)?);
        self.try_write(REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn try_syncvalue(&self, index: usize) -> Result<reg::Syncvalue, RW::Error> {
        assert!(index < 8);
        Ok(reg::Syncvalue(self.try_read(REG_SYNCVALUE + index as u8)?))
    }
    pub fn try_set_syncvalue(&self, index: usize, value: reg::Syncvalue) -> Result<(), RW::Error> {
        assert!(index < 8);
        self.try_write(REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn try_with_syncvalue<F: FnOnce(reg::Syncvalue) -> reg::Syncvalue>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 8);
        let tmp = reg::Syncvalue(self.try_read(REG_SYNCVALUE + index as u8)?);
        self.try_write(REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn try_packetconfig1(&self) -> Result<reg::Packetconfig1, RW::Error> {
        Ok(reg::Packetconfig1(self.try_read(REG_PACKETCONFIG1)?))
    }
    pub fn try_set_packetconfig1(&self, value: reg::Packetconfig1) -> Result<(), RW::Error> {
        self.try_write(REG_PACKETCONFIG1, value.0)
    }
    pub fn try_with_packetconfig1<F: FnOnce(reg::Packetconfig1) -> reg::Packetconfig1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Packetconfig1(self.try_read(REG_PACKETCONFIG1)?);
        self.try_write(REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn try_payloadlength(&self) -> Result<reg::Payloadlength, RW::Error> {
        Ok(reg::Payloadlength(self.try_read(REG_PAYLOADLENGTH)?))
    }
    pub fn try_set_payloadlength(&self, value: reg::Payloadlength) -> Result<(), RW::Error> {
        self.try_write(REG_PAYLOADLENGTH, value.0)
    }
    pub fn try_with_payloadlength<F: FnOnce(reg::Payloadlength) -> reg::Payloadlength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Payloadlength(self.try_read(REG_PAYLOADLENGTH)?);
        self.try_write(REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn try_nodeadrs(&self) -> Result<reg::Nodeadrs, RW::Error> {
        Ok(reg::Nodeadrs(self.try_read(REG_NODEADRS)?))
    }
    pub fn try_set_nodeadrs(&self, value: reg::Nodeadrs) -> Result<(), RW::Error> {
        self.try_write(REG_NODEADRS, value.0)
    }
    pub fn try_with_nodeadrs<F: FnOnce(reg::Nodeadrs) -> reg::Nodeadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Nodeadrs(self.try_read(REG_NODEADRS)?);
        self.try_write(REG_NODEADRS, f(tmp).0)
    }

    pub fn try_broadcastadrs(&self) -> Result<reg::Broadcastadrs, RW::Error> {
        Ok(reg::Broadcastadrs(self.try_read(REG_BROADCASTADRS)?))
    }
    pub fn try_set_broadcastadrs(&self, value: reg::Broadcastadrs) -> Result<(), RW::Error> {
        self.try_write(REG_BROADCASTADRS, value.0)
    }
    pub fn try_with_broadcastadrs<F: FnOnce(reg::Broadcastadrs) -> reg::Broadcastadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Broadcastadrs(self.try_read(REG_BROADCASTADRS)?);
        self.try_write(REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn try_automodes(&self) -> Result<reg::Automodes, RW::Error> {
        Ok(reg::Automodes(self.try_read(REG_AUTOMODES)?))
    }
    pub fn try_set_automodes(&self, value: reg::Automodes) -> Result<(), RW::Error> {
        self.try_write(REG_AUTOMODES, value.0)
    }
    pub fn try_with_automodes<F: FnOnce(reg::Automodes) -> reg::Automodes>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Automodes(self.try_read(REG_AUTOMODES)?);
        self.try_write(REG_AUTOMODES, f(tmp).0)
    }

    pub fn try_fifothresh(&self) -> Result<reg::Fifothresh, RW::Error> {
        Ok(reg::Fifothresh(self.try_read(REG_FIFOTHRESH)?))
    }
    pub fn try_set_fifothresh(&self, value: reg::Fifothresh) -> Result<(), RW::Error> {
        self.try_write(REG_FIFOTHRESH, value.0)
    }
    pub fn try_with_fifothresh<F: FnOnce(reg::Fifothresh) -> reg::Fifothresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Fifothresh(self.try_read(REG_FIFOTHRESH)?);
        self.try_write(REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn try_packetconfig2(&self) -> Result<reg::Packetconfig2, RW::Error> {
        Ok(reg::Packetconfig2(self.try_read(REG_PACKETCONFIG2)?))
    }
    pub fn try_set_packetconfig2(&self, value: reg::Packetconfig2) -> Result<(), RW::Error> {
        self.try_write(REG_PACKETCONFIG2, value.0)
    }
    pub fn try_with_packetconfig2<F: FnOnce(reg::Packetconfig2) -> reg::Packetconfig2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Packetconfig2(self.try_read(REG_PACKETCONFIG2)?);
        self.try_write(REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn try_aeskey(&self, index: usize) -> Result<reg::Aeskey, RW::Error> {
        assert!(index < 16);
        Ok(reg::Aeskey(self.try_read(REG_AESKEY + index as u8)?))
    }
    pub fn try_set_aeskey(&self, index: usize, value: reg::Aeskey) -> Result<(), RW::Error> {
        assert!(index < 16);
        self.try_write(REG_AESKEY + index as u8, value.0)
    }
    pub fn try_with_aeskey<F: FnOnce(reg::Aeskey) -> reg::Aeskey>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 16);
        let tmp = reg::Aeskey(self.try_read(REG_AESKEY + index as u8)?);
        self.try_write(REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn try_temp1(&self) -> Result<reg::Temp1, RW::Error> {
        Ok(reg::Temp1(self.try_read(REG_TEMP1)?))
    }
    pub fn try_set_temp1(&self, value: reg::Temp1) -> Result<(), RW::Error> {
        self.try_write(REG_TEMP1, value.0)
    }
    pub fn try_with_temp1<F: FnOnce(reg::Temp1) -> reg::Temp1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Temp1(self.try_read(REG_TEMP1)?);
        self.try_write(REG_TEMP1, f(tmp).0)
    }

    pub fn try_temp2(&self) -> Result<reg::Temp2, RW::Error> {
        Ok(reg::Temp2(self.try_read(REG_TEMP2)?))
    }
    pub fn try_set_temp2(&self, value: reg::Temp2) -> Result<(), RW::Error> {
        self.try_write(REG_TEMP2, value.0)
    }
    pub fn try_with_temp2<F: FnOnce(reg::Temp2) -> reg::Temp2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Temp2(self.try_read(REG_TEMP2)?);
        self.try_write(REG_TEMP2, f(tmp).0)
    }

    pub fn try_testlna(&self) -> Result<reg::Testlna, RW::Error> {
        Ok(reg::Testlna(self.try_read(REG_TESTLNA)?))
    }
    pub fn try_set_testlna(&self, value: reg::Testlna) -> Result<(), RW::Error> {
        self.try_write(REG_TESTLNA, value.0)
    }
    pub fn try_with_testlna<F: FnOnce(reg::Testlna) -> reg::Testlna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Testlna(self.try_read(REG_TESTLNA)?);
        self.try_write(REG_TESTLNA, f(tmp).0)
    }

    pub fn try_testpa1(&self) -> Result<reg::Testpa1, RW::Error> {
        Ok(reg::Testpa1(self.try_read(REG_TESTPA1)?))
    }
    pub fn try_set_testpa1(&self, value: reg::Testpa1) -> Result<(), RW::Error> {
        self.try_write(REG_TESTPA1, value.0)
    }
    pub fn try_with_testpa1<F: FnOnce(reg::Testpa1) -> reg::Testpa1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Testpa1(self.try_read(REG_TESTPA1)?);
        self.try_write(REG_TESTPA1, f(tmp).0)
    }

    pub fn try_testpa2(&self) -> Result<reg::Testpa2, RW::Error> {
        Ok(reg::Testpa2(self.try_read(REG_TESTPA2)?))
    }
    pub fn try_set_testpa2(&self, value: reg::Testpa2) -> Result<(), RW::Error> {
        self.try_write(REG_TESTPA2, value.0)
    }
    pub fn try_with_testpa2<F: FnOnce(reg::Testpa2) -> reg::Testpa2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Testpa2(self.try_read(REG_TESTPA2)?);
        self.try_write(REG_TESTPA2, f(tmp).0)
    }

    pub fn try_testdagc(&self) -> Result<reg::Testdagc, RW::Error> {
        Ok(reg::Testdagc(self.try_read(REG_TESTDAGC)?))
    }
    pub fn try_set_testdagc(&self, value: reg::Testdagc) -> Result<(), RW::Error> {
        self.try_write(REG_TESTDAGC, value.0)
    }
    pub fn try_with_testdagc<F: FnOnce(reg::Testdagc) -> reg::Testdagc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Testdagc(self.try_read(REG_TESTDAGC)?);
        self.try_write(REG_TESTDAGC, f(tmp).0)
    }

    pub fn try_testafc(&self) -> Result<reg::Testafc, RW::Error> {
        Ok(reg::Testafc(self.try_read(REG_TESTAFC)?))
    }
    pub fn try_set_testafc(&self, value: reg::Testafc) -> Result<(), RW::Error> {
        self.try_write(REG_TESTAFC, value.0)
    }
    pub fn try_with_testafc<F: FnOnce(reg::Testafc) -> reg::Testafc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = reg::Testafc(self.try_read(REG_TESTAFC)?);
        self.try_write(REG_TESTAFC, f(tmp).0)
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

        pub fn sequencer_off(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_sequencer_off(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn listen_on(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_listen_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn listen_abort(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_listen_abort(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn mode(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x7 // [4:2]
        }
    
        pub fn set_mode(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 2);
            self.0 |= value << 2;
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
            if self.sequencer_off() != 0 { write!(f, " sequencer_off")? }
            if self.listen_on() != 0 { write!(f, " listen_on")? }
            if self.listen_abort() != 0 { write!(f, " listen_abort")? }
            if self.mode() != 0 { write!(f, " mode=0x{:x}", self.mode())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Datamodul(u8);

    impl From<u8> for Datamodul {
        fn from(other: u8) -> Self { Datamodul(other) }
    }

    impl From<Datamodul> for u8 {
        fn from(other: Datamodul) -> Self { other.0 }
    }

    impl Datamodul {
        pub fn value(&self) -> u8 { self.0 }

        pub fn data_mode(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x3 // [6:5]
        }
    
        pub fn set_data_mode(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn modulation_type(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x3 // [4:3]
        }
    
        pub fn set_modulation_type(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn modulation_shaping(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_modulation_shaping(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Datamodul {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Datamodul {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.data_mode() != 0 { write!(f, " data_mode=0x{:x}", self.data_mode())? }
            if self.modulation_type() != 0 { write!(f, " modulation_type=0x{:x}", self.modulation_type())? }
            if self.modulation_shaping() != 0 { write!(f, " modulation_shaping=0x{:x}", self.modulation_shaping())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Bitratemsb(u8);

    impl From<u8> for Bitratemsb {
        fn from(other: u8) -> Self { Bitratemsb(other) }
    }

    impl From<Bitratemsb> for u8 {
        fn from(other: Bitratemsb) -> Self { other.0 }
    }

    impl Bitratemsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn bitrate(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_bitrate(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Bitratemsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Bitratemsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.bitrate() != 0 { write!(f, " bitrate=0x{:x}", self.bitrate())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Bitratelsb(u8);

    impl From<u8> for Bitratelsb {
        fn from(other: u8) -> Self { Bitratelsb(other) }
    }

    impl From<Bitratelsb> for u8 {
        fn from(other: Bitratelsb) -> Self { other.0 }
    }

    impl Bitratelsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn bitrate(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_bitrate(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Bitratelsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Bitratelsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.bitrate() != 0 { write!(f, " bitrate=0x{:x}", self.bitrate())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Fdevmsb(u8);

    impl From<u8> for Fdevmsb {
        fn from(other: u8) -> Self { Fdevmsb(other) }
    }

    impl From<Fdevmsb> for u8 {
        fn from(other: Fdevmsb) -> Self { other.0 }
    }

    impl Fdevmsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fdev(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1f // [4:0]
        }
    
        pub fn set_fdev(mut self, value: u8) -> Self {
            assert!((value & !0x1f) == 0);
            self.0 &= !(0x1f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Fdevmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Fdevmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fdev() != 0 { write!(f, " fdev=0x{:x}", self.fdev())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Fdevlsb(u8);

    impl From<u8> for Fdevlsb {
        fn from(other: u8) -> Self { Fdevlsb(other) }
    }

    impl From<Fdevlsb> for u8 {
        fn from(other: Fdevlsb) -> Self { other.0 }
    }

    impl Fdevlsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fdev(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fdev(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Fdevlsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Fdevlsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fdev() != 0 { write!(f, " fdev=0x{:x}", self.fdev())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Frfmsb(u8);

    impl From<u8> for Frfmsb {
        fn from(other: u8) -> Self { Frfmsb(other) }
    }

    impl From<Frfmsb> for u8 {
        fn from(other: Frfmsb) -> Self { other.0 }
    }

    impl Frfmsb {
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

    impl ::core::fmt::Display for Frfmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Frfmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Frfmid(u8);

    impl From<u8> for Frfmid {
        fn from(other: u8) -> Self { Frfmid(other) }
    }

    impl From<Frfmid> for u8 {
        fn from(other: Frfmid) -> Self { other.0 }
    }

    impl Frfmid {
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

    impl ::core::fmt::Display for Frfmid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Frfmid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Frflsb(u8);

    impl From<u8> for Frflsb {
        fn from(other: u8) -> Self { Frflsb(other) }
    }

    impl From<Frflsb> for u8 {
        fn from(other: Frflsb) -> Self { other.0 }
    }

    impl Frflsb {
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

    impl ::core::fmt::Display for Frflsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Frflsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.frf() != 0 { write!(f, " frf=0x{:x}", self.frf())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Osc1(u8);

    impl From<u8> for Osc1 {
        fn from(other: u8) -> Self { Osc1(other) }
    }

    impl From<Osc1> for u8 {
        fn from(other: Osc1) -> Self { other.0 }
    }

    impl Osc1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rc_cal_start(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_rc_cal_start(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn rc_cal_done(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_rc_cal_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
    }

    impl ::core::fmt::Display for Osc1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Osc1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rc_cal_start() != 0 { write!(f, " rc_cal_start")? }
            if self.rc_cal_done() != 0 { write!(f, " rc_cal_done")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Afcctrl(u8);

    impl From<u8> for Afcctrl {
        fn from(other: u8) -> Self { Afcctrl(other) }
    }

    impl From<Afcctrl> for u8 {
        fn from(other: Afcctrl) -> Self { other.0 }
    }

    impl Afcctrl {
        pub fn value(&self) -> u8 { self.0 }

        pub fn afc_low_beta_on(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_afc_low_beta_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
    }

    impl ::core::fmt::Display for Afcctrl {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Afcctrl {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.afc_low_beta_on() != 0 { write!(f, " afc_low_beta_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Listen1(u8);

    impl From<u8> for Listen1 {
        fn from(other: u8) -> Self { Listen1(other) }
    }

    impl From<Listen1> for u8 {
        fn from(other: Listen1) -> Self { other.0 }
    }

    impl Listen1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn listen_resol_idle(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_listen_resol_idle(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn listen_resol_rx(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x3 // [5:4]
        }
    
        pub fn set_listen_resol_rx(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn listen_criteria(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_listen_criteria(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn listen_end(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x3 // [2:1]
        }
    
        pub fn set_listen_end(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 1);
            self.0 |= value << 1;
            self
        }
    
    }

    impl ::core::fmt::Display for Listen1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Listen1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.listen_resol_idle() != 0 { write!(f, " listen_resol_idle=0x{:x}", self.listen_resol_idle())? }
            if self.listen_resol_rx() != 0 { write!(f, " listen_resol_rx=0x{:x}", self.listen_resol_rx())? }
            if self.listen_criteria() != 0 { write!(f, " listen_criteria")? }
            if self.listen_end() != 0 { write!(f, " listen_end=0x{:x}", self.listen_end())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Listen2(u8);

    impl From<u8> for Listen2 {
        fn from(other: u8) -> Self { Listen2(other) }
    }

    impl From<Listen2> for u8 {
        fn from(other: Listen2) -> Self { other.0 }
    }

    impl Listen2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn listen_coef_idle(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_listen_coef_idle(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Listen2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Listen2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.listen_coef_idle() != 0 { write!(f, " listen_coef_idle=0x{:x}", self.listen_coef_idle())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Listen3(u8);

    impl From<u8> for Listen3 {
        fn from(other: u8) -> Self { Listen3(other) }
    }

    impl From<Listen3> for u8 {
        fn from(other: Listen3) -> Self { other.0 }
    }

    impl Listen3 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn listen_coef_rx(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_listen_coef_rx(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Listen3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Listen3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.listen_coef_rx() != 0 { write!(f, " listen_coef_rx=0x{:x}", self.listen_coef_rx())? }
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
    pub struct Palevel(u8);

    impl From<u8> for Palevel {
        fn from(other: u8) -> Self { Palevel(other) }
    }

    impl From<Palevel> for u8 {
        fn from(other: Palevel) -> Self { other.0 }
    }

    impl Palevel {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa0_on(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_pa0_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn pa1_on(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_pa1_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn pa2_on(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_pa2_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn output_power(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1f // [4:0]
        }
    
        pub fn set_output_power(mut self, value: u8) -> Self {
            assert!((value & !0x1f) == 0);
            self.0 &= !(0x1f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Palevel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Palevel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa0_on() != 0 { write!(f, " pa0_on")? }
            if self.pa1_on() != 0 { write!(f, " pa1_on")? }
            if self.pa2_on() != 0 { write!(f, " pa2_on")? }
            if self.output_power() != 0 { write!(f, " output_power=0x{:x}", self.output_power())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Paramp(u8);

    impl From<u8> for Paramp {
        fn from(other: u8) -> Self { Paramp(other) }
    }

    impl From<Paramp> for u8 {
        fn from(other: Paramp) -> Self { other.0 }
    }

    impl Paramp {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa_ramo(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_pa_ramo(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Paramp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Paramp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa_ramo() != 0 { write!(f, " pa_ramo=0x{:x}", self.pa_ramo())? }
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
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_ocp_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn ocp_trim(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xf // [3:0]
        }
    
        pub fn set_ocp_trim(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 0);
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

        pub fn lna_zin(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_lna_zin(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn lna_current_gain(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x7 // [5:3]
        }
    
        pub fn set_lna_current_gain(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn lna_gain_select(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_lna_gain_select(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
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
            if self.lna_zin() != 0 { write!(f, " lna_zin")? }
            if self.lna_current_gain() != 0 { write!(f, " lna_current_gain=0x{:x}", self.lna_current_gain())? }
            if self.lna_gain_select() != 0 { write!(f, " lna_gain_select=0x{:x}", self.lna_gain_select())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rxbw(u8);

    impl From<u8> for Rxbw {
        fn from(other: u8) -> Self { Rxbw(other) }
    }

    impl From<Rxbw> for u8 {
        fn from(other: Rxbw) -> Self { other.0 }
    }

    impl Rxbw {
        pub fn value(&self) -> u8 { self.0 }

        pub fn dcc_freq(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x7 // [7:5]
        }
    
        pub fn set_dcc_freq(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn rx_bw_mant(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x3 // [4:3]
        }
    
        pub fn set_rx_bw_mant(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn rx_bw_exp(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_rx_bw_exp(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rxbw {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rxbw {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.dcc_freq() != 0 { write!(f, " dcc_freq=0x{:x}", self.dcc_freq())? }
            if self.rx_bw_mant() != 0 { write!(f, " rx_bw_mant=0x{:x}", self.rx_bw_mant())? }
            if self.rx_bw_exp() != 0 { write!(f, " rx_bw_exp=0x{:x}", self.rx_bw_exp())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Afcbw(u8);

    impl From<u8> for Afcbw {
        fn from(other: u8) -> Self { Afcbw(other) }
    }

    impl From<Afcbw> for u8 {
        fn from(other: Afcbw) -> Self { other.0 }
    }

    impl Afcbw {
        pub fn value(&self) -> u8 { self.0 }

        pub fn dcc_freq_afc(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x7f // [11:5]
        }
    
        pub fn set_dcc_freq_afc(mut self, value: u8) -> Self {
            assert!((value & !0x7f) == 0);
            self.0 &= !(0x7f << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn rx_bw_mant_afc(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x3 // [4:3]
        }
    
        pub fn set_rx_bw_mant_afc(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn rx_bw_exp_afc(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_rx_bw_exp_afc(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Afcbw {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Afcbw {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.dcc_freq_afc() != 0 { write!(f, " dcc_freq_afc=0x{:x}", self.dcc_freq_afc())? }
            if self.rx_bw_mant_afc() != 0 { write!(f, " rx_bw_mant_afc=0x{:x}", self.rx_bw_mant_afc())? }
            if self.rx_bw_exp_afc() != 0 { write!(f, " rx_bw_exp_afc=0x{:x}", self.rx_bw_exp_afc())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Ookpeak(u8);

    impl From<u8> for Ookpeak {
        fn from(other: u8) -> Self { Ookpeak(other) }
    }

    impl From<Ookpeak> for u8 {
        fn from(other: Ookpeak) -> Self { other.0 }
    }

    impl Ookpeak {
        pub fn value(&self) -> u8 { self.0 }

        pub fn ook_thresh_type(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_ook_thresh_type(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn ook_peak_thresh_step(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x7 // [5:3]
        }
    
        pub fn set_ook_peak_thresh_step(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn ook_peak_thresh_dec(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_ook_peak_thresh_dec(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Ookpeak {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Ookpeak {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.ook_thresh_type() != 0 { write!(f, " ook_thresh_type=0x{:x}", self.ook_thresh_type())? }
            if self.ook_peak_thresh_step() != 0 { write!(f, " ook_peak_thresh_step=0x{:x}", self.ook_peak_thresh_step())? }
            if self.ook_peak_thresh_dec() != 0 { write!(f, " ook_peak_thresh_dec=0x{:x}", self.ook_peak_thresh_dec())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Ookavg(u8);

    impl From<u8> for Ookavg {
        fn from(other: u8) -> Self { Ookavg(other) }
    }

    impl From<Ookavg> for u8 {
        fn from(other: Ookavg) -> Self { other.0 }
    }

    impl Ookavg {
        pub fn value(&self) -> u8 { self.0 }

        pub fn ook_average_thresh_filt(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x3 // [7:6]
        }
    
        pub fn set_ook_average_thresh_filt(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 6);
            self.0 |= value << 6;
            self
        }
    
    }

    impl ::core::fmt::Display for Ookavg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Ookavg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.ook_average_thresh_filt() != 0 { write!(f, " ook_average_thresh_filt=0x{:x}", self.ook_average_thresh_filt())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Ookfix(u8);

    impl From<u8> for Ookfix {
        fn from(other: u8) -> Self { Ookfix(other) }
    }

    impl From<Ookfix> for u8 {
        fn from(other: Ookfix) -> Self { other.0 }
    }

    impl Ookfix {
        pub fn value(&self) -> u8 { self.0 }

        pub fn ook_fixed_thresh(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_ook_fixed_thresh(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Ookfix {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Ookfix {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.ook_fixed_thresh() != 0 { write!(f, " ook_fixed_thresh=0x{:x}", self.ook_fixed_thresh())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Afcfei(u8);

    impl From<u8> for Afcfei {
        fn from(other: u8) -> Self { Afcfei(other) }
    }

    impl From<Afcfei> for u8 {
        fn from(other: Afcfei) -> Self { other.0 }
    }

    impl Afcfei {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fei_done(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_fei_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn fei_start(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_fei_start(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn afc_done(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_afc_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn afc_autoclear_on(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_afc_autoclear_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn afc_auto_on(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_afc_auto_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn afc_clear(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_afc_clear(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn afc_start(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_afc_start(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Afcfei {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Afcfei {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fei_done() != 0 { write!(f, " fei_done")? }
            if self.fei_start() != 0 { write!(f, " fei_start")? }
            if self.afc_done() != 0 { write!(f, " afc_done")? }
            if self.afc_autoclear_on() != 0 { write!(f, " afc_autoclear_on")? }
            if self.afc_auto_on() != 0 { write!(f, " afc_auto_on")? }
            if self.afc_clear() != 0 { write!(f, " afc_clear")? }
            if self.afc_start() != 0 { write!(f, " afc_start")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Afcmsb(u8);

    impl From<u8> for Afcmsb {
        fn from(other: u8) -> Self { Afcmsb(other) }
    }

    impl From<Afcmsb> for u8 {
        fn from(other: Afcmsb) -> Self { other.0 }
    }

    impl Afcmsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn afc_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_afc_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Afcmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Afcmsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.afc_value() != 0 { write!(f, " afc_value=0x{:x}", self.afc_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Afclsb(u8);

    impl From<u8> for Afclsb {
        fn from(other: u8) -> Self { Afclsb(other) }
    }

    impl From<Afclsb> for u8 {
        fn from(other: Afclsb) -> Self { other.0 }
    }

    impl Afclsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn afc_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_afc_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Afclsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Afclsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.afc_value() != 0 { write!(f, " afc_value=0x{:x}", self.afc_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Feimsb(u8);

    impl From<u8> for Feimsb {
        fn from(other: u8) -> Self { Feimsb(other) }
    }

    impl From<Feimsb> for u8 {
        fn from(other: Feimsb) -> Self { other.0 }
    }

    impl Feimsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fei_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fei_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Feimsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Feimsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fei_value() != 0 { write!(f, " fei_value=0x{:x}", self.fei_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Feilsb(u8);

    impl From<u8> for Feilsb {
        fn from(other: u8) -> Self { Feilsb(other) }
    }

    impl From<Feilsb> for u8 {
        fn from(other: Feilsb) -> Self { other.0 }
    }

    impl Feilsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fei_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_fei_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Feilsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Feilsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fei_value() != 0 { write!(f, " fei_value=0x{:x}", self.fei_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rssiconfig(u8);

    impl From<u8> for Rssiconfig {
        fn from(other: u8) -> Self { Rssiconfig(other) }
    }

    impl From<Rssiconfig> for u8 {
        fn from(other: Rssiconfig) -> Self { other.0 }
    }

    impl Rssiconfig {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rssi_done(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_rssi_done(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn rssi_start(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_rssi_start(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rssiconfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rssiconfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rssi_done() != 0 { write!(f, " rssi_done")? }
            if self.rssi_start() != 0 { write!(f, " rssi_start")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rssivalue(u8);

    impl From<u8> for Rssivalue {
        fn from(other: u8) -> Self { Rssivalue(other) }
    }

    impl From<Rssivalue> for u8 {
        fn from(other: Rssivalue) -> Self { other.0 }
    }

    impl Rssivalue {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rssi_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_rssi_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rssivalue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rssivalue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rssi_value() != 0 { write!(f, " rssi_value=0x{:x}", self.rssi_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Diomapping1(u8);

    impl From<u8> for Diomapping1 {
        fn from(other: u8) -> Self { Diomapping1(other) }
    }

    impl From<Diomapping1> for u8 {
        fn from(other: Diomapping1) -> Self { other.0 }
    }

    impl Diomapping1 {
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

    impl ::core::fmt::Display for Diomapping1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Diomapping1 {
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
    pub struct Diomapping2(u8);

    impl From<u8> for Diomapping2 {
        fn from(other: u8) -> Self { Diomapping2(other) }
    }

    impl From<Diomapping2> for u8 {
        fn from(other: Diomapping2) -> Self { other.0 }
    }

    impl Diomapping2 {
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
    
        pub fn clk_out(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_clk_out(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Diomapping2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Diomapping2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.dio4_mapping() != 0 { write!(f, " dio4_mapping=0x{:x}", self.dio4_mapping())? }
            if self.dio5_mapping() != 0 { write!(f, " dio5_mapping=0x{:x}", self.dio5_mapping())? }
            if self.clk_out() != 0 { write!(f, " clk_out=0x{:x}", self.clk_out())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Irqflags1(u8);

    impl From<u8> for Irqflags1 {
        fn from(other: u8) -> Self { Irqflags1(other) }
    }

    impl From<Irqflags1> for u8 {
        fn from(other: Irqflags1) -> Self { other.0 }
    }

    impl Irqflags1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn mode_ready(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_mode_ready(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn rx_ready(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_rx_ready(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn tx_ready(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_tx_ready(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn pll_lock(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_pll_lock(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn rssi(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_rssi(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn timeout(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_timeout(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn auto_mode(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_auto_mode(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn sync_address_match(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_sync_address_match(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Irqflags1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Irqflags1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.mode_ready() != 0 { write!(f, " mode_ready")? }
            if self.rx_ready() != 0 { write!(f, " rx_ready")? }
            if self.tx_ready() != 0 { write!(f, " tx_ready")? }
            if self.pll_lock() != 0 { write!(f, " pll_lock")? }
            if self.rssi() != 0 { write!(f, " rssi")? }
            if self.timeout() != 0 { write!(f, " timeout")? }
            if self.auto_mode() != 0 { write!(f, " auto_mode")? }
            if self.sync_address_match() != 0 { write!(f, " sync_address_match")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Irqflags2(u8);

    impl From<u8> for Irqflags2 {
        fn from(other: u8) -> Self { Irqflags2(other) }
    }

    impl From<Irqflags2> for u8 {
        fn from(other: Irqflags2) -> Self { other.0 }
    }

    impl Irqflags2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn fifo_full(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_fifo_full(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn fifo_not_empty(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_fifo_not_empty(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn fifo_level(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x1 // [5]
        }
    
        pub fn set_fifo_level(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn fifo_overrun(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_fifo_overrun(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn packet_sent(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_packet_sent(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn payload_ready(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_payload_ready(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn crc_ok(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_crc_ok(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
    }

    impl ::core::fmt::Display for Irqflags2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Irqflags2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.fifo_full() != 0 { write!(f, " fifo_full")? }
            if self.fifo_not_empty() != 0 { write!(f, " fifo_not_empty")? }
            if self.fifo_level() != 0 { write!(f, " fifo_level")? }
            if self.fifo_overrun() != 0 { write!(f, " fifo_overrun")? }
            if self.packet_sent() != 0 { write!(f, " packet_sent")? }
            if self.payload_ready() != 0 { write!(f, " payload_ready")? }
            if self.crc_ok() != 0 { write!(f, " crc_ok")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rssithresh(u8);

    impl From<u8> for Rssithresh {
        fn from(other: u8) -> Self { Rssithresh(other) }
    }

    impl From<Rssithresh> for u8 {
        fn from(other: Rssithresh) -> Self { other.0 }
    }

    impl Rssithresh {
        pub fn value(&self) -> u8 { self.0 }

        pub fn rssi_threshold(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_rssi_threshold(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rssithresh {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rssithresh {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.rssi_threshold() != 0 { write!(f, " rssi_threshold=0x{:x}", self.rssi_threshold())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rxtimeout1(u8);

    impl From<u8> for Rxtimeout1 {
        fn from(other: u8) -> Self { Rxtimeout1(other) }
    }

    impl From<Rxtimeout1> for u8 {
        fn from(other: Rxtimeout1) -> Self { other.0 }
    }

    impl Rxtimeout1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn timeout_rx_start(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_timeout_rx_start(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rxtimeout1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rxtimeout1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.timeout_rx_start() != 0 { write!(f, " timeout_rx_start=0x{:x}", self.timeout_rx_start())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Rxtimeout2(u8);

    impl From<u8> for Rxtimeout2 {
        fn from(other: u8) -> Self { Rxtimeout2(other) }
    }

    impl From<Rxtimeout2> for u8 {
        fn from(other: Rxtimeout2) -> Self { other.0 }
    }

    impl Rxtimeout2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn timeout_rssi_thresh(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_timeout_rssi_thresh(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Rxtimeout2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Rxtimeout2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.timeout_rssi_thresh() != 0 { write!(f, " timeout_rssi_thresh=0x{:x}", self.timeout_rssi_thresh())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Preamblemsb(u8);

    impl From<u8> for Preamblemsb {
        fn from(other: u8) -> Self { Preamblemsb(other) }
    }

    impl From<Preamblemsb> for u8 {
        fn from(other: Preamblemsb) -> Self { other.0 }
    }

    impl Preamblemsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn preamble_size(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_preamble_size(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Preamblemsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Preamblemsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.preamble_size() != 0 { write!(f, " preamble_size=0x{:x}", self.preamble_size())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Preamblelsb(u8);

    impl From<u8> for Preamblelsb {
        fn from(other: u8) -> Self { Preamblelsb(other) }
    }

    impl From<Preamblelsb> for u8 {
        fn from(other: Preamblelsb) -> Self { other.0 }
    }

    impl Preamblelsb {
        pub fn value(&self) -> u8 { self.0 }

        pub fn preamble_size(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_preamble_size(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Preamblelsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Preamblelsb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.preamble_size() != 0 { write!(f, " preamble_size=0x{:x}", self.preamble_size())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Syncconfig(u8);

    impl From<u8> for Syncconfig {
        fn from(other: u8) -> Self { Syncconfig(other) }
    }

    impl From<Syncconfig> for u8 {
        fn from(other: Syncconfig) -> Self { other.0 }
    }

    impl Syncconfig {
        pub fn value(&self) -> u8 { self.0 }

        pub fn sync_on(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_sync_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn fifo_fill_condition(&self) -> u8 {
            ((self.0 as u8) >> 6) & 0x1 // [6]
        }
    
        pub fn set_fifo_fill_condition(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 6);
            self.0 |= value << 6;
            self
        }
    
        pub fn sync_size(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x7 // [5:3]
        }
    
        pub fn set_sync_size(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn sync_tol(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7 // [2:0]
        }
    
        pub fn set_sync_tol(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Syncconfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Syncconfig {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.sync_on() != 0 { write!(f, " sync_on")? }
            if self.fifo_fill_condition() != 0 { write!(f, " fifo_fill_condition")? }
            if self.sync_size() != 0 { write!(f, " sync_size=0x{:x}", self.sync_size())? }
            if self.sync_tol() != 0 { write!(f, " sync_tol=0x{:x}", self.sync_tol())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Syncvalue(u8);

    impl From<u8> for Syncvalue {
        fn from(other: u8) -> Self { Syncvalue(other) }
    }

    impl From<Syncvalue> for u8 {
        fn from(other: Syncvalue) -> Self { other.0 }
    }

    impl Syncvalue {
        pub fn value(&self) -> u8 { self.0 }

        pub fn sync_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_sync_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Syncvalue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Syncvalue {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.sync_value() != 0 { write!(f, " sync_value=0x{:x}", self.sync_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Packetconfig1(u8);

    impl From<u8> for Packetconfig1 {
        fn from(other: u8) -> Self { Packetconfig1(other) }
    }

    impl From<Packetconfig1> for u8 {
        fn from(other: Packetconfig1) -> Self { other.0 }
    }

    impl Packetconfig1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn packet_format(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_packet_format(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn dc_free(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x3 // [6:5]
        }
    
        pub fn set_dc_free(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn crc_on(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0x1 // [4]
        }
    
        pub fn set_crc_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn crc_auto_clear_off(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_crc_auto_clear_off(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn address_filtering(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x3 // [2:1]
        }
    
        pub fn set_address_filtering(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 1);
            self.0 |= value << 1;
            self
        }
    
    }

    impl ::core::fmt::Display for Packetconfig1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Packetconfig1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.packet_format() != 0 { write!(f, " packet_format")? }
            if self.dc_free() != 0 { write!(f, " dc_free=0x{:x}", self.dc_free())? }
            if self.crc_on() != 0 { write!(f, " crc_on")? }
            if self.crc_auto_clear_off() != 0 { write!(f, " crc_auto_clear_off")? }
            if self.address_filtering() != 0 { write!(f, " address_filtering=0x{:x}", self.address_filtering())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Payloadlength(u8);

    impl From<u8> for Payloadlength {
        fn from(other: u8) -> Self { Payloadlength(other) }
    }

    impl From<Payloadlength> for u8 {
        fn from(other: Payloadlength) -> Self { other.0 }
    }

    impl Payloadlength {
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

    impl ::core::fmt::Display for Payloadlength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Payloadlength {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.payload_length() != 0 { write!(f, " payload_length=0x{:x}", self.payload_length())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Nodeadrs(u8);

    impl From<u8> for Nodeadrs {
        fn from(other: u8) -> Self { Nodeadrs(other) }
    }

    impl From<Nodeadrs> for u8 {
        fn from(other: Nodeadrs) -> Self { other.0 }
    }

    impl Nodeadrs {
        pub fn value(&self) -> u8 { self.0 }

        pub fn node_address(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_node_address(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Nodeadrs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Nodeadrs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.node_address() != 0 { write!(f, " node_address=0x{:x}", self.node_address())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Broadcastadrs(u8);

    impl From<u8> for Broadcastadrs {
        fn from(other: u8) -> Self { Broadcastadrs(other) }
    }

    impl From<Broadcastadrs> for u8 {
        fn from(other: Broadcastadrs) -> Self { other.0 }
    }

    impl Broadcastadrs {
        pub fn value(&self) -> u8 { self.0 }

        pub fn broadcast_address(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_broadcast_address(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Broadcastadrs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Broadcastadrs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.broadcast_address() != 0 { write!(f, " broadcast_address=0x{:x}", self.broadcast_address())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Automodes(u8);

    impl From<u8> for Automodes {
        fn from(other: u8) -> Self { Automodes(other) }
    }

    impl From<Automodes> for u8 {
        fn from(other: Automodes) -> Self { other.0 }
    }

    impl Automodes {
        pub fn value(&self) -> u8 { self.0 }

        pub fn enter_condition(&self) -> u8 {
            ((self.0 as u8) >> 5) & 0x7 // [7:5]
        }
    
        pub fn set_enter_condition(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 5);
            self.0 |= value << 5;
            self
        }
    
        pub fn exit_condition(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x7 // [4:2]
        }
    
        pub fn set_exit_condition(mut self, value: u8) -> Self {
            assert!((value & !0x7) == 0);
            self.0 &= !(0x7 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn intermediate_mode(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x3 // [1:0]
        }
    
        pub fn set_intermediate_mode(mut self, value: u8) -> Self {
            assert!((value & !0x3) == 0);
            self.0 &= !(0x3 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Automodes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Automodes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.enter_condition() != 0 { write!(f, " enter_condition=0x{:x}", self.enter_condition())? }
            if self.exit_condition() != 0 { write!(f, " exit_condition=0x{:x}", self.exit_condition())? }
            if self.intermediate_mode() != 0 { write!(f, " intermediate_mode=0x{:x}", self.intermediate_mode())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Fifothresh(u8);

    impl From<u8> for Fifothresh {
        fn from(other: u8) -> Self { Fifothresh(other) }
    }

    impl From<Fifothresh> for u8 {
        fn from(other: Fifothresh) -> Self { other.0 }
    }

    impl Fifothresh {
        pub fn value(&self) -> u8 { self.0 }

        pub fn tx_start_condition(&self) -> u8 {
            ((self.0 as u8) >> 7) & 0x1 // [7]
        }
    
        pub fn set_tx_start_condition(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 7);
            self.0 |= value << 7;
            self
        }
    
        pub fn fifo_threshold(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x7f // [6:0]
        }
    
        pub fn set_fifo_threshold(mut self, value: u8) -> Self {
            assert!((value & !0x7f) == 0);
            self.0 &= !(0x7f << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Fifothresh {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Fifothresh {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.tx_start_condition() != 0 { write!(f, " tx_start_condition")? }
            if self.fifo_threshold() != 0 { write!(f, " fifo_threshold=0x{:x}", self.fifo_threshold())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Packetconfig2(u8);

    impl From<u8> for Packetconfig2 {
        fn from(other: u8) -> Self { Packetconfig2(other) }
    }

    impl From<Packetconfig2> for u8 {
        fn from(other: Packetconfig2) -> Self { other.0 }
    }

    impl Packetconfig2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn inter_packet_rx_delay(&self) -> u8 {
            ((self.0 as u8) >> 4) & 0xf // [7:4]
        }
    
        pub fn set_inter_packet_rx_delay(mut self, value: u8) -> Self {
            assert!((value & !0xf) == 0);
            self.0 &= !(0xf << 4);
            self.0 |= value << 4;
            self
        }
    
        pub fn restart_rx(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_restart_rx(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
        pub fn auto_rx_restart_on(&self) -> u8 {
            ((self.0 as u8) >> 1) & 0x1 // [1]
        }
    
        pub fn set_auto_rx_restart_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 1);
            self.0 |= value << 1;
            self
        }
    
        pub fn aes_on(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0x1 // [0]
        }
    
        pub fn set_aes_on(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Packetconfig2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Packetconfig2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.inter_packet_rx_delay() != 0 { write!(f, " inter_packet_rx_delay=0x{:x}", self.inter_packet_rx_delay())? }
            if self.restart_rx() != 0 { write!(f, " restart_rx")? }
            if self.auto_rx_restart_on() != 0 { write!(f, " auto_rx_restart_on")? }
            if self.aes_on() != 0 { write!(f, " aes_on")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Aeskey(u8);

    impl From<u8> for Aeskey {
        fn from(other: u8) -> Self { Aeskey(other) }
    }

    impl From<Aeskey> for u8 {
        fn from(other: Aeskey) -> Self { other.0 }
    }

    impl Aeskey {
        pub fn value(&self) -> u8 { self.0 }

        pub fn aes_key(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_aes_key(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Aeskey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Aeskey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.aes_key() != 0 { write!(f, " aes_key=0x{:x}", self.aes_key())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Temp1(u8);

    impl From<u8> for Temp1 {
        fn from(other: u8) -> Self { Temp1(other) }
    }

    impl From<Temp1> for u8 {
        fn from(other: Temp1) -> Self { other.0 }
    }

    impl Temp1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn temp_meas_start(&self) -> u8 {
            ((self.0 as u8) >> 3) & 0x1 // [3]
        }
    
        pub fn set_temp_meas_start(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 3);
            self.0 |= value << 3;
            self
        }
    
        pub fn temp_meas_running(&self) -> u8 {
            ((self.0 as u8) >> 2) & 0x1 // [2]
        }
    
        pub fn set_temp_meas_running(mut self, value: u8) -> Self {
            assert!((value & !0x1) == 0);
            self.0 &= !(0x1 << 2);
            self.0 |= value << 2;
            self
        }
    
    }

    impl ::core::fmt::Display for Temp1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Temp1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.temp_meas_start() != 0 { write!(f, " temp_meas_start")? }
            if self.temp_meas_running() != 0 { write!(f, " temp_meas_running")? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Temp2(u8);

    impl From<u8> for Temp2 {
        fn from(other: u8) -> Self { Temp2(other) }
    }

    impl From<Temp2> for u8 {
        fn from(other: Temp2) -> Self { other.0 }
    }

    impl Temp2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn temp_value(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_temp_value(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Temp2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Temp2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.temp_value() != 0 { write!(f, " temp_value=0x{:x}", self.temp_value())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Testlna(u8);

    impl From<u8> for Testlna {
        fn from(other: u8) -> Self { Testlna(other) }
    }

    impl From<Testlna> for u8 {
        fn from(other: Testlna) -> Self { other.0 }
    }

    impl Testlna {
        pub fn value(&self) -> u8 { self.0 }

        pub fn sensitivity_boost(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_sensitivity_boost(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Testlna {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Testlna {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.sensitivity_boost() != 0 { write!(f, " sensitivity_boost=0x{:x}", self.sensitivity_boost())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Testpa1(u8);

    impl From<u8> for Testpa1 {
        fn from(other: u8) -> Self { Testpa1(other) }
    }

    impl From<Testpa1> for u8 {
        fn from(other: Testpa1) -> Self { other.0 }
    }

    impl Testpa1 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa20dbm1(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_pa20dbm1(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Testpa1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Testpa1 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa20dbm1() != 0 { write!(f, " pa20dbm1=0x{:x}", self.pa20dbm1())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Testpa2(u8);

    impl From<u8> for Testpa2 {
        fn from(other: u8) -> Self { Testpa2(other) }
    }

    impl From<Testpa2> for u8 {
        fn from(other: Testpa2) -> Self { other.0 }
    }

    impl Testpa2 {
        pub fn value(&self) -> u8 { self.0 }

        pub fn pa20dbm2(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_pa20dbm2(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Testpa2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Testpa2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.pa20dbm2() != 0 { write!(f, " pa20dbm2=0x{:x}", self.pa20dbm2())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Testdagc(u8);

    impl From<u8> for Testdagc {
        fn from(other: u8) -> Self { Testdagc(other) }
    }

    impl From<Testdagc> for u8 {
        fn from(other: Testdagc) -> Self { other.0 }
    }

    impl Testdagc {
        pub fn value(&self) -> u8 { self.0 }

        pub fn continuous_dagc(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_continuous_dagc(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Testdagc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Testdagc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.continuous_dagc() != 0 { write!(f, " continuous_dagc=0x{:x}", self.continuous_dagc())? }
            write!(f, "]")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Testafc(u8);

    impl From<u8> for Testafc {
        fn from(other: u8) -> Self { Testafc(other) }
    }

    impl From<Testafc> for u8 {
        fn from(other: Testafc) -> Self { other.0 }
    }

    impl Testafc {
        pub fn value(&self) -> u8 { self.0 }

        pub fn low_beta_afc_offset(&self) -> u8 {
            ((self.0 as u8) >> 0) & 0xff // [7:0]
        }
    
        pub fn set_low_beta_afc_offset(mut self, value: u8) -> Self {
            assert!((value & !0xff) == 0);
            self.0 &= !(0xff << 0);
            self.0 |= value << 0;
            self
        }
    
    }

    impl ::core::fmt::Display for Testafc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::core::fmt::Debug for Testafc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(f, "[0x{:02x}", self.0)?;
            if self.low_beta_afc_offset() != 0 { write!(f, " low_beta_afc_offset=0x{:x}", self.low_beta_afc_offset())? }
            write!(f, "]")?;
            Ok(())
        }
    }


}

