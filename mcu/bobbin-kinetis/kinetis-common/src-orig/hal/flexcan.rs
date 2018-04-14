pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::hal::can::*;
pub use ::chip::flexcan::*;

use bobbin_common::bits::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdAcceptanceMode {
    FormatA = 0b00,
    FormatB = 0b01,
    FormatC = 0b10,
    FormatD = 0b11,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockSource {
    Oscillator = 0b0,
    Peripheral = 0b1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Code {
    RxInactive = 0b0000,
    RxBusy = 0b0001,
    RxFull = 0b0010,
    RxEmpty = 0b0100,
    RxOverrun = 0b0110,
    RxAnswer = 0b1010,
    TxInactive = 0b1000,
    TxAbort = 0b1001,
    TxData = 0b1100,
    TxAnswer = 0b1110,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CanId {
    Std(StandardId),
    Ext(ExtendedId),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct StandardId(pub u16);

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ExtendedId(pub u32);

impl ::core::fmt::Debug for StandardId {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       write!(f, "[{:04x}]", self.0)
   }
}

impl ::core::fmt::Debug for ExtendedId {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       write!(f, "[{:08x}]", self.0)
   }
}

pub fn std_id(value: u16) -> CanId {
    CanId::Std(StandardId(value))
}

pub fn ext_id(value: u32) -> CanId {
    CanId::Ext(ExtendedId(value))
}


#[derive(Debug, Default)]
pub struct Config {
    ctrl1: Ctrl1,
}

impl Config {
    pub fn set_loopback(mut self, value: bool) -> Self {
        self.ctrl1 = self.ctrl1.set_lpb(value);
        self
    }
}


impl Configure<Config> for FlexcanPeriph {
    fn config(&self) -> Config {
        Config {
            ctrl1: Ctrl1(0),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_ctrl1(|_| cfg.ctrl1)
    }
}

impl Enabled for FlexcanPeriph {
    fn enabled(&self) -> bool {        
        !self.mcr().test_mdis()
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_mcr(|r| r.set_mdis(!value))
    }
}

impl FlexcanPeriph {
    pub fn clear_ram(&self, words: usize) -> &Self {
        for i in 0..words {
            self.set_ram(i, |_| Ram(0));            
        }
        self
    }

    pub fn mbuf(&self, index: usize) -> MBuf {
        MBuf { can: *self, index: index }
    }
}

pub struct MBuf {
    can: FlexcanPeriph,
    index: usize,
}

impl ::core::fmt::Debug for MBuf {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       write!(f, "[{}: {:?} {:?}]", self.index, self.mb8h0(), self.mb8h1())
   }
}

impl MBuf {    
    pub fn mb8h0(&self) -> Mb8h0 {
        self.can.mb8h0(self.index)
    }

    pub fn mb8h1(&self) -> Mb8h1 {
        self.can.mb8h1(self.index)
    }

    // FLAG
    pub fn flag(&self) -> bool {
        self.can.iflag1().bufi(self.index) != 0
    }

    pub fn clr_flag(&self) -> &Self {
        self.can.set_iflag1(|_| Iflag1(0).set_bufi(self.index, 1));
        self
    }

    // IDMASK
    pub fn idmask(&self) -> u32 {
        self.can.rximr(self.index).mi().value()
    }
    
    pub fn set_idmask(&self, value: u32) -> &Self {
        self.can.set_rximr(self.index, |_| Rximr(0).set_mi(value));
        self
    }    
    
    pub fn edl(&self) -> bool {
        self.can.mb8h0(self.index).edl() != 0
    }

    pub fn set_edl(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_edl(value));
        self
    }

    // BRS

    pub fn brs(&self) -> bool {
        self.can.mb8h0(self.index).brs() != 0
    }

    pub fn set_brs(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_brs(value));
        self
    }    

    // ESI

    pub fn esi(&self) -> bool {
        self.can.mb8h0(self.index).esi() != 0
    }

    pub fn set_esi(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_esi(value));
        self
    }

    // CODE

    pub fn code(&self) -> Code {
        match self.can.mb8h0(self.index).code().value() {
            0b0000 => Code::RxInactive,
            0b0010 => Code::RxFull,
            0b0100 => Code::RxEmpty,
            0b0110 => Code::RxOverrun,
            0b1010 => Code::RxAnswer,
            0b1000 => Code::TxInactive,
            0b1001 => Code::TxAbort,
            0b1100 => Code::TxData,
            0b1110 => Code::TxAnswer,                
            n if (n & 1) != 0 => Code::RxBusy,
            n => panic!("Unexpected CODE value: 0b{:04b}", n),                
        }
    }

    pub fn set_code(&self, value: Code) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_code(value as u8));
        self
    }

    // SRR

    pub fn srr(&self) -> bool {
        self.can.mb8h0(self.index).srr() != 0
    }

    pub fn set_srr(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_srr(value));
        self
    }

    // IDE

    pub fn ide(&self) -> bool {
        self.can.mb8h0(self.index).ide() != 0
    }

    pub fn set_ide(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_ide(value));
        self
    }        

    // RTR

    pub fn rtr(&self) -> bool {
        self.can.mb8h0(self.index).rtr() != 0
    }

    pub fn set_rtr(&self, value: bool) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_rtr(value));
        self
    }

    // DLC

    pub fn dlc(&self) -> u8 {
        self.can.mb8h0(self.index).dlc().value()
    }

    pub fn set_dlc(&self, value: u8) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_dlc(value));
        self
    }

    // TIME_STAMP

    pub fn time_stamp(&self) -> u16 {
        self.can.mb8h0(self.index).time_stamp().value()
    }

    pub fn set_time_stamp(&self, value: u16) -> &Self {        
        self.can.with_mb8h0(self.index, |r| r.set_time_stamp(value));
        self
    }    

    // PRIO

    pub fn prio(&self) -> u8 {
        self.can.mb8h1(self.index).prio().value()
    }

    pub fn set_prio(&self, value: u8) -> &Self {
        self.can.with_mb8h1(self.index, |r| r.set_prio(value));
        self
    }        

    // ID

    pub fn id(&self) -> CanId {
        let h0 = self.can.mb8h0(self.index);
        let h1 = self.can.mb8h1(self.index);
        match h0.ide() {
            U1::B0 => CanId::Std(StandardId(h1.id_std().value())),
            U1::B1 => CanId::Ext(ExtendedId(h1.id_ext().value())),
        }
    }

    pub fn set_id(&self, id: CanId) -> &Self {
        self.can.with_mb8h1(self.index, |r| match id {
            CanId::Std(std_id) => r.set_id_std(std_id.0),
            CanId::Ext(ext_id) => r.set_id_ext(ext_id.0),
        });
        let ide = if let CanId::Ext(_) = id { 1 } else { 0 };
        self.can.with_mb8h0(self.index, |r| r.set_ide(ide));
        self
    }

    pub fn id_std(&self) -> u16 {
        self.can.mb8h1(self.index).id_std().value()
    }

    pub fn set_id_std(&self, value: u32) -> &Self {
        self.can.with_mb8h1(self.index, |r| r.set_id_std(value));
        self
    }        

    pub fn id_ext(&self) -> u32 {
        self.can.mb8h1(self.index).id_ext().value()
    }

    pub fn set_id_ext(&self, value: u32) -> &Self {
        self.can.with_mb8h1(self.index, |r| r.set_id_ext(value));
        self
    }            

    pub fn read(&self, buf: &mut [u8]) -> (CanId, usize) {
        let h0 = self.can.mb8h0(self.index);
        let h1 = self.can.mb8h1(self.index);
        let id = match h0.ide() {
            U1::B0 => CanId::Std(StandardId(h1.id_std().value())),
            U1::B1 => CanId::Ext(ExtendedId(h1.id_ext().value())),
        };

        let dlc = h0.dlc() as usize;
        if dlc > 0 {
            let d0 = self.can.mb8d0(self.index).0;
            buf[0] = (d0 >> 24) as u8;
            if dlc > 1 {
                buf[1] = (d0 >> 16) as u8;
            }
            if dlc > 2 {
                buf[2] = (d0 >> 8) as u8;
            }
            if dlc > 3 {
                buf[3] = (d0 >> 0) as u8;
            }
        }
        if dlc > 4 {
            let d1 = self.can.mb8d1(self.index).0;
            buf[4] = (d1 >> 24) as u8;
            if dlc > 5 {
                buf[5] = (d1 >> 16) as u8;
            }
            if dlc > 6 {
                buf[6] = (d1 >> 8) as u8;
            }
            if dlc > 7 {
                buf[7] = (d1 >> 0) as u8;
            }
        }            
        (id, dlc)
    }

    pub fn write(&self, id: CanId, buf: &[u8]) -> &Self {
        assert!(buf.len() <= 8, "Only payloads up to 8 bytes are supported");
        let mut d = [0u8; 8];
        for i in 0..buf.len() {
            d[i] = buf[i];
        }
        self.can.set_mb8d0(self.index, |_| Mb8d0(
            (d[0] as u32) << 24 |
            (d[1] as u32) << 16 |
            (d[2] as u32) << 8 |
            (d[3] as u32)
        ));
        self.can.set_mb8d1(self.index, |_| Mb8d1(
            (d[4] as u32) << 24 |
            (d[5] as u32) << 16 |
            (d[6] as u32) << 8 |
            (d[7] as u32)
        ));
        self.can.with_mb8h1(self.index, |r| match id {
            CanId::Std(std_id) => r.set_id_std(std_id.0),
            CanId::Ext(ext_id) => r.set_id_ext(ext_id.0),
        });
        let ide = if let CanId::Ext(_) = id { 1 } else { 0 };
        self.can.with_mb8h0(self.index, |r| r.set_ide(ide).set_code(Code::TxData as u8).set_dlc(buf.len()));
        self
    }
}