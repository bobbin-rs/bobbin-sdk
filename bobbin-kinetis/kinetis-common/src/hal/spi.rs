pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::spi::*;
pub use ::chip::spi::*;

impl SpiPeriph {
    pub fn init(&self, br: u8, pbr: u8) {
        // Halt and clear FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        self.with_mcr(|r| r.set_pcsis(0b1));
        self.with_mcr(|r| r.set_mdis(0).set_mstr(1));

        // Set configuration 0
        self.set_ctar(0, |_| Ctar(0)
            // .set_pcssck(0b01)
            // .set_pasc(0b01)
            .set_cssck(0b0100)
            .set_asc(0b0100)            
            .set_br(br)
            .set_pbr(pbr)
            .set_fmsz(7)
        );              
    }

    pub fn write(&self, bytes: &[u8]) {
        // Flush FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        
        // Clear status bits
        self.set_sr(|_| Sr(0)
            .set_tcf(1)
            .set_eoqf(1)
            .set_tfuf(1)
            .set_tfff(1)
            .set_rfof(1)
            .set_rfdf(1));
        self.with_mcr(|r| r.set_halt(0)); 

        let len = bytes.len();
        for i in 0..len {
            let last = if i == len - 1 { 1 } else { 0 };

            // NOTE
            // TFFF flag clears automatically when DMA is used to fill TX FIFO.
            // To clear TFFF when not using DMA, follow these steps for every PUSH performed using CPU to fill TX FIFO:
            // 1. Wait until TFFF = 1.
            // 2. Write data to PUSHR using CPU.
            // 3. Clear TFFF by writing a 1 to its location. If TX FIFO is not
            // full, this flag will not clear.

            while self.sr().tfff() == 0 {}                
            self.set_pushr(|_| Pushr(0).set_cont(1).set_ctas(0).set_eoq(last).set_txdata(bytes[i]).set_pcs(1));
            self.set_sr(|_| Sr(0).set_tfff(1));
        }            
        while self.sr().eoqf() == 0 {}
        self.set_sr(|_| Sr(0).set_eoqf(1));
        self.with_mcr(|r| r.set_halt(1));
    }

    pub fn read(&self, bytes: &mut [u8]) {
        // Flush FIFOs
        self.with_mcr(|r| r.set_halt(1).set_clr_txf(1).set_clr_rxf(1));
        // Clear status bits
        self.set_sr(|_| Sr(0).set_tcf(1).set_eoqf(1).set_tfuf(1).set_tfff(1).set_rfof(1).set_rfdf(1));
        self.with_mcr(|r| r.set_halt(0)); 
        let len = bytes.len();
        for i in 0..len {
            self.set_pushr(|_| Pushr(0).set_cont(1).set_ctas(0).set_txdata(0xff).set_pcs(1));
            while self.sr().rfdf() == 0 {}
            bytes[i] = self.popr().rxdata().value() as u8;
            self.set_sr(|_| Sr(0).set_rfdf(1));
        }                        
        self.with_mcr(|r| r.set_halt(1)); 
    }    
}