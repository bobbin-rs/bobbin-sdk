use ftfe::*;
use bobbin_hal::flash::*;

pub trait FlashBusy {
    fn flash_busy(&self) -> bool;
}

impl FlashBusy for FtfePeriph {
    fn flash_busy(&self) -> bool {
        !self.fstat().test_ccif()
    }
}

pub trait FtfeWriteCommand {
    fn write_command(&self, buf: &[u8]);
}

impl FtfeWriteCommand for FtfePeriph {
    fn write_command(&self, buf: &[u8]) {
        self.set_fstat(|_| Fstat(0x70));
        while self.flash_busy() {}
        for i in 0..buf.len() {
            self.set_fccob(i, |_| Fccob(buf[i]));
        }
        self.set_fstat(|_| Fstat(0x80));
    }
}

impl FlashErase for FtfePeriph {
    fn erase_start(&self, addr: *mut u8) -> Result<(), FlashError> {
        let addr = addr as u32;
        self.write_command(&[
            (addr >> 0) as u8,
            (addr >> 8) as u8,
            (addr >> 16) as u8,
            0x08,
        ]);
        Ok(())
    }

    fn erase_complete(&self) -> bool {
        !self.flash_busy()
    }
}

/// Write bytes to flash memory.
impl FlashWrite for FtfePeriph {
    fn write_start(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError> {
        let len = src.len();
        if len % 8 != 0 {
            return Err(FlashError::InvalidWriteSize)
        }        
        let addr = dst as u32;
        let mut i = 0;
        while i < len {
            let daddr = addr + (i as u32);
            self.write_command(&[
                (daddr >> 0) as u8,
                (daddr >> 8) as u8,
                (daddr >> 16) as u8,
                0x07,
                src[i + 0],
                src[i + 1],
                src[i + 2],
                src[i + 3],
                src[i + 4],
                src[i + 5],
                src[i + 6],
                src[i + 7],
            ]);
            i += 8;
        }
        Ok(())
    }

    fn write_complete(&self) -> bool {
        !self.flash_busy()
    }
}
