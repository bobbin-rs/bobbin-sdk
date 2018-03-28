#![no_std]
pub const REG_FIFO: u8 = 0x00;
pub const REG_OPMODE: u8 = 0x01;
pub const REG_DATAMODUL: u8 = 0x02;
pub const REG_BITRATEMSB: u8 = 0x03;
pub const REG_BITRATELSB: u8 = 0x04;
pub const REG_FDEVMSB: u8 = 0x05;
pub const REG_FDEVLSB: u8 = 0x06;
pub const REG_FRFMSB: u8 = 0x07;
pub const REG_FRFMID: u8 = 0x08;
pub const REG_FRFLSB: u8 = 0x09;
pub const REG_OSC1: u8 = 0x0a;
pub const REG_AFCCTRL: u8 = 0x0b;
pub const REG_LISTEN1: u8 = 0x0d;
pub const REG_LISTEN2: u8 = 0x0e;
pub const REG_LISTEN3: u8 = 0x0f;
pub const REG_VERSION: u8 = 0x10;
pub const REG_PALEVEL: u8 = 0x11;
pub const REG_PARAMP: u8 = 0x12;
pub const REG_OCP: u8 = 0x13;
pub const REG_LNA: u8 = 0x18;
pub const REG_RXBW: u8 = 0x19;
pub const REG_AFCBW: u8 = 0x1a;
pub const REG_OOKPEAK: u8 = 0x1b;
pub const REG_OOKAVG: u8 = 0x1c;
pub const REG_OOKFIX: u8 = 0x1d;
pub const REG_AFCFEI: u8 = 0x1e;
pub const REG_AFCMSB: u8 = 0x1f;
pub const REG_AFCLSB: u8 = 0x20;
pub const REG_FEIMSB: u8 = 0x21;
pub const REG_FEILSB: u8 = 0x22;
pub const REG_RSSICONFIG: u8 = 0x23;
pub const REG_RSSIVALUE: u8 = 0x24;
pub const REG_DIOMAPPING1: u8 = 0x25;
pub const REG_DIOMAPPING2: u8 = 0x26;
pub const REG_IRQFLAGS1: u8 = 0x27;
pub const REG_IRQFLAGS2: u8 = 0x28;
pub const REG_RSSITHRESH: u8 = 0x29;
pub const REG_RXTIMEOUT1: u8 = 0x2a;
pub const REG_RXTIMEOUT2: u8 = 0x2b;
pub const REG_PREAMBLEMSB: u8 = 0x2c;
pub const REG_PREAMBLELSB: u8 = 0x2d;
pub const REG_SYNCCONFIG: u8 = 0x2e;
pub const REG_SYNCVALUE: u8 = 0x2f;
pub const REG_SYNCVALUE1: u8 = 0x2f;
pub const REG_SYNCVALUE2: u8 = 0x30;
pub const REG_SYNCVALUE3: u8 = 0x31;
pub const REG_SYNCVALUE4: u8 = 0x32;
pub const REG_SYNCVALUE5: u8 = 0x33;
pub const REG_SYNCVALUE6: u8 = 0x34;
pub const REG_SYNCVALUE7: u8 = 0x35;
pub const REG_SYNCVALUE8: u8 = 0x36;
pub const REG_PACKETCONFIG1: u8 = 0x37;
pub const REG_PAYLOADLENGTH: u8 = 0x38;
pub const REG_NODEADRS: u8 = 0x39;
pub const REG_BROADCASTADRS: u8 = 0x3a;
pub const REG_AUTOMODES: u8 = 0x3b;
pub const REG_FIFOTHRESH: u8 = 0x3c;
pub const REG_PACKETCONFIG2: u8 = 0x3d;
pub const REG_AESKEY: u8 = 0x3e;
pub const REG_AESKEY1: u8 = 0x3e;
pub const REG_AESKEY2: u8 = 0x3f;
pub const REG_AESKEY3: u8 = 0x40;
pub const REG_AESKEY4: u8 = 0x41;
pub const REG_AESKEY5: u8 = 0x42;
pub const REG_AESKEY6: u8 = 0x43;
pub const REG_AESKEY7: u8 = 0x44;
pub const REG_AESKEY8: u8 = 0x45;
pub const REG_AESKEY9: u8 = 0x46;
pub const REG_AESKEY10: u8 = 0x47;
pub const REG_AESKEY11: u8 = 0x48;
pub const REG_AESKEY12: u8 = 0x49;
pub const REG_AESKEY13: u8 = 0x4a;
pub const REG_AESKEY14: u8 = 0x4b;
pub const REG_AESKEY15: u8 = 0x4c;
pub const REG_AESKEY16: u8 = 0x4d;
pub const REG_TEMP1: u8 = 0x4e;
pub const REG_TEMP2: u8 = 0x4f;
pub const REG_TESTLNA: u8 = 0x58;
pub const REG_TESTPA1: u8 = 0x5a;
pub const REG_TESTPA2: u8 = 0x5c;
pub const REG_TESTDAGC: u8 = 0x6f;
pub const REG_TESTAFC: u8 = 0x71;

pub trait ReadWrite {
    type Addr: Copy;
    type Value: Copy;
    fn read(&self, addr: Self::Addr) -> Self::Value;
    fn write(&self, addr: Self::Addr, val: Self::Value);
}

pub trait TryReadWrite {
    type Addr: Copy;
    type Value: Copy;
    type Error;
    fn try_read(&self, addr: Self::Addr) -> Result<Self::Value, Self::Error>;
    fn try_write(&self, addr: Self::Addr, val: Self::Value) -> Result<(), Self::Error>;
}


pub struct Rfm69<RW> { rw: RW }

impl<RW: ReadWrite<Addr=u8, Value=u8>> ReadWrite for Rfm69<RW> {
    type Addr = RW::Addr;
    type Value = RW::Value;
    fn read(&self, addr: Self::Addr) -> Self::Value { self.rw.read(addr) }
    fn write(&self, addr: Self::Addr, val: Self::Value) { self.rw.write(addr, val) }
}

impl<RW: TryReadWrite<Addr=u8, Value=u8>> TryReadWrite for Rfm69<RW> {
    type Addr = RW::Addr;
    type Value = RW::Value;
    type Error = RW::Error;
    fn try_read(&self, addr: Self::Addr) -> Result<Self::Value, Self::Error> { self.rw.try_read(addr) }
    fn try_write(&self, addr: Self::Addr, val: Self::Value) -> Result<(), Self::Error> { self.rw.try_write(addr, val) }
}

impl<RW> Rfm69<RW> {
    pub fn new(rw: RW) -> Self { Rfm69 { rw } }
}

impl<RW: ReadWrite<Addr=u8, Value=u8>> Rfm69<RW> {
    pub fn fifo(&self) -> Fifo {
        Fifo(self.rw.read(REG_FIFO))
    }
    pub fn set_fifo(&self, value: Fifo) {
        self.rw.write(REG_FIFO, value.0)
    }
    pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) {
        let tmp = Fifo(self.rw.read(REG_FIFO));
        self.rw.write(REG_FIFO, f(tmp).0)
    }

    pub fn opmode(&self) -> Opmode {
        Opmode(self.rw.read(REG_OPMODE))
    }
    pub fn set_opmode(&self, value: Opmode) {
        self.rw.write(REG_OPMODE, value.0)
    }
    pub fn with_opmode<F: FnOnce(Opmode) -> Opmode>(&self, f: F) {
        let tmp = Opmode(self.rw.read(REG_OPMODE));
        self.rw.write(REG_OPMODE, f(tmp).0)
    }

    pub fn datamodul(&self) -> Datamodul {
        Datamodul(self.rw.read(REG_DATAMODUL))
    }
    pub fn set_datamodul(&self, value: Datamodul) {
        self.rw.write(REG_DATAMODUL, value.0)
    }
    pub fn with_datamodul<F: FnOnce(Datamodul) -> Datamodul>(&self, f: F) {
        let tmp = Datamodul(self.rw.read(REG_DATAMODUL));
        self.rw.write(REG_DATAMODUL, f(tmp).0)
    }

    pub fn bitratemsb(&self) -> Bitratemsb {
        Bitratemsb(self.rw.read(REG_BITRATEMSB))
    }
    pub fn set_bitratemsb(&self, value: Bitratemsb) {
        self.rw.write(REG_BITRATEMSB, value.0)
    }
    pub fn with_bitratemsb<F: FnOnce(Bitratemsb) -> Bitratemsb>(&self, f: F) {
        let tmp = Bitratemsb(self.rw.read(REG_BITRATEMSB));
        self.rw.write(REG_BITRATEMSB, f(tmp).0)
    }

    pub fn bitratelsb(&self) -> Bitratelsb {
        Bitratelsb(self.rw.read(REG_BITRATELSB))
    }
    pub fn set_bitratelsb(&self, value: Bitratelsb) {
        self.rw.write(REG_BITRATELSB, value.0)
    }
    pub fn with_bitratelsb<F: FnOnce(Bitratelsb) -> Bitratelsb>(&self, f: F) {
        let tmp = Bitratelsb(self.rw.read(REG_BITRATELSB));
        self.rw.write(REG_BITRATELSB, f(tmp).0)
    }

    pub fn fdevmsb(&self) -> Fdevmsb {
        Fdevmsb(self.rw.read(REG_FDEVMSB))
    }
    pub fn set_fdevmsb(&self, value: Fdevmsb) {
        self.rw.write(REG_FDEVMSB, value.0)
    }
    pub fn with_fdevmsb<F: FnOnce(Fdevmsb) -> Fdevmsb>(&self, f: F) {
        let tmp = Fdevmsb(self.rw.read(REG_FDEVMSB));
        self.rw.write(REG_FDEVMSB, f(tmp).0)
    }

    pub fn fdevlsb(&self) -> Fdevlsb {
        Fdevlsb(self.rw.read(REG_FDEVLSB))
    }
    pub fn set_fdevlsb(&self, value: Fdevlsb) {
        self.rw.write(REG_FDEVLSB, value.0)
    }
    pub fn with_fdevlsb<F: FnOnce(Fdevlsb) -> Fdevlsb>(&self, f: F) {
        let tmp = Fdevlsb(self.rw.read(REG_FDEVLSB));
        self.rw.write(REG_FDEVLSB, f(tmp).0)
    }

    pub fn frfmsb(&self) -> Frfmsb {
        Frfmsb(self.rw.read(REG_FRFMSB))
    }
    pub fn set_frfmsb(&self, value: Frfmsb) {
        self.rw.write(REG_FRFMSB, value.0)
    }
    pub fn with_frfmsb<F: FnOnce(Frfmsb) -> Frfmsb>(&self, f: F) {
        let tmp = Frfmsb(self.rw.read(REG_FRFMSB));
        self.rw.write(REG_FRFMSB, f(tmp).0)
    }

    pub fn frfmid(&self) -> Frfmid {
        Frfmid(self.rw.read(REG_FRFMID))
    }
    pub fn set_frfmid(&self, value: Frfmid) {
        self.rw.write(REG_FRFMID, value.0)
    }
    pub fn with_frfmid<F: FnOnce(Frfmid) -> Frfmid>(&self, f: F) {
        let tmp = Frfmid(self.rw.read(REG_FRFMID));
        self.rw.write(REG_FRFMID, f(tmp).0)
    }

    pub fn frflsb(&self) -> Frflsb {
        Frflsb(self.rw.read(REG_FRFLSB))
    }
    pub fn set_frflsb(&self, value: Frflsb) {
        self.rw.write(REG_FRFLSB, value.0)
    }
    pub fn with_frflsb<F: FnOnce(Frflsb) -> Frflsb>(&self, f: F) {
        let tmp = Frflsb(self.rw.read(REG_FRFLSB));
        self.rw.write(REG_FRFLSB, f(tmp).0)
    }

    pub fn osc1(&self) -> Osc1 {
        Osc1(self.rw.read(REG_OSC1))
    }
    pub fn set_osc1(&self, value: Osc1) {
        self.rw.write(REG_OSC1, value.0)
    }
    pub fn with_osc1<F: FnOnce(Osc1) -> Osc1>(&self, f: F) {
        let tmp = Osc1(self.rw.read(REG_OSC1));
        self.rw.write(REG_OSC1, f(tmp).0)
    }

    pub fn afcctrl(&self) -> Afcctrl {
        Afcctrl(self.rw.read(REG_AFCCTRL))
    }
    pub fn set_afcctrl(&self, value: Afcctrl) {
        self.rw.write(REG_AFCCTRL, value.0)
    }
    pub fn with_afcctrl<F: FnOnce(Afcctrl) -> Afcctrl>(&self, f: F) {
        let tmp = Afcctrl(self.rw.read(REG_AFCCTRL));
        self.rw.write(REG_AFCCTRL, f(tmp).0)
    }

    pub fn listen1(&self) -> Listen1 {
        Listen1(self.rw.read(REG_LISTEN1))
    }
    pub fn set_listen1(&self, value: Listen1) {
        self.rw.write(REG_LISTEN1, value.0)
    }
    pub fn with_listen1<F: FnOnce(Listen1) -> Listen1>(&self, f: F) {
        let tmp = Listen1(self.rw.read(REG_LISTEN1));
        self.rw.write(REG_LISTEN1, f(tmp).0)
    }

    pub fn listen2(&self) -> Listen2 {
        Listen2(self.rw.read(REG_LISTEN2))
    }
    pub fn set_listen2(&self, value: Listen2) {
        self.rw.write(REG_LISTEN2, value.0)
    }
    pub fn with_listen2<F: FnOnce(Listen2) -> Listen2>(&self, f: F) {
        let tmp = Listen2(self.rw.read(REG_LISTEN2));
        self.rw.write(REG_LISTEN2, f(tmp).0)
    }

    pub fn listen3(&self) -> Listen3 {
        Listen3(self.rw.read(REG_LISTEN3))
    }
    pub fn set_listen3(&self, value: Listen3) {
        self.rw.write(REG_LISTEN3, value.0)
    }
    pub fn with_listen3<F: FnOnce(Listen3) -> Listen3>(&self, f: F) {
        let tmp = Listen3(self.rw.read(REG_LISTEN3));
        self.rw.write(REG_LISTEN3, f(tmp).0)
    }

    pub fn version(&self) -> Version {
        Version(self.rw.read(REG_VERSION))
    }
    pub fn set_version(&self, value: Version) {
        self.rw.write(REG_VERSION, value.0)
    }
    pub fn with_version<F: FnOnce(Version) -> Version>(&self, f: F) {
        let tmp = Version(self.rw.read(REG_VERSION));
        self.rw.write(REG_VERSION, f(tmp).0)
    }

    pub fn palevel(&self) -> Palevel {
        Palevel(self.rw.read(REG_PALEVEL))
    }
    pub fn set_palevel(&self, value: Palevel) {
        self.rw.write(REG_PALEVEL, value.0)
    }
    pub fn with_palevel<F: FnOnce(Palevel) -> Palevel>(&self, f: F) {
        let tmp = Palevel(self.rw.read(REG_PALEVEL));
        self.rw.write(REG_PALEVEL, f(tmp).0)
    }

    pub fn paramp(&self) -> Paramp {
        Paramp(self.rw.read(REG_PARAMP))
    }
    pub fn set_paramp(&self, value: Paramp) {
        self.rw.write(REG_PARAMP, value.0)
    }
    pub fn with_paramp<F: FnOnce(Paramp) -> Paramp>(&self, f: F) {
        let tmp = Paramp(self.rw.read(REG_PARAMP));
        self.rw.write(REG_PARAMP, f(tmp).0)
    }

    pub fn ocp(&self) -> Ocp {
        Ocp(self.rw.read(REG_OCP))
    }
    pub fn set_ocp(&self, value: Ocp) {
        self.rw.write(REG_OCP, value.0)
    }
    pub fn with_ocp<F: FnOnce(Ocp) -> Ocp>(&self, f: F) {
        let tmp = Ocp(self.rw.read(REG_OCP));
        self.rw.write(REG_OCP, f(tmp).0)
    }

    pub fn lna(&self) -> Lna {
        Lna(self.rw.read(REG_LNA))
    }
    pub fn set_lna(&self, value: Lna) {
        self.rw.write(REG_LNA, value.0)
    }
    pub fn with_lna<F: FnOnce(Lna) -> Lna>(&self, f: F) {
        let tmp = Lna(self.rw.read(REG_LNA));
        self.rw.write(REG_LNA, f(tmp).0)
    }

    pub fn rxbw(&self) -> Rxbw {
        Rxbw(self.rw.read(REG_RXBW))
    }
    pub fn set_rxbw(&self, value: Rxbw) {
        self.rw.write(REG_RXBW, value.0)
    }
    pub fn with_rxbw<F: FnOnce(Rxbw) -> Rxbw>(&self, f: F) {
        let tmp = Rxbw(self.rw.read(REG_RXBW));
        self.rw.write(REG_RXBW, f(tmp).0)
    }

    pub fn afcbw(&self) -> Afcbw {
        Afcbw(self.rw.read(REG_AFCBW))
    }
    pub fn set_afcbw(&self, value: Afcbw) {
        self.rw.write(REG_AFCBW, value.0)
    }
    pub fn with_afcbw<F: FnOnce(Afcbw) -> Afcbw>(&self, f: F) {
        let tmp = Afcbw(self.rw.read(REG_AFCBW));
        self.rw.write(REG_AFCBW, f(tmp).0)
    }

    pub fn ookpeak(&self) -> Ookpeak {
        Ookpeak(self.rw.read(REG_OOKPEAK))
    }
    pub fn set_ookpeak(&self, value: Ookpeak) {
        self.rw.write(REG_OOKPEAK, value.0)
    }
    pub fn with_ookpeak<F: FnOnce(Ookpeak) -> Ookpeak>(&self, f: F) {
        let tmp = Ookpeak(self.rw.read(REG_OOKPEAK));
        self.rw.write(REG_OOKPEAK, f(tmp).0)
    }

    pub fn ookavg(&self) -> Ookavg {
        Ookavg(self.rw.read(REG_OOKAVG))
    }
    pub fn set_ookavg(&self, value: Ookavg) {
        self.rw.write(REG_OOKAVG, value.0)
    }
    pub fn with_ookavg<F: FnOnce(Ookavg) -> Ookavg>(&self, f: F) {
        let tmp = Ookavg(self.rw.read(REG_OOKAVG));
        self.rw.write(REG_OOKAVG, f(tmp).0)
    }

    pub fn ookfix(&self) -> Ookfix {
        Ookfix(self.rw.read(REG_OOKFIX))
    }
    pub fn set_ookfix(&self, value: Ookfix) {
        self.rw.write(REG_OOKFIX, value.0)
    }
    pub fn with_ookfix<F: FnOnce(Ookfix) -> Ookfix>(&self, f: F) {
        let tmp = Ookfix(self.rw.read(REG_OOKFIX));
        self.rw.write(REG_OOKFIX, f(tmp).0)
    }

    pub fn afcfei(&self) -> Afcfei {
        Afcfei(self.rw.read(REG_AFCFEI))
    }
    pub fn set_afcfei(&self, value: Afcfei) {
        self.rw.write(REG_AFCFEI, value.0)
    }
    pub fn with_afcfei<F: FnOnce(Afcfei) -> Afcfei>(&self, f: F) {
        let tmp = Afcfei(self.rw.read(REG_AFCFEI));
        self.rw.write(REG_AFCFEI, f(tmp).0)
    }

    pub fn afcmsb(&self) -> Afcmsb {
        Afcmsb(self.rw.read(REG_AFCMSB))
    }
    pub fn set_afcmsb(&self, value: Afcmsb) {
        self.rw.write(REG_AFCMSB, value.0)
    }
    pub fn with_afcmsb<F: FnOnce(Afcmsb) -> Afcmsb>(&self, f: F) {
        let tmp = Afcmsb(self.rw.read(REG_AFCMSB));
        self.rw.write(REG_AFCMSB, f(tmp).0)
    }

    pub fn afclsb(&self) -> Afclsb {
        Afclsb(self.rw.read(REG_AFCLSB))
    }
    pub fn set_afclsb(&self, value: Afclsb) {
        self.rw.write(REG_AFCLSB, value.0)
    }
    pub fn with_afclsb<F: FnOnce(Afclsb) -> Afclsb>(&self, f: F) {
        let tmp = Afclsb(self.rw.read(REG_AFCLSB));
        self.rw.write(REG_AFCLSB, f(tmp).0)
    }

    pub fn feimsb(&self) -> Feimsb {
        Feimsb(self.rw.read(REG_FEIMSB))
    }
    pub fn set_feimsb(&self, value: Feimsb) {
        self.rw.write(REG_FEIMSB, value.0)
    }
    pub fn with_feimsb<F: FnOnce(Feimsb) -> Feimsb>(&self, f: F) {
        let tmp = Feimsb(self.rw.read(REG_FEIMSB));
        self.rw.write(REG_FEIMSB, f(tmp).0)
    }

    pub fn feilsb(&self) -> Feilsb {
        Feilsb(self.rw.read(REG_FEILSB))
    }
    pub fn set_feilsb(&self, value: Feilsb) {
        self.rw.write(REG_FEILSB, value.0)
    }
    pub fn with_feilsb<F: FnOnce(Feilsb) -> Feilsb>(&self, f: F) {
        let tmp = Feilsb(self.rw.read(REG_FEILSB));
        self.rw.write(REG_FEILSB, f(tmp).0)
    }

    pub fn rssiconfig(&self) -> Rssiconfig {
        Rssiconfig(self.rw.read(REG_RSSICONFIG))
    }
    pub fn set_rssiconfig(&self, value: Rssiconfig) {
        self.rw.write(REG_RSSICONFIG, value.0)
    }
    pub fn with_rssiconfig<F: FnOnce(Rssiconfig) -> Rssiconfig>(&self, f: F) {
        let tmp = Rssiconfig(self.rw.read(REG_RSSICONFIG));
        self.rw.write(REG_RSSICONFIG, f(tmp).0)
    }

    pub fn rssivalue(&self) -> Rssivalue {
        Rssivalue(self.rw.read(REG_RSSIVALUE))
    }
    pub fn set_rssivalue(&self, value: Rssivalue) {
        self.rw.write(REG_RSSIVALUE, value.0)
    }
    pub fn with_rssivalue<F: FnOnce(Rssivalue) -> Rssivalue>(&self, f: F) {
        let tmp = Rssivalue(self.rw.read(REG_RSSIVALUE));
        self.rw.write(REG_RSSIVALUE, f(tmp).0)
    }

    pub fn diomapping1(&self) -> Diomapping1 {
        Diomapping1(self.rw.read(REG_DIOMAPPING1))
    }
    pub fn set_diomapping1(&self, value: Diomapping1) {
        self.rw.write(REG_DIOMAPPING1, value.0)
    }
    pub fn with_diomapping1<F: FnOnce(Diomapping1) -> Diomapping1>(&self, f: F) {
        let tmp = Diomapping1(self.rw.read(REG_DIOMAPPING1));
        self.rw.write(REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn diomapping2(&self) -> Diomapping2 {
        Diomapping2(self.rw.read(REG_DIOMAPPING2))
    }
    pub fn set_diomapping2(&self, value: Diomapping2) {
        self.rw.write(REG_DIOMAPPING2, value.0)
    }
    pub fn with_diomapping2<F: FnOnce(Diomapping2) -> Diomapping2>(&self, f: F) {
        let tmp = Diomapping2(self.rw.read(REG_DIOMAPPING2));
        self.rw.write(REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn irqflags1(&self) -> Irqflags1 {
        Irqflags1(self.rw.read(REG_IRQFLAGS1))
    }
    pub fn set_irqflags1(&self, value: Irqflags1) {
        self.rw.write(REG_IRQFLAGS1, value.0)
    }
    pub fn with_irqflags1<F: FnOnce(Irqflags1) -> Irqflags1>(&self, f: F) {
        let tmp = Irqflags1(self.rw.read(REG_IRQFLAGS1));
        self.rw.write(REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn irqflags2(&self) -> Irqflags2 {
        Irqflags2(self.rw.read(REG_IRQFLAGS2))
    }
    pub fn set_irqflags2(&self, value: Irqflags2) {
        self.rw.write(REG_IRQFLAGS2, value.0)
    }
    pub fn with_irqflags2<F: FnOnce(Irqflags2) -> Irqflags2>(&self, f: F) {
        let tmp = Irqflags2(self.rw.read(REG_IRQFLAGS2));
        self.rw.write(REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn rssithresh(&self) -> Rssithresh {
        Rssithresh(self.rw.read(REG_RSSITHRESH))
    }
    pub fn set_rssithresh(&self, value: Rssithresh) {
        self.rw.write(REG_RSSITHRESH, value.0)
    }
    pub fn with_rssithresh<F: FnOnce(Rssithresh) -> Rssithresh>(&self, f: F) {
        let tmp = Rssithresh(self.rw.read(REG_RSSITHRESH));
        self.rw.write(REG_RSSITHRESH, f(tmp).0)
    }

    pub fn rxtimeout1(&self) -> Rxtimeout1 {
        Rxtimeout1(self.rw.read(REG_RXTIMEOUT1))
    }
    pub fn set_rxtimeout1(&self, value: Rxtimeout1) {
        self.rw.write(REG_RXTIMEOUT1, value.0)
    }
    pub fn with_rxtimeout1<F: FnOnce(Rxtimeout1) -> Rxtimeout1>(&self, f: F) {
        let tmp = Rxtimeout1(self.rw.read(REG_RXTIMEOUT1));
        self.rw.write(REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn rxtimeout2(&self) -> Rxtimeout2 {
        Rxtimeout2(self.rw.read(REG_RXTIMEOUT2))
    }
    pub fn set_rxtimeout2(&self, value: Rxtimeout2) {
        self.rw.write(REG_RXTIMEOUT2, value.0)
    }
    pub fn with_rxtimeout2<F: FnOnce(Rxtimeout2) -> Rxtimeout2>(&self, f: F) {
        let tmp = Rxtimeout2(self.rw.read(REG_RXTIMEOUT2));
        self.rw.write(REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn preamblemsb(&self) -> Preamblemsb {
        Preamblemsb(self.rw.read(REG_PREAMBLEMSB))
    }
    pub fn set_preamblemsb(&self, value: Preamblemsb) {
        self.rw.write(REG_PREAMBLEMSB, value.0)
    }
    pub fn with_preamblemsb<F: FnOnce(Preamblemsb) -> Preamblemsb>(&self, f: F) {
        let tmp = Preamblemsb(self.rw.read(REG_PREAMBLEMSB));
        self.rw.write(REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn preamblelsb(&self) -> Preamblelsb {
        Preamblelsb(self.rw.read(REG_PREAMBLELSB))
    }
    pub fn set_preamblelsb(&self, value: Preamblelsb) {
        self.rw.write(REG_PREAMBLELSB, value.0)
    }
    pub fn with_preamblelsb<F: FnOnce(Preamblelsb) -> Preamblelsb>(&self, f: F) {
        let tmp = Preamblelsb(self.rw.read(REG_PREAMBLELSB));
        self.rw.write(REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn syncconfig(&self) -> Syncconfig {
        Syncconfig(self.rw.read(REG_SYNCCONFIG))
    }
    pub fn set_syncconfig(&self, value: Syncconfig) {
        self.rw.write(REG_SYNCCONFIG, value.0)
    }
    pub fn with_syncconfig<F: FnOnce(Syncconfig) -> Syncconfig>(&self, f: F) {
        let tmp = Syncconfig(self.rw.read(REG_SYNCCONFIG));
        self.rw.write(REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn syncvalue(&self, index: usize) -> Syncvalue {
        assert!(index < 8);
        Syncvalue(self.rw.read(REG_SYNCVALUE + index as u8))
    }
    pub fn set_syncvalue(&self, index: usize, value: Syncvalue) {
        assert!(index < 8);
        self.rw.write(REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn with_syncvalue<F: FnOnce(Syncvalue) -> Syncvalue>(&self, index: usize, f: F) {
        assert!(index < 8);
        let tmp = Syncvalue(self.rw.read(REG_SYNCVALUE + index as u8));
        self.rw.write(REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn packetconfig1(&self) -> Packetconfig1 {
        Packetconfig1(self.rw.read(REG_PACKETCONFIG1))
    }
    pub fn set_packetconfig1(&self, value: Packetconfig1) {
        self.rw.write(REG_PACKETCONFIG1, value.0)
    }
    pub fn with_packetconfig1<F: FnOnce(Packetconfig1) -> Packetconfig1>(&self, f: F) {
        let tmp = Packetconfig1(self.rw.read(REG_PACKETCONFIG1));
        self.rw.write(REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn payloadlength(&self) -> Payloadlength {
        Payloadlength(self.rw.read(REG_PAYLOADLENGTH))
    }
    pub fn set_payloadlength(&self, value: Payloadlength) {
        self.rw.write(REG_PAYLOADLENGTH, value.0)
    }
    pub fn with_payloadlength<F: FnOnce(Payloadlength) -> Payloadlength>(&self, f: F) {
        let tmp = Payloadlength(self.rw.read(REG_PAYLOADLENGTH));
        self.rw.write(REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn nodeadrs(&self) -> Nodeadrs {
        Nodeadrs(self.rw.read(REG_NODEADRS))
    }
    pub fn set_nodeadrs(&self, value: Nodeadrs) {
        self.rw.write(REG_NODEADRS, value.0)
    }
    pub fn with_nodeadrs<F: FnOnce(Nodeadrs) -> Nodeadrs>(&self, f: F) {
        let tmp = Nodeadrs(self.rw.read(REG_NODEADRS));
        self.rw.write(REG_NODEADRS, f(tmp).0)
    }

    pub fn broadcastadrs(&self) -> Broadcastadrs {
        Broadcastadrs(self.rw.read(REG_BROADCASTADRS))
    }
    pub fn set_broadcastadrs(&self, value: Broadcastadrs) {
        self.rw.write(REG_BROADCASTADRS, value.0)
    }
    pub fn with_broadcastadrs<F: FnOnce(Broadcastadrs) -> Broadcastadrs>(&self, f: F) {
        let tmp = Broadcastadrs(self.rw.read(REG_BROADCASTADRS));
        self.rw.write(REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn automodes(&self) -> Automodes {
        Automodes(self.rw.read(REG_AUTOMODES))
    }
    pub fn set_automodes(&self, value: Automodes) {
        self.rw.write(REG_AUTOMODES, value.0)
    }
    pub fn with_automodes<F: FnOnce(Automodes) -> Automodes>(&self, f: F) {
        let tmp = Automodes(self.rw.read(REG_AUTOMODES));
        self.rw.write(REG_AUTOMODES, f(tmp).0)
    }

    pub fn fifothresh(&self) -> Fifothresh {
        Fifothresh(self.rw.read(REG_FIFOTHRESH))
    }
    pub fn set_fifothresh(&self, value: Fifothresh) {
        self.rw.write(REG_FIFOTHRESH, value.0)
    }
    pub fn with_fifothresh<F: FnOnce(Fifothresh) -> Fifothresh>(&self, f: F) {
        let tmp = Fifothresh(self.rw.read(REG_FIFOTHRESH));
        self.rw.write(REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn packetconfig2(&self) -> Packetconfig2 {
        Packetconfig2(self.rw.read(REG_PACKETCONFIG2))
    }
    pub fn set_packetconfig2(&self, value: Packetconfig2) {
        self.rw.write(REG_PACKETCONFIG2, value.0)
    }
    pub fn with_packetconfig2<F: FnOnce(Packetconfig2) -> Packetconfig2>(&self, f: F) {
        let tmp = Packetconfig2(self.rw.read(REG_PACKETCONFIG2));
        self.rw.write(REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn aeskey(&self, index: usize) -> Aeskey {
        assert!(index < 16);
        Aeskey(self.rw.read(REG_AESKEY + index as u8))
    }
    pub fn set_aeskey(&self, index: usize, value: Aeskey) {
        assert!(index < 16);
        self.rw.write(REG_AESKEY + index as u8, value.0)
    }
    pub fn with_aeskey<F: FnOnce(Aeskey) -> Aeskey>(&self, index: usize, f: F) {
        assert!(index < 16);
        let tmp = Aeskey(self.rw.read(REG_AESKEY + index as u8));
        self.rw.write(REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn temp1(&self) -> Temp1 {
        Temp1(self.rw.read(REG_TEMP1))
    }
    pub fn set_temp1(&self, value: Temp1) {
        self.rw.write(REG_TEMP1, value.0)
    }
    pub fn with_temp1<F: FnOnce(Temp1) -> Temp1>(&self, f: F) {
        let tmp = Temp1(self.rw.read(REG_TEMP1));
        self.rw.write(REG_TEMP1, f(tmp).0)
    }

    pub fn temp2(&self) -> Temp2 {
        Temp2(self.rw.read(REG_TEMP2))
    }
    pub fn set_temp2(&self, value: Temp2) {
        self.rw.write(REG_TEMP2, value.0)
    }
    pub fn with_temp2<F: FnOnce(Temp2) -> Temp2>(&self, f: F) {
        let tmp = Temp2(self.rw.read(REG_TEMP2));
        self.rw.write(REG_TEMP2, f(tmp).0)
    }

    pub fn testlna(&self) -> Testlna {
        Testlna(self.rw.read(REG_TESTLNA))
    }
    pub fn set_testlna(&self, value: Testlna) {
        self.rw.write(REG_TESTLNA, value.0)
    }
    pub fn with_testlna<F: FnOnce(Testlna) -> Testlna>(&self, f: F) {
        let tmp = Testlna(self.rw.read(REG_TESTLNA));
        self.rw.write(REG_TESTLNA, f(tmp).0)
    }

    pub fn testpa1(&self) -> Testpa1 {
        Testpa1(self.rw.read(REG_TESTPA1))
    }
    pub fn set_testpa1(&self, value: Testpa1) {
        self.rw.write(REG_TESTPA1, value.0)
    }
    pub fn with_testpa1<F: FnOnce(Testpa1) -> Testpa1>(&self, f: F) {
        let tmp = Testpa1(self.rw.read(REG_TESTPA1));
        self.rw.write(REG_TESTPA1, f(tmp).0)
    }

    pub fn testpa2(&self) -> Testpa2 {
        Testpa2(self.rw.read(REG_TESTPA2))
    }
    pub fn set_testpa2(&self, value: Testpa2) {
        self.rw.write(REG_TESTPA2, value.0)
    }
    pub fn with_testpa2<F: FnOnce(Testpa2) -> Testpa2>(&self, f: F) {
        let tmp = Testpa2(self.rw.read(REG_TESTPA2));
        self.rw.write(REG_TESTPA2, f(tmp).0)
    }

    pub fn testdagc(&self) -> Testdagc {
        Testdagc(self.rw.read(REG_TESTDAGC))
    }
    pub fn set_testdagc(&self, value: Testdagc) {
        self.rw.write(REG_TESTDAGC, value.0)
    }
    pub fn with_testdagc<F: FnOnce(Testdagc) -> Testdagc>(&self, f: F) {
        let tmp = Testdagc(self.rw.read(REG_TESTDAGC));
        self.rw.write(REG_TESTDAGC, f(tmp).0)
    }

    pub fn testafc(&self) -> Testafc {
        Testafc(self.rw.read(REG_TESTAFC))
    }
    pub fn set_testafc(&self, value: Testafc) {
        self.rw.write(REG_TESTAFC, value.0)
    }
    pub fn with_testafc<F: FnOnce(Testafc) -> Testafc>(&self, f: F) {
        let tmp = Testafc(self.rw.read(REG_TESTAFC));
        self.rw.write(REG_TESTAFC, f(tmp).0)
    }

}

impl<RW: TryReadWrite<Addr=u8, Value=u8>> Rfm69<RW> {
    pub fn try_fifo(&self) -> Result<Fifo, RW::Error> {
        Ok(Fifo(self.rw.try_read(REG_FIFO)?))
    }
    pub fn try_set_fifo(&self, value: Fifo) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FIFO, value.0)
    }
    pub fn try_with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Fifo(self.rw.try_read(REG_FIFO)?);
        self.rw.try_write(REG_FIFO, f(tmp).0)
    }

    pub fn try_opmode(&self) -> Result<Opmode, RW::Error> {
        Ok(Opmode(self.rw.try_read(REG_OPMODE)?))
    }
    pub fn try_set_opmode(&self, value: Opmode) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OPMODE, value.0)
    }
    pub fn try_with_opmode<F: FnOnce(Opmode) -> Opmode>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Opmode(self.rw.try_read(REG_OPMODE)?);
        self.rw.try_write(REG_OPMODE, f(tmp).0)
    }

    pub fn try_datamodul(&self) -> Result<Datamodul, RW::Error> {
        Ok(Datamodul(self.rw.try_read(REG_DATAMODUL)?))
    }
    pub fn try_set_datamodul(&self, value: Datamodul) -> Result<(), RW::Error> {
        self.rw.try_write(REG_DATAMODUL, value.0)
    }
    pub fn try_with_datamodul<F: FnOnce(Datamodul) -> Datamodul>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Datamodul(self.rw.try_read(REG_DATAMODUL)?);
        self.rw.try_write(REG_DATAMODUL, f(tmp).0)
    }

    pub fn try_bitratemsb(&self) -> Result<Bitratemsb, RW::Error> {
        Ok(Bitratemsb(self.rw.try_read(REG_BITRATEMSB)?))
    }
    pub fn try_set_bitratemsb(&self, value: Bitratemsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_BITRATEMSB, value.0)
    }
    pub fn try_with_bitratemsb<F: FnOnce(Bitratemsb) -> Bitratemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Bitratemsb(self.rw.try_read(REG_BITRATEMSB)?);
        self.rw.try_write(REG_BITRATEMSB, f(tmp).0)
    }

    pub fn try_bitratelsb(&self) -> Result<Bitratelsb, RW::Error> {
        Ok(Bitratelsb(self.rw.try_read(REG_BITRATELSB)?))
    }
    pub fn try_set_bitratelsb(&self, value: Bitratelsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_BITRATELSB, value.0)
    }
    pub fn try_with_bitratelsb<F: FnOnce(Bitratelsb) -> Bitratelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Bitratelsb(self.rw.try_read(REG_BITRATELSB)?);
        self.rw.try_write(REG_BITRATELSB, f(tmp).0)
    }

    pub fn try_fdevmsb(&self) -> Result<Fdevmsb, RW::Error> {
        Ok(Fdevmsb(self.rw.try_read(REG_FDEVMSB)?))
    }
    pub fn try_set_fdevmsb(&self, value: Fdevmsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FDEVMSB, value.0)
    }
    pub fn try_with_fdevmsb<F: FnOnce(Fdevmsb) -> Fdevmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Fdevmsb(self.rw.try_read(REG_FDEVMSB)?);
        self.rw.try_write(REG_FDEVMSB, f(tmp).0)
    }

    pub fn try_fdevlsb(&self) -> Result<Fdevlsb, RW::Error> {
        Ok(Fdevlsb(self.rw.try_read(REG_FDEVLSB)?))
    }
    pub fn try_set_fdevlsb(&self, value: Fdevlsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FDEVLSB, value.0)
    }
    pub fn try_with_fdevlsb<F: FnOnce(Fdevlsb) -> Fdevlsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Fdevlsb(self.rw.try_read(REG_FDEVLSB)?);
        self.rw.try_write(REG_FDEVLSB, f(tmp).0)
    }

    pub fn try_frfmsb(&self) -> Result<Frfmsb, RW::Error> {
        Ok(Frfmsb(self.rw.try_read(REG_FRFMSB)?))
    }
    pub fn try_set_frfmsb(&self, value: Frfmsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FRFMSB, value.0)
    }
    pub fn try_with_frfmsb<F: FnOnce(Frfmsb) -> Frfmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Frfmsb(self.rw.try_read(REG_FRFMSB)?);
        self.rw.try_write(REG_FRFMSB, f(tmp).0)
    }

    pub fn try_frfmid(&self) -> Result<Frfmid, RW::Error> {
        Ok(Frfmid(self.rw.try_read(REG_FRFMID)?))
    }
    pub fn try_set_frfmid(&self, value: Frfmid) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FRFMID, value.0)
    }
    pub fn try_with_frfmid<F: FnOnce(Frfmid) -> Frfmid>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Frfmid(self.rw.try_read(REG_FRFMID)?);
        self.rw.try_write(REG_FRFMID, f(tmp).0)
    }

    pub fn try_frflsb(&self) -> Result<Frflsb, RW::Error> {
        Ok(Frflsb(self.rw.try_read(REG_FRFLSB)?))
    }
    pub fn try_set_frflsb(&self, value: Frflsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FRFLSB, value.0)
    }
    pub fn try_with_frflsb<F: FnOnce(Frflsb) -> Frflsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Frflsb(self.rw.try_read(REG_FRFLSB)?);
        self.rw.try_write(REG_FRFLSB, f(tmp).0)
    }

    pub fn try_osc1(&self) -> Result<Osc1, RW::Error> {
        Ok(Osc1(self.rw.try_read(REG_OSC1)?))
    }
    pub fn try_set_osc1(&self, value: Osc1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OSC1, value.0)
    }
    pub fn try_with_osc1<F: FnOnce(Osc1) -> Osc1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Osc1(self.rw.try_read(REG_OSC1)?);
        self.rw.try_write(REG_OSC1, f(tmp).0)
    }

    pub fn try_afcctrl(&self) -> Result<Afcctrl, RW::Error> {
        Ok(Afcctrl(self.rw.try_read(REG_AFCCTRL)?))
    }
    pub fn try_set_afcctrl(&self, value: Afcctrl) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AFCCTRL, value.0)
    }
    pub fn try_with_afcctrl<F: FnOnce(Afcctrl) -> Afcctrl>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Afcctrl(self.rw.try_read(REG_AFCCTRL)?);
        self.rw.try_write(REG_AFCCTRL, f(tmp).0)
    }

    pub fn try_listen1(&self) -> Result<Listen1, RW::Error> {
        Ok(Listen1(self.rw.try_read(REG_LISTEN1)?))
    }
    pub fn try_set_listen1(&self, value: Listen1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_LISTEN1, value.0)
    }
    pub fn try_with_listen1<F: FnOnce(Listen1) -> Listen1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Listen1(self.rw.try_read(REG_LISTEN1)?);
        self.rw.try_write(REG_LISTEN1, f(tmp).0)
    }

    pub fn try_listen2(&self) -> Result<Listen2, RW::Error> {
        Ok(Listen2(self.rw.try_read(REG_LISTEN2)?))
    }
    pub fn try_set_listen2(&self, value: Listen2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_LISTEN2, value.0)
    }
    pub fn try_with_listen2<F: FnOnce(Listen2) -> Listen2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Listen2(self.rw.try_read(REG_LISTEN2)?);
        self.rw.try_write(REG_LISTEN2, f(tmp).0)
    }

    pub fn try_listen3(&self) -> Result<Listen3, RW::Error> {
        Ok(Listen3(self.rw.try_read(REG_LISTEN3)?))
    }
    pub fn try_set_listen3(&self, value: Listen3) -> Result<(), RW::Error> {
        self.rw.try_write(REG_LISTEN3, value.0)
    }
    pub fn try_with_listen3<F: FnOnce(Listen3) -> Listen3>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Listen3(self.rw.try_read(REG_LISTEN3)?);
        self.rw.try_write(REG_LISTEN3, f(tmp).0)
    }

    pub fn try_version(&self) -> Result<Version, RW::Error> {
        Ok(Version(self.rw.try_read(REG_VERSION)?))
    }
    pub fn try_set_version(&self, value: Version) -> Result<(), RW::Error> {
        self.rw.try_write(REG_VERSION, value.0)
    }
    pub fn try_with_version<F: FnOnce(Version) -> Version>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Version(self.rw.try_read(REG_VERSION)?);
        self.rw.try_write(REG_VERSION, f(tmp).0)
    }

    pub fn try_palevel(&self) -> Result<Palevel, RW::Error> {
        Ok(Palevel(self.rw.try_read(REG_PALEVEL)?))
    }
    pub fn try_set_palevel(&self, value: Palevel) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PALEVEL, value.0)
    }
    pub fn try_with_palevel<F: FnOnce(Palevel) -> Palevel>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Palevel(self.rw.try_read(REG_PALEVEL)?);
        self.rw.try_write(REG_PALEVEL, f(tmp).0)
    }

    pub fn try_paramp(&self) -> Result<Paramp, RW::Error> {
        Ok(Paramp(self.rw.try_read(REG_PARAMP)?))
    }
    pub fn try_set_paramp(&self, value: Paramp) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PARAMP, value.0)
    }
    pub fn try_with_paramp<F: FnOnce(Paramp) -> Paramp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Paramp(self.rw.try_read(REG_PARAMP)?);
        self.rw.try_write(REG_PARAMP, f(tmp).0)
    }

    pub fn try_ocp(&self) -> Result<Ocp, RW::Error> {
        Ok(Ocp(self.rw.try_read(REG_OCP)?))
    }
    pub fn try_set_ocp(&self, value: Ocp) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OCP, value.0)
    }
    pub fn try_with_ocp<F: FnOnce(Ocp) -> Ocp>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Ocp(self.rw.try_read(REG_OCP)?);
        self.rw.try_write(REG_OCP, f(tmp).0)
    }

    pub fn try_lna(&self) -> Result<Lna, RW::Error> {
        Ok(Lna(self.rw.try_read(REG_LNA)?))
    }
    pub fn try_set_lna(&self, value: Lna) -> Result<(), RW::Error> {
        self.rw.try_write(REG_LNA, value.0)
    }
    pub fn try_with_lna<F: FnOnce(Lna) -> Lna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Lna(self.rw.try_read(REG_LNA)?);
        self.rw.try_write(REG_LNA, f(tmp).0)
    }

    pub fn try_rxbw(&self) -> Result<Rxbw, RW::Error> {
        Ok(Rxbw(self.rw.try_read(REG_RXBW)?))
    }
    pub fn try_set_rxbw(&self, value: Rxbw) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RXBW, value.0)
    }
    pub fn try_with_rxbw<F: FnOnce(Rxbw) -> Rxbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rxbw(self.rw.try_read(REG_RXBW)?);
        self.rw.try_write(REG_RXBW, f(tmp).0)
    }

    pub fn try_afcbw(&self) -> Result<Afcbw, RW::Error> {
        Ok(Afcbw(self.rw.try_read(REG_AFCBW)?))
    }
    pub fn try_set_afcbw(&self, value: Afcbw) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AFCBW, value.0)
    }
    pub fn try_with_afcbw<F: FnOnce(Afcbw) -> Afcbw>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Afcbw(self.rw.try_read(REG_AFCBW)?);
        self.rw.try_write(REG_AFCBW, f(tmp).0)
    }

    pub fn try_ookpeak(&self) -> Result<Ookpeak, RW::Error> {
        Ok(Ookpeak(self.rw.try_read(REG_OOKPEAK)?))
    }
    pub fn try_set_ookpeak(&self, value: Ookpeak) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OOKPEAK, value.0)
    }
    pub fn try_with_ookpeak<F: FnOnce(Ookpeak) -> Ookpeak>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Ookpeak(self.rw.try_read(REG_OOKPEAK)?);
        self.rw.try_write(REG_OOKPEAK, f(tmp).0)
    }

    pub fn try_ookavg(&self) -> Result<Ookavg, RW::Error> {
        Ok(Ookavg(self.rw.try_read(REG_OOKAVG)?))
    }
    pub fn try_set_ookavg(&self, value: Ookavg) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OOKAVG, value.0)
    }
    pub fn try_with_ookavg<F: FnOnce(Ookavg) -> Ookavg>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Ookavg(self.rw.try_read(REG_OOKAVG)?);
        self.rw.try_write(REG_OOKAVG, f(tmp).0)
    }

    pub fn try_ookfix(&self) -> Result<Ookfix, RW::Error> {
        Ok(Ookfix(self.rw.try_read(REG_OOKFIX)?))
    }
    pub fn try_set_ookfix(&self, value: Ookfix) -> Result<(), RW::Error> {
        self.rw.try_write(REG_OOKFIX, value.0)
    }
    pub fn try_with_ookfix<F: FnOnce(Ookfix) -> Ookfix>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Ookfix(self.rw.try_read(REG_OOKFIX)?);
        self.rw.try_write(REG_OOKFIX, f(tmp).0)
    }

    pub fn try_afcfei(&self) -> Result<Afcfei, RW::Error> {
        Ok(Afcfei(self.rw.try_read(REG_AFCFEI)?))
    }
    pub fn try_set_afcfei(&self, value: Afcfei) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AFCFEI, value.0)
    }
    pub fn try_with_afcfei<F: FnOnce(Afcfei) -> Afcfei>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Afcfei(self.rw.try_read(REG_AFCFEI)?);
        self.rw.try_write(REG_AFCFEI, f(tmp).0)
    }

    pub fn try_afcmsb(&self) -> Result<Afcmsb, RW::Error> {
        Ok(Afcmsb(self.rw.try_read(REG_AFCMSB)?))
    }
    pub fn try_set_afcmsb(&self, value: Afcmsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AFCMSB, value.0)
    }
    pub fn try_with_afcmsb<F: FnOnce(Afcmsb) -> Afcmsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Afcmsb(self.rw.try_read(REG_AFCMSB)?);
        self.rw.try_write(REG_AFCMSB, f(tmp).0)
    }

    pub fn try_afclsb(&self) -> Result<Afclsb, RW::Error> {
        Ok(Afclsb(self.rw.try_read(REG_AFCLSB)?))
    }
    pub fn try_set_afclsb(&self, value: Afclsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AFCLSB, value.0)
    }
    pub fn try_with_afclsb<F: FnOnce(Afclsb) -> Afclsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Afclsb(self.rw.try_read(REG_AFCLSB)?);
        self.rw.try_write(REG_AFCLSB, f(tmp).0)
    }

    pub fn try_feimsb(&self) -> Result<Feimsb, RW::Error> {
        Ok(Feimsb(self.rw.try_read(REG_FEIMSB)?))
    }
    pub fn try_set_feimsb(&self, value: Feimsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FEIMSB, value.0)
    }
    pub fn try_with_feimsb<F: FnOnce(Feimsb) -> Feimsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Feimsb(self.rw.try_read(REG_FEIMSB)?);
        self.rw.try_write(REG_FEIMSB, f(tmp).0)
    }

    pub fn try_feilsb(&self) -> Result<Feilsb, RW::Error> {
        Ok(Feilsb(self.rw.try_read(REG_FEILSB)?))
    }
    pub fn try_set_feilsb(&self, value: Feilsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FEILSB, value.0)
    }
    pub fn try_with_feilsb<F: FnOnce(Feilsb) -> Feilsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Feilsb(self.rw.try_read(REG_FEILSB)?);
        self.rw.try_write(REG_FEILSB, f(tmp).0)
    }

    pub fn try_rssiconfig(&self) -> Result<Rssiconfig, RW::Error> {
        Ok(Rssiconfig(self.rw.try_read(REG_RSSICONFIG)?))
    }
    pub fn try_set_rssiconfig(&self, value: Rssiconfig) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RSSICONFIG, value.0)
    }
    pub fn try_with_rssiconfig<F: FnOnce(Rssiconfig) -> Rssiconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rssiconfig(self.rw.try_read(REG_RSSICONFIG)?);
        self.rw.try_write(REG_RSSICONFIG, f(tmp).0)
    }

    pub fn try_rssivalue(&self) -> Result<Rssivalue, RW::Error> {
        Ok(Rssivalue(self.rw.try_read(REG_RSSIVALUE)?))
    }
    pub fn try_set_rssivalue(&self, value: Rssivalue) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RSSIVALUE, value.0)
    }
    pub fn try_with_rssivalue<F: FnOnce(Rssivalue) -> Rssivalue>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rssivalue(self.rw.try_read(REG_RSSIVALUE)?);
        self.rw.try_write(REG_RSSIVALUE, f(tmp).0)
    }

    pub fn try_diomapping1(&self) -> Result<Diomapping1, RW::Error> {
        Ok(Diomapping1(self.rw.try_read(REG_DIOMAPPING1)?))
    }
    pub fn try_set_diomapping1(&self, value: Diomapping1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_DIOMAPPING1, value.0)
    }
    pub fn try_with_diomapping1<F: FnOnce(Diomapping1) -> Diomapping1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Diomapping1(self.rw.try_read(REG_DIOMAPPING1)?);
        self.rw.try_write(REG_DIOMAPPING1, f(tmp).0)
    }

    pub fn try_diomapping2(&self) -> Result<Diomapping2, RW::Error> {
        Ok(Diomapping2(self.rw.try_read(REG_DIOMAPPING2)?))
    }
    pub fn try_set_diomapping2(&self, value: Diomapping2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_DIOMAPPING2, value.0)
    }
    pub fn try_with_diomapping2<F: FnOnce(Diomapping2) -> Diomapping2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Diomapping2(self.rw.try_read(REG_DIOMAPPING2)?);
        self.rw.try_write(REG_DIOMAPPING2, f(tmp).0)
    }

    pub fn try_irqflags1(&self) -> Result<Irqflags1, RW::Error> {
        Ok(Irqflags1(self.rw.try_read(REG_IRQFLAGS1)?))
    }
    pub fn try_set_irqflags1(&self, value: Irqflags1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_IRQFLAGS1, value.0)
    }
    pub fn try_with_irqflags1<F: FnOnce(Irqflags1) -> Irqflags1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Irqflags1(self.rw.try_read(REG_IRQFLAGS1)?);
        self.rw.try_write(REG_IRQFLAGS1, f(tmp).0)
    }

    pub fn try_irqflags2(&self) -> Result<Irqflags2, RW::Error> {
        Ok(Irqflags2(self.rw.try_read(REG_IRQFLAGS2)?))
    }
    pub fn try_set_irqflags2(&self, value: Irqflags2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_IRQFLAGS2, value.0)
    }
    pub fn try_with_irqflags2<F: FnOnce(Irqflags2) -> Irqflags2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Irqflags2(self.rw.try_read(REG_IRQFLAGS2)?);
        self.rw.try_write(REG_IRQFLAGS2, f(tmp).0)
    }

    pub fn try_rssithresh(&self) -> Result<Rssithresh, RW::Error> {
        Ok(Rssithresh(self.rw.try_read(REG_RSSITHRESH)?))
    }
    pub fn try_set_rssithresh(&self, value: Rssithresh) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RSSITHRESH, value.0)
    }
    pub fn try_with_rssithresh<F: FnOnce(Rssithresh) -> Rssithresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rssithresh(self.rw.try_read(REG_RSSITHRESH)?);
        self.rw.try_write(REG_RSSITHRESH, f(tmp).0)
    }

    pub fn try_rxtimeout1(&self) -> Result<Rxtimeout1, RW::Error> {
        Ok(Rxtimeout1(self.rw.try_read(REG_RXTIMEOUT1)?))
    }
    pub fn try_set_rxtimeout1(&self, value: Rxtimeout1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RXTIMEOUT1, value.0)
    }
    pub fn try_with_rxtimeout1<F: FnOnce(Rxtimeout1) -> Rxtimeout1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rxtimeout1(self.rw.try_read(REG_RXTIMEOUT1)?);
        self.rw.try_write(REG_RXTIMEOUT1, f(tmp).0)
    }

    pub fn try_rxtimeout2(&self) -> Result<Rxtimeout2, RW::Error> {
        Ok(Rxtimeout2(self.rw.try_read(REG_RXTIMEOUT2)?))
    }
    pub fn try_set_rxtimeout2(&self, value: Rxtimeout2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_RXTIMEOUT2, value.0)
    }
    pub fn try_with_rxtimeout2<F: FnOnce(Rxtimeout2) -> Rxtimeout2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Rxtimeout2(self.rw.try_read(REG_RXTIMEOUT2)?);
        self.rw.try_write(REG_RXTIMEOUT2, f(tmp).0)
    }

    pub fn try_preamblemsb(&self) -> Result<Preamblemsb, RW::Error> {
        Ok(Preamblemsb(self.rw.try_read(REG_PREAMBLEMSB)?))
    }
    pub fn try_set_preamblemsb(&self, value: Preamblemsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PREAMBLEMSB, value.0)
    }
    pub fn try_with_preamblemsb<F: FnOnce(Preamblemsb) -> Preamblemsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Preamblemsb(self.rw.try_read(REG_PREAMBLEMSB)?);
        self.rw.try_write(REG_PREAMBLEMSB, f(tmp).0)
    }

    pub fn try_preamblelsb(&self) -> Result<Preamblelsb, RW::Error> {
        Ok(Preamblelsb(self.rw.try_read(REG_PREAMBLELSB)?))
    }
    pub fn try_set_preamblelsb(&self, value: Preamblelsb) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PREAMBLELSB, value.0)
    }
    pub fn try_with_preamblelsb<F: FnOnce(Preamblelsb) -> Preamblelsb>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Preamblelsb(self.rw.try_read(REG_PREAMBLELSB)?);
        self.rw.try_write(REG_PREAMBLELSB, f(tmp).0)
    }

    pub fn try_syncconfig(&self) -> Result<Syncconfig, RW::Error> {
        Ok(Syncconfig(self.rw.try_read(REG_SYNCCONFIG)?))
    }
    pub fn try_set_syncconfig(&self, value: Syncconfig) -> Result<(), RW::Error> {
        self.rw.try_write(REG_SYNCCONFIG, value.0)
    }
    pub fn try_with_syncconfig<F: FnOnce(Syncconfig) -> Syncconfig>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Syncconfig(self.rw.try_read(REG_SYNCCONFIG)?);
        self.rw.try_write(REG_SYNCCONFIG, f(tmp).0)
    }

    pub fn try_syncvalue(&self, index: usize) -> Result<Syncvalue, RW::Error> {
        assert!(index < 8);
        Ok(Syncvalue(self.rw.try_read(REG_SYNCVALUE + index as u8)?))
    }
    pub fn try_set_syncvalue(&self, index: usize, value: Syncvalue) -> Result<(), RW::Error> {
        assert!(index < 8);
        self.rw.try_write(REG_SYNCVALUE + index as u8, value.0)
    }
    pub fn try_with_syncvalue<F: FnOnce(Syncvalue) -> Syncvalue>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 8);
        let tmp = Syncvalue(self.rw.try_read(REG_SYNCVALUE + index as u8)?);
        self.rw.try_write(REG_SYNCVALUE + index as u8, f(tmp).0)
    }

    pub fn try_packetconfig1(&self) -> Result<Packetconfig1, RW::Error> {
        Ok(Packetconfig1(self.rw.try_read(REG_PACKETCONFIG1)?))
    }
    pub fn try_set_packetconfig1(&self, value: Packetconfig1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PACKETCONFIG1, value.0)
    }
    pub fn try_with_packetconfig1<F: FnOnce(Packetconfig1) -> Packetconfig1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Packetconfig1(self.rw.try_read(REG_PACKETCONFIG1)?);
        self.rw.try_write(REG_PACKETCONFIG1, f(tmp).0)
    }

    pub fn try_payloadlength(&self) -> Result<Payloadlength, RW::Error> {
        Ok(Payloadlength(self.rw.try_read(REG_PAYLOADLENGTH)?))
    }
    pub fn try_set_payloadlength(&self, value: Payloadlength) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PAYLOADLENGTH, value.0)
    }
    pub fn try_with_payloadlength<F: FnOnce(Payloadlength) -> Payloadlength>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Payloadlength(self.rw.try_read(REG_PAYLOADLENGTH)?);
        self.rw.try_write(REG_PAYLOADLENGTH, f(tmp).0)
    }

    pub fn try_nodeadrs(&self) -> Result<Nodeadrs, RW::Error> {
        Ok(Nodeadrs(self.rw.try_read(REG_NODEADRS)?))
    }
    pub fn try_set_nodeadrs(&self, value: Nodeadrs) -> Result<(), RW::Error> {
        self.rw.try_write(REG_NODEADRS, value.0)
    }
    pub fn try_with_nodeadrs<F: FnOnce(Nodeadrs) -> Nodeadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Nodeadrs(self.rw.try_read(REG_NODEADRS)?);
        self.rw.try_write(REG_NODEADRS, f(tmp).0)
    }

    pub fn try_broadcastadrs(&self) -> Result<Broadcastadrs, RW::Error> {
        Ok(Broadcastadrs(self.rw.try_read(REG_BROADCASTADRS)?))
    }
    pub fn try_set_broadcastadrs(&self, value: Broadcastadrs) -> Result<(), RW::Error> {
        self.rw.try_write(REG_BROADCASTADRS, value.0)
    }
    pub fn try_with_broadcastadrs<F: FnOnce(Broadcastadrs) -> Broadcastadrs>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Broadcastadrs(self.rw.try_read(REG_BROADCASTADRS)?);
        self.rw.try_write(REG_BROADCASTADRS, f(tmp).0)
    }

    pub fn try_automodes(&self) -> Result<Automodes, RW::Error> {
        Ok(Automodes(self.rw.try_read(REG_AUTOMODES)?))
    }
    pub fn try_set_automodes(&self, value: Automodes) -> Result<(), RW::Error> {
        self.rw.try_write(REG_AUTOMODES, value.0)
    }
    pub fn try_with_automodes<F: FnOnce(Automodes) -> Automodes>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Automodes(self.rw.try_read(REG_AUTOMODES)?);
        self.rw.try_write(REG_AUTOMODES, f(tmp).0)
    }

    pub fn try_fifothresh(&self) -> Result<Fifothresh, RW::Error> {
        Ok(Fifothresh(self.rw.try_read(REG_FIFOTHRESH)?))
    }
    pub fn try_set_fifothresh(&self, value: Fifothresh) -> Result<(), RW::Error> {
        self.rw.try_write(REG_FIFOTHRESH, value.0)
    }
    pub fn try_with_fifothresh<F: FnOnce(Fifothresh) -> Fifothresh>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Fifothresh(self.rw.try_read(REG_FIFOTHRESH)?);
        self.rw.try_write(REG_FIFOTHRESH, f(tmp).0)
    }

    pub fn try_packetconfig2(&self) -> Result<Packetconfig2, RW::Error> {
        Ok(Packetconfig2(self.rw.try_read(REG_PACKETCONFIG2)?))
    }
    pub fn try_set_packetconfig2(&self, value: Packetconfig2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_PACKETCONFIG2, value.0)
    }
    pub fn try_with_packetconfig2<F: FnOnce(Packetconfig2) -> Packetconfig2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Packetconfig2(self.rw.try_read(REG_PACKETCONFIG2)?);
        self.rw.try_write(REG_PACKETCONFIG2, f(tmp).0)
    }

    pub fn try_aeskey(&self, index: usize) -> Result<Aeskey, RW::Error> {
        assert!(index < 16);
        Ok(Aeskey(self.rw.try_read(REG_AESKEY + index as u8)?))
    }
    pub fn try_set_aeskey(&self, index: usize, value: Aeskey) -> Result<(), RW::Error> {
        assert!(index < 16);
        self.rw.try_write(REG_AESKEY + index as u8, value.0)
    }
    pub fn try_with_aeskey<F: FnOnce(Aeskey) -> Aeskey>(&self, index: usize, f: F) -> Result<(), RW::Error> {
        assert!(index < 16);
        let tmp = Aeskey(self.rw.try_read(REG_AESKEY + index as u8)?);
        self.rw.try_write(REG_AESKEY + index as u8, f(tmp).0)
    }

    pub fn try_temp1(&self) -> Result<Temp1, RW::Error> {
        Ok(Temp1(self.rw.try_read(REG_TEMP1)?))
    }
    pub fn try_set_temp1(&self, value: Temp1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TEMP1, value.0)
    }
    pub fn try_with_temp1<F: FnOnce(Temp1) -> Temp1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Temp1(self.rw.try_read(REG_TEMP1)?);
        self.rw.try_write(REG_TEMP1, f(tmp).0)
    }

    pub fn try_temp2(&self) -> Result<Temp2, RW::Error> {
        Ok(Temp2(self.rw.try_read(REG_TEMP2)?))
    }
    pub fn try_set_temp2(&self, value: Temp2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TEMP2, value.0)
    }
    pub fn try_with_temp2<F: FnOnce(Temp2) -> Temp2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Temp2(self.rw.try_read(REG_TEMP2)?);
        self.rw.try_write(REG_TEMP2, f(tmp).0)
    }

    pub fn try_testlna(&self) -> Result<Testlna, RW::Error> {
        Ok(Testlna(self.rw.try_read(REG_TESTLNA)?))
    }
    pub fn try_set_testlna(&self, value: Testlna) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TESTLNA, value.0)
    }
    pub fn try_with_testlna<F: FnOnce(Testlna) -> Testlna>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Testlna(self.rw.try_read(REG_TESTLNA)?);
        self.rw.try_write(REG_TESTLNA, f(tmp).0)
    }

    pub fn try_testpa1(&self) -> Result<Testpa1, RW::Error> {
        Ok(Testpa1(self.rw.try_read(REG_TESTPA1)?))
    }
    pub fn try_set_testpa1(&self, value: Testpa1) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TESTPA1, value.0)
    }
    pub fn try_with_testpa1<F: FnOnce(Testpa1) -> Testpa1>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Testpa1(self.rw.try_read(REG_TESTPA1)?);
        self.rw.try_write(REG_TESTPA1, f(tmp).0)
    }

    pub fn try_testpa2(&self) -> Result<Testpa2, RW::Error> {
        Ok(Testpa2(self.rw.try_read(REG_TESTPA2)?))
    }
    pub fn try_set_testpa2(&self, value: Testpa2) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TESTPA2, value.0)
    }
    pub fn try_with_testpa2<F: FnOnce(Testpa2) -> Testpa2>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Testpa2(self.rw.try_read(REG_TESTPA2)?);
        self.rw.try_write(REG_TESTPA2, f(tmp).0)
    }

    pub fn try_testdagc(&self) -> Result<Testdagc, RW::Error> {
        Ok(Testdagc(self.rw.try_read(REG_TESTDAGC)?))
    }
    pub fn try_set_testdagc(&self, value: Testdagc) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TESTDAGC, value.0)
    }
    pub fn try_with_testdagc<F: FnOnce(Testdagc) -> Testdagc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Testdagc(self.rw.try_read(REG_TESTDAGC)?);
        self.rw.try_write(REG_TESTDAGC, f(tmp).0)
    }

    pub fn try_testafc(&self) -> Result<Testafc, RW::Error> {
        Ok(Testafc(self.rw.try_read(REG_TESTAFC)?))
    }
    pub fn try_set_testafc(&self, value: Testafc) -> Result<(), RW::Error> {
        self.rw.try_write(REG_TESTAFC, value.0)
    }
    pub fn try_with_testafc<F: FnOnce(Testafc) -> Testafc>(&self, f: F) -> Result<(), RW::Error> {
        let tmp = Testafc(self.rw.try_read(REG_TESTAFC)?);
        self.rw.try_write(REG_TESTAFC, f(tmp).0)
    }

}

pub struct Fifo(pub u8);

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

pub struct Opmode(pub u8);

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

pub struct Datamodul(pub u8);

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

pub struct Bitratemsb(pub u8);

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

pub struct Bitratelsb(pub u8);

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

pub struct Fdevmsb(pub u8);

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

pub struct Fdevlsb(pub u8);

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

pub struct Frfmsb(pub u8);

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

pub struct Frfmid(pub u8);

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

pub struct Frflsb(pub u8);

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

pub struct Osc1(pub u8);

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

pub struct Afcctrl(pub u8);

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

pub struct Listen1(pub u8);

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

pub struct Listen2(pub u8);

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

pub struct Listen3(pub u8);

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

pub struct Version(pub u8);

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

pub struct Palevel(pub u8);

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

pub struct Paramp(pub u8);

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

pub struct Ocp(pub u8);

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

pub struct Lna(pub u8);

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

pub struct Rxbw(pub u8);

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

pub struct Afcbw(pub u8);

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

pub struct Ookpeak(pub u8);

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

pub struct Ookavg(pub u8);

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

pub struct Ookfix(pub u8);

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

pub struct Afcfei(pub u8);

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

pub struct Afcmsb(pub u8);

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

pub struct Afclsb(pub u8);

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

pub struct Feimsb(pub u8);

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

pub struct Feilsb(pub u8);

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

pub struct Rssiconfig(pub u8);

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

pub struct Rssivalue(pub u8);

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

pub struct Diomapping1(pub u8);

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

pub struct Diomapping2(pub u8);

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

pub struct Irqflags1(pub u8);

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

pub struct Irqflags2(pub u8);

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

pub struct Rssithresh(pub u8);

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

pub struct Rxtimeout1(pub u8);

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

pub struct Rxtimeout2(pub u8);

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

pub struct Preamblemsb(pub u8);

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

pub struct Preamblelsb(pub u8);

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

pub struct Syncconfig(pub u8);

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

pub struct Syncvalue(pub u8);

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

pub struct Packetconfig1(pub u8);

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

pub struct Payloadlength(pub u8);

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

pub struct Nodeadrs(pub u8);

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

pub struct Broadcastadrs(pub u8);

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

pub struct Automodes(pub u8);

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

pub struct Fifothresh(pub u8);

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

pub struct Packetconfig2(pub u8);

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

pub struct Aeskey(pub u8);

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

pub struct Temp1(pub u8);

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

pub struct Temp2(pub u8);

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

pub struct Testlna(pub u8);

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

pub struct Testpa1(pub u8);

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

pub struct Testpa2(pub u8);

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

pub struct Testdagc(pub u8);

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

pub struct Testafc(pub u8);

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

