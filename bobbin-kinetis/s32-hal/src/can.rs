use ::chip::can::*;

pub enum IdAcceptanceMode {
    FormatA = 0b00,
    FormatB = 0b01,
    FormatC = 0b10,
    FormatD = 0b11,
}

pub enum ClockSource {
    Oscillator = 0b0,
    Peripheral = 0b1,
}

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum CanId {
    Std(StandardId),
    Ext(ExtendedId),
}

#[derive(Clone, Copy)]
pub struct StandardId(pub u16);

#[derive(Clone, Copy)]
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


pub fn device(can: Can) -> CanDevice {
    CanDevice { can: can }
}

pub struct CanDevice {
    pub can: Can
}

impl CanDevice {
    // MCR

    pub fn mdis(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().mdis() != 0
        }
    }

    pub fn set_mdis(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_mdis(value));
        }
    }

    pub fn frz(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().frz() != 0
        }
    }

    pub fn set_frz(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_frz(value));
        }
    }

    pub fn rfen(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().rfen() != 0
        }
    }

    pub fn set_rfen(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_rfen(value));
        }
    }

    pub fn halt(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().halt() != 0
        }
    }

    pub fn set_halt(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_halt(value));
        }
    }

    pub fn notrdy(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().notrdy() != 0
        }
    }

    pub fn softrst(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().softrst() != 0
        }
    }    

    pub fn set_softrst(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_softrst(value));
        }
    }    

    pub fn frzack(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().frzack() != 0
        }
    }

    pub fn supv(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().supv() != 0
        }
    }

    pub fn set_supv(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_supv(value));
        }
    }

    pub fn wrnen(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().wrnen() != 0
        }
    }

    pub fn set_wrnen(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_wrnen(value));
        }
    }

    pub fn lpmack(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().lpmack() != 0
        }
    }

    pub fn srxdis(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().srxdis() != 0
        }
    }

    pub fn set_srxdis(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_srxdis(value));
        }
    }    

    pub fn irmq(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mcr().irmq() != 0
        }
    }

    pub fn set_irmq(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_irmq(value));
        }
    }    
    

    pub fn idam(&self) -> IdAcceptanceMode {
        let can = self.can;
        unsafe {
            match can.mcr().idam() {
                0b00 => IdAcceptanceMode::FormatA,
                0b01 => IdAcceptanceMode::FormatB,
                0b10 => IdAcceptanceMode::FormatC,
                0b11 => IdAcceptanceMode::FormatD,
                _ => panic!("Invalid IDAM value"),
            }
        }
    }

    pub fn set_idam(&self, value: IdAcceptanceMode) {
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_idam(value as u32));
        }
    }    

    pub fn maxmb(&self) -> u8 {
        let can = self.can;
        unsafe {
            can.mcr().maxmb() as u8
        }
    }

    pub fn set_maxmb(&self, value: u8) {
        let mut can = self.can;
        unsafe {
            can.with_mcr(|r| r.set_maxmb(value as u32))
        }
    }

    // CTRL1

    pub fn presdiv(&self) -> u8 {
        let can = self.can;
        unsafe {
            can.ctrl1().presdiv() as u8
        }
    }

    pub fn set_presdiv(&self, value: u8) {
        let mut can = self.can;
        unsafe {
            can.with_ctrl1(|r| r.set_presdiv(value as u32))
        }
    }

    pub fn clksrc(&self) -> ClockSource {
        let can = self.can;
        unsafe {
            match can.ctrl1().clksrc() {
                0b0 => ClockSource::Oscillator,
                0b1 => ClockSource::Peripheral,
                _ => panic!("Invalid Clocksource value")
            }
        }
    }

    pub fn set_clksrc(&self, value: ClockSource) {
        let mut can = self.can;
        unsafe {
            can.with_ctrl1(|r| r.set_clksrc(value as u32))
        }
    }    

    pub fn lpb(&self) -> bool {
        let can = self.can;
        unsafe {
            can.ctrl1().lpb() != 0
        }
    }

    pub fn set_lpb(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_ctrl1(|r| r.set_lpb(value));
        }
    }        

    // TIMER

    pub fn timer(&self) -> u16 {
        let can = self.can;
        unsafe {
            can.timer().timer() as u16
        }
    }

    // IFLAG

    pub fn bufi(&self, index: usize) -> bool {
        let can = self.can;
        unsafe {
            can.iflag1().bufi(index) != 0
        }
    }

    pub fn clr_bufi(&self, index: usize) {
        let mut can = self.can;
        unsafe {
            can.set_iflag1(Iflag1(0).set_bufi(index, 1));
        }
    }

    // RXIMR

    pub fn mi(&self, index: usize) -> u32 {
        let can = self.can;
        unsafe {
            can.rximr(index).mi() as u32
        }
    }
    
    pub fn set_mi(&self, index: usize, value: u32) {
        let mut can = self.can;
        unsafe {
            can.set_rximr(index, Rximr(0).set_mi(value));
        }
    }
        
    pub fn clear_ram(&self) {
        let mut can = self.can;
        unsafe {
            for i in 0..128 {
                can.set_ram(i, Ram(0))
            }
        }
    }

    pub fn enter_freeze_mode(&self) {
        self.set_frz(true);
        self.set_halt(true);
        while !self.frzack() {}
    }

    pub fn exit_freeze_mode(&self) {
        self.set_frz(false);
        self.set_halt(false);
        while self.frzack() {}
    }    

    pub fn mbuf(&self, index: usize) -> MBuf {
        MBuf { can: self.can, index: index }
    }
}

pub struct MBuf {
    can: Can,
    index: usize,
}

impl MBuf {    
    pub fn mb8h0(&self) -> Mb8h0 {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index)
        }
    }

    pub fn mb8h1(&self) -> Mb8h1 {
        let can = self.can;
        unsafe {
            can.mb8h1(self.index)
        }
    }

    // FLAG
    pub fn flag(&self) -> bool {
        let can = self.can;
        unsafe {
            can.iflag1().bufi(self.index) != 0
        }
    }

    pub fn clr_flag(&self) {
        let mut can = self.can;
        unsafe {
            can.set_iflag1(Iflag1(0).set_bufi(self.index, 1));
        }
    }

    // IDMASK
    pub fn idmask(&self) -> u32 {
        let can = self.can;
        unsafe {
            can.rximr(self.index).mi() as u32
        }
    }
    
    pub fn set_idmask(&self, value: u32) {
        let mut can = self.can;
        unsafe {
            can.set_rximr(self.index, Rximr(0).set_mi(value));
        }
    }    
    
    pub fn edl(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).edl() != 0
        }
    }

    pub fn set_edl(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_edl(value))
        }
    }

    // BRS

    pub fn brs(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).brs() != 0
        }
    }

    pub fn set_brs(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_brs(value))
        }
    }    

    // ESI

    pub fn esi(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).esi() != 0
        }
    }

    pub fn set_esi(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_esi(value))
        }
    }

    // CODE

    pub fn code(&self) -> Code {
        let can = self.can;
        unsafe {
            match can.mb8h0(self.index).code() {
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
    }

    pub fn set_code(&self, value: Code) {        
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_code(value as u32))
        }
    }

    // SRR

    pub fn srr(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).srr() != 0
        }
    }

    pub fn set_srr(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_srr(value))
        }
    }

    // IDE

    pub fn ide(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).ide() != 0
        }
    }

    pub fn set_ide(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_ide(value))
        }
    }        

    // RTR

    pub fn rtr(&self) -> bool {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).rtr() != 0
        }
    }

    pub fn set_rtr(&self, value: bool) {        
        let value = if value { 1 } else { 0 };
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_rtr(value))
        }
    }

    // DLC

    pub fn dlc(&self) -> u8 {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).dlc() as u8
        }
    }

    pub fn set_dlc(&self, value: u8) {        
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_dlc(value as u32))
        }
    }

    // TIME_STAMP

    pub fn time_stamp(&self) -> u16 {
        let can = self.can;
        unsafe {
            can.mb8h0(self.index).time_stamp() as u16
        }
    }

    pub fn set_time_stamp(&self, value: u16) {        
        let mut can = self.can;
        unsafe {
            can.with_mb8h0(self.index, |r| r.set_time_stamp(value as u32))
        }
    }    

    // PRIO

    pub fn prio(&self) -> u8 {
        let can = self.can;
        unsafe {
            can.mb8h1(self.index).prio() as u8
        }
    }

    pub fn set_prio(&self, value: u8) {
        let mut can = self.can;
        unsafe {
            can.with_mb8h1(self.index, |r| r.set_prio(value as u32))
        }
    }        

    // ID

    pub fn id(&self) -> CanId {
        let can = self.can;
        unsafe {
            let h0 = can.mb8h0(self.index);
            let h1 = can.mb8h1(self.index);
            match h0.ide() {
                0 => CanId::Std(StandardId(h1.id_std() as u16)),
                1 => CanId::Ext(ExtendedId(h1.id_ext())),
                _ => panic!("Invalid IDE value")
            }
        }
    }

    pub fn set_id(&self, id: CanId) {
        let mut can = self.can;
        unsafe {
            can.with_mb8h1(self.index, |r| match id {
                CanId::Std(std_id) => r.set_id_std(std_id.0 as u32),
                CanId::Ext(ext_id) => r.set_id_ext(ext_id.0),
            });
            let ide = if let CanId::Ext(_) = id { 1 } else { 0 };
            can.with_mb8h0(self.index, |r| r.set_ide(ide));
        }
    }

    pub fn id_std(&self) -> u32 {
        let can = self.can;
        unsafe {
            can.mb8h1(self.index).id_std() as u32
        }
    }

    pub fn set_id_std(&self, value: u32) {
        let mut can = self.can;
        unsafe {
            can.with_mb8h1(self.index, |r| r.set_id_std(value as u32))
        }
    }        

    pub fn id_ext(&self) -> u32 {
        let can = self.can;
        unsafe {
            can.mb8h1(self.index).id_ext() as u32
        }
    }

    pub fn set_id_ext(&self, value: u32) {
        let mut can = self.can;
        unsafe {
            can.with_mb8h1(self.index, |r| r.set_id_ext(value as u32))
        }
    }            

    pub fn read(&self, buf: &mut [u8]) -> (CanId, usize) {
        let can = self.can;        
        unsafe {
            let h0 = can.mb8h0(self.index);
            let h1 = can.mb8h1(self.index);
            let id = match h0.ide() {
                0 => CanId::Std(StandardId(h1.id_std() as u16)),
                1 => CanId::Ext(ExtendedId(h1.id_ext())),
                _ => panic!("Invalid IDE value"),
            };

            let dlc = h0.dlc() as usize;
            if dlc > 0 {
                let d0 = can.mb8d0(self.index).0;
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
                let d1 = can.mb8d1(self.index).0;
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
    }

    pub fn write(&self, id: CanId, buf: &[u8]) {
        assert!(buf.len() <= 8, "Only payloads up to 8 bytes are supported");
        let mut d = [0u8; 8];
        for i in 0..buf.len() {
            d[i] = buf[i];
        }
        let mut can = self.can;        
        unsafe {
            can.set_mb8d0(self.index, Mb8d0(
                (d[0] as u32) << 24 |
                (d[1] as u32) << 16 |
                (d[2] as u32) << 8 |
                (d[3] as u32)
            ));
            can.set_mb8d1(self.index, Mb8d1(
                (d[4] as u32) << 24 |
                (d[5] as u32) << 16 |
                (d[6] as u32) << 8 |
                (d[7] as u32)
            ));
            can.with_mb8h1(self.index, |r| match id {
                CanId::Std(std_id) => r.set_id_std(std_id.0 as u32),
                CanId::Ext(ext_id) => r.set_id_ext(ext_id.0),
            });
            let ide = if let CanId::Ext(_) = id { 1 } else { 0 };
            can.with_mb8h0(self.index, |r| r.set_ide(ide).set_code(Code::TxData as u32).set_dlc(buf.len() as u32));
        }
    }

}