use flash::FlashPeriph;
use bobbin_hal::flash::*;

pub const KEY1: u32 = 0x45670123;
pub const KEY2: u32 = 0xCDEF89AB;

pub trait FlashLockUnlock {
    fn flash_locked(&self) -> bool;
    fn flash_unlock(&self);
    fn flash_lock(&self);
    fn unlocked<T, F: FnOnce(&Self)->T>(&self, f: F) -> T {
        self.flash_unlock();
        let ret = f(self);
        self.flash_lock();
        ret
    }
}

pub trait FlashBusy {
    fn flash_busy(&self) -> bool;
}

impl FlashLockUnlock for FlashPeriph {
    fn flash_locked(&self) -> bool {
        self.cr().test_lock()
    }

    fn flash_lock(&self) {
        self.with_cr(|r| r.set_lock(1));
    }

    fn flash_unlock(&self) {
        self.set_keyr(|r| r.set_fkeyr(KEY1));
        self.set_keyr(|r| r.set_fkeyr(KEY2));
    }
}

impl FlashBusy for FlashPeriph {
    fn flash_busy(&self) -> bool {
        self.sr().test_bsy()
    }
}

impl FlashErase for FlashPeriph {
    fn erase_begin(&self) {
        self.flash_unlock();
    }

    fn erase_start(&self, addr: *mut u8) -> Result<(), FlashError> {
        // ignore length for now
        self.with_sr(|r| r.set_eop(1));
        self.with_cr(|r| r.set_per(1));
        self.set_ar(|r| r.set_far(addr as u32));
        self.with_cr(|r| r.set_strt(1));
        unsafe { asm!("nop")}
        Ok(())
    }

    fn erase_complete(&self) -> bool {
        return !self.flash_busy()
    }

    fn erase_end(&self){
        self.with_sr(|r| r.set_eop(1));
        self.with_cr(|r| r.set_per(0));
        self.flash_lock();
    }
}


/// Write bytes to flash memory.
impl FlashWrite for FlashPeriph {
    fn write_begin(&self) {
        self.flash_unlock();
        self.with_cr(|r| r.set_pg(1));
    }

    fn write_start(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError> {
        let len = src.len();
        if len % 2 != 0 {
            return Err(FlashError::InvalidWriteSize)
        }
        let mut i = 0;
        while i < len {
            unsafe {
                let s = src.as_ptr().offset(i as isize) as *const u16;
                let d = dst.offset(i as isize) as *mut u16;
                *d = *s;
                i += 2;
            }
            while self.flash_busy() {}
        }
        Ok(())
    }
    fn write_complete(&self) -> bool {
        !self.flash_busy()
    }
    fn write_end(&self) {
        self.with_cr(|r| r.set_pg(0));        
        self.flash_lock();
    }
}

impl Flash for FlashPeriph {}