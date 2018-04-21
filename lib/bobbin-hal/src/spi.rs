//! Traits for using the SPI protocol.
//! 
//! TODO: Add Error and Results to traits

/// Query whether the SPI device is currently busy.
pub trait SpiBusy {
    /// Returns true if the SPI device is busy.
    fn busy(&self) -> bool;
    /// Busy wait until `busy()` is true.
    fn wait_not_busy(&self) -> &Self {
        while self.busy() {}
        self
    }
}

/// Enable or disable SPI output
pub trait SpiOutputEnabled {
    /// Returns true if SPI output is enabled.
    fn output_enabled(&self) -> bool;
    /// Set the SPI output state to `value`.
    fn set_output_enabled(&self, value: bool) -> &Self;
}

/// Query the SPI transmit state
pub trait SpiCanTx {
    /// Returns true if the device can accept additional words to transmit.
    fn can_tx(&self) -> bool;
    /// Busy wait until `can_tx()` is true.
    fn wait_can_tx(&self) -> &Self {
        while !self.can_tx() {}
        self
    }
}

/// Query the SPI receive state
pub trait SpiCanRx {
    /// Returns true if the device has words available to receive.
    fn can_rx(&self) -> bool;
    /// Busy wait until `can_rx()` is true.
    fn wait_can_rx(&self) -> &Self {
        while !self.can_rx() {}
        self
    }    
}

/// Transmit a word using the SPI device.
pub trait SpiTx<T: Copy> {
    /// Writes a word `c` to the SPI device for transmission. Behavior is undefined
    /// if this is called while `can_tx()` is false.
    fn tx(&self, c: T) -> &Self;
}

/// Receive a word from the SPI device.
pub trait SpiRx<T: Copy> {
    /// Reads and returns a word from the SPI device. Behavior is undefined if this is
    /// called while `can_rx()` is false.
    fn rx(&self) -> T;
}

/// Write bytes to the SPI device.
pub trait SpiWrite {
    /// Writes the buffer `tx` to the SPI device, blocking until complete.
    fn write(&self, tx: &[u8]);
}

/// Read bytes from the SPI device.
pub trait SpiRead {
    /// Reads from the SPI device into the buffer `rx``, blocking until complete.
    fn read(&self, rx: &mut [u8]);
}

/// Simultaneous Read and Write from the SPI device
pub trait SpiTransfer : SpiRead + SpiWrite {
    /// Simultaneously writes from `tx` and reads into `tx`, blocking until complete.
    /// Behavior is undefined if `tx` and `rx` are not the same length.
    fn transfer(&self, tx: &[u8], rx: &mut [u8]) {
        self.write(tx);
        self.read(rx);
    }
}