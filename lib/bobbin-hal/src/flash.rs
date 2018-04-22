//! Traits for reading and writing flash memory.
//! 
//! Devices supporting this trait have the following characteristics:
//! 
//! - The device must be placed into erase or write mode using `erase_begin()` or `write_begin()` 
//! before any erase or write operations are started, and `erase_end()` or `write_end()` must be
//! called after all erase or write operations are completed.
//! - Memory must be erased before writing.
//! - Memory must be erased a block at a time. The block size is device and possibly address
//! dependent.
//! - Memory may be written a phrase at a time. The phrase size is device dependent.
//! - Erases and writes are asynchronous, and completion status is indicated with `erase_complete()` or
//! `write_complete()`.


/// The result of a flash operation
pub enum FlashError {
    /// The erase address was not valid.
    InvalidEraseAddr,
    /// The erase size was not valid.
    InvalidEraseSize,
    /// The erase operation failed.
    EraseFailed,
    /// The erase operation timed out.
    EraseTimeout,
    /// The write address was not valid.
    InvalidWriteAddr,
    /// The write size was not valid.
    InvalidWriteSize,
    /// The write operation failed.
    WriteFailed,
    /// Write Timeout
    WriteTimeout,
}

/// Query the Flash device for erase and write parameters.
pub trait FlashInfo {    
    /// Returns the start and length of the erase region containing `addr` or 
    /// None if the address cannot be erased.
    fn info_erase(&self, addr: *mut u8) -> Option<(*mut u8, usize)>;
    /// Returns the minimum size of a write in bytes.
    fn info_write_size(&self) -> usize;
}

/// Erase a section of flash memory.
pub trait FlashErase {
    /// Put the device into erase mode. `erase_end()` must be called when erasing is done.
    fn erase_begin(&self) {}
    /// Starts the erase process for the page beginning at `addr`. The device must have
    /// previously be put in erase mode using `erase_begin()`.
    fn erase_start(&self, addr: *mut u8) -> Result<(), FlashError>;
    /// Returns true if an erase is in progress.
    fn erase_complete(&self) -> bool;
    /// Busy wait for the erase to complete.
    fn erase_wait(&self) -> Result<(), FlashError> {
        while !self.erase_complete() {}
        Ok(())
    }
    /// Erase flash and wait for the erase to complete, then end the erase.
    fn erase(&self, addr: *mut u8) -> Result<(), FlashError> {
        self.erase_start(addr)?;
        self.erase_wait()
    }
    /// Exit erase mode. Behavior is undefined if the device is not currently in erase mode or if
    /// an erase is pending.
    fn erase_end(&self) {}
}

/// Write bytes to flash memory.
pub trait FlashWrite {
    /// Put the device into write mode. `write_end()` must be called when writing is done.
    fn write_begin(&self) {}
    /// Start writing `len` bytes from `buf` to `addr`.
    fn write_start(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError>;
    /// Returns true if the current write is complete;
    fn write_complete(&self) -> bool;
    /// Busy wait for the write to complete;
    fn write_wait(&self) -> Result<(), FlashError> {
        while !self.write_complete() {}
        Ok(())
    }
    /// Write `len` bytes from `addr` to flash memory and wait for the write to complete.
    fn write(&self, dst: *mut u8, src: &[u8]) -> Result<(), FlashError> {
        self.write_start(dst, src)?;
        self.write_wait()
    }
    /// Exit write mode. Behavior is undefined if the device is not currently in write mode or
    /// if a write is pending.
    fn write_end(&self) {}
}

pub trait Flash : FlashErase + FlashWrite {}

pub trait GetFlash {
    type Output: Flash;
    fn flash(&self) -> &Self::Output;
}