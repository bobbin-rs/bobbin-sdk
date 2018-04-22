use nvmctrl::*;
use bobbin_hal::flash::*;

pub trait FlashBusy {
    fn flash_busy(&self) -> bool;
}

impl FlashBusy for NvmctrlPeriph {
    fn flash_busy(&self) -> bool {
        self.intflag().test_ready()
    }
}

impl FlashErase for NvmctrlPeriph {
    fn erase_start(&self, addr: *mut u8) -> Result<(), FlashError> {
        // Note from https://github.com/blacksphere/blackmagic/blob/master/src/target/samd.c
        /* Write address of first word in row to erase it */
		/* Must be shifted right for 16-bit address, see Datasheet §20.8.8 Address */
        self.set_addr(|_| Addr(addr as u32 >> 1));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x41));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x02));
        Ok(())
    }

    fn erase_complete(&self) -> bool {
        !self.flash_busy()
    }
}

/// Write bytes to flash memory.
impl FlashWrite for NvmctrlPeriph {
    fn write_start(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError> {
        // Writes must be 16 bit or 32 bit
        let len = src.len();
        if len % 2 != 0 {
            return Err(FlashError::InvalidWriteSize)
        }   
        let mut i = 0;
        while i < len {     
            unsafe {
                let d = dst.offset(i as isize) as *mut u16;
                let s = src.as_ptr().offset(i as isize) as *const u16;
                *d = *s;
            }
            i += 2;  
        }
        self.set_addr(|r| r.set_addr(dst as u32));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x04));
        Ok(())
    }

    fn write_complete(&self) -> bool {
        !self.flash_busy()
    }
}
