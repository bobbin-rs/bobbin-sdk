use ::chip::dma::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Channel(pub Dma, pub usize);

pub const DMA1_1: Channel = Channel(DMA1, 0);
pub const DMA1_2: Channel = Channel(DMA1, 1);
pub const DMA1_3: Channel = Channel(DMA1, 2);
pub const DMA1_4: Channel = Channel(DMA1, 3);
pub const DMA1_5: Channel = Channel(DMA1, 4);
pub const DMA1_6: Channel = Channel(DMA1, 5);
pub const DMA1_7: Channel = Channel(DMA1, 6);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Bit8 = 0b00,
    Bit16 = 0b01,
    Bit32 = 0b10,
    Reserved = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    PtoM = 0b0,
    MtoP = 0b1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub mem2mem: bool,
    pub priority: Priority,
    pub msize: Size,
    pub psize: Size,
    pub minc: bool,
    pub pinc: bool,
    pub circ: bool,
    pub dir: Dir,
    pub teie: bool,
    pub htie: bool,
    pub tcie: bool,
    pub en: bool,
}

#[inline="always"]
fn b2u32(v: bool) -> u32 {
    if v { 1 } else { 0 }
}

pub fn set_config(channel: Channel, cfg: &Config) {
    let dma = channel.0;
    unsafe {
        dma.with_ccr(channel.1, |r| r
            .set_mem2mem(b2u32(cfg.mem2mem))
            .set_pl(cfg.priority as u32)
            .set_msize(cfg.msize as u32)
            .set_psize(cfg.psize as u32)
            .set_minc(b2u32(cfg.minc))
            .set_pinc(b2u32(cfg.pinc))
            .set_circ(b2u32(cfg.circ))
            .set_dir(cfg.dir as u32)
            .set_teie(b2u32(cfg.teie))
            .set_htie(b2u32(cfg.htie))
            .set_tcie(b2u32(cfg.tcie))
            .set_en(b2u32(cfg.en))
        )
    }
}

pub fn set_enabled(channel: Channel, value: bool) {
    let value = if value { 1 } else { 0 };
    let dma = channel.0;
    unsafe {
        dma.with_ccr(channel.1, |r| r.set_en(value))
    }
}

pub fn periph_addr(channel: Channel) -> u32 {
    let dma = channel.0;
    unsafe {
        dma.cpar(channel.1).pa()
    }
}

pub fn set_periph_addr(channel: Channel, addr: u32) {
    let dma = channel.0;
    unsafe {
        dma.set_cpar(channel.1, Cpar(0).set_pa(addr))
    }
}

pub fn memory_addr(channel: Channel) -> u32 {
    let dma = channel.0;
    unsafe {
        dma.cmar(channel.1).ma()
    }
}

pub fn set_memory_addr(channel: Channel, addr: u32) {
    let dma = channel.0;
    unsafe {
        dma.set_cmar(channel.1, Cmar(0).set_ma(addr))
    }
}

pub fn transfer_count(channel: Channel) -> usize {
    let dma = channel.0;
    unsafe {
        dma.cndtr(channel.1).ndt() as usize
    }
}

pub fn set_transfer_count(channel: Channel, count: usize) {
    let dma = channel.0;
    unsafe {
        dma.set_cndtr(channel.1, Cndtr(0).set_ndt(count as u32))
    }
}

pub fn set_channel_selection(channel: Channel, sel: u8) {
    let dma = channel.0;
    unsafe {
        dma.with_cselr(|r| r.set_cs(channel.1, sel as u32))
    }
}

pub fn clear_flags(channel: Channel) {
    let dma = channel.0;
    unsafe {
        dma.set_ifcr(Ifcr(0)
            .set_cgif(channel.1, 1)
            .set_ctcif(channel.1, 1)
            .set_chtif(channel.1, 1)
            .set_cteif(channel.1, 1)
        )
    }
}

pub fn gif(channel: Channel) -> bool {
    let dma = channel.0;
    unsafe {
        dma.isr().gif(channel.1) != 0
    }
}

pub fn clear_gif(channel: Channel) {
    let dma = channel.0;
    unsafe {
        dma.set_ifcr(Ifcr(0).set_cgif(channel.1, 1))
    }
}

pub fn tcif(channel: Channel) -> bool {
    let dma = channel.0;
    unsafe {
        dma.isr().tcif(channel.1) != 0
    }
}

pub fn clear_tcif(channel: Channel) {
    let dma = channel.0;
    unsafe {
        dma.set_ifcr(Ifcr(0).set_ctcif(channel.1, 1))
    }
}

pub fn htif(channel: Channel) -> bool {
    let dma = channel.0;
    unsafe {
        dma.isr().htif(channel.1) != 0
    }
}

pub fn clear_htif(channel: Channel) {
    let dma = channel.0;
    unsafe {
        dma.set_ifcr(Ifcr(0).set_chtif(channel.1, 1))
    }
}

pub fn teif(channel: Channel) -> bool {
    let dma = channel.0;
    unsafe {
        dma.isr().teif(channel.1) != 0
    }
}

pub fn clear_teif(channel: Channel) {
    let dma = channel.0;
    unsafe {
        dma.set_ifcr(Ifcr(0).set_cteif(channel.1, 1))
    }
}