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
        self.set_keyr(|r| r.set_key(KEY1));
        self.set_keyr(|r| r.set_key(KEY2));
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
        let snb = match addr as u32 {
            0x0800_0000 => 0,
            0x0800_8000 => 1,
            0x0801_0000 => 2,
            0x0801_8000 => 3,
            0x0802_0000 => 4,
            0x0804_0000 => 5,
            0x0808_0000 => 6,
            0x080c_0000 => 7,
            _ => return Err(FlashError::InvalidEraseAddr),
        };        
        self.with_cr(|r| r.set_snb(snb).set_ser(1).set_strt(1));
        Ok(())
    }

    fn erase_complete(&self) -> bool {
        return !self.flash_busy()
    }

    fn erase_end(&self){
        self.with_cr(|r| r.set_ser(0));
        self.flash_lock();
    }
}

/// Write bytes to flash memory.
impl FlashWrite for FlashPeriph {
    fn write_begin(&self) {
        self.flash_unlock();
    }

    fn write_start(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError> {
        self.with_cr(|r| r.set_pg(1).set_psize(0b00));
        for i in 0..src.len() {
            unsafe {
                *dst.offset(i as isize) = src[i];
                #[cfg(target_os="none")]
                asm!("dsb");
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