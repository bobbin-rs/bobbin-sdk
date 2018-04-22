//! Traits for interacting with CANBUS devices.
//! 
//! TBD: Add Address, Message and CanRead / CanWrite traits

pub enum CanError {    
}

/// A CAN identifier.
pub enum CanId {
    /// A standard 11-bit identifier.
    Std(u16),
    /// An extended 29-bit identifier.
    Ext(u32),
}

/// A CAN message.
pub struct CanMessage {
    pub id: CanId,
    pub len: usize,
    pub data: [u8; 8],
}

/// Operations on a device that can transmit CAN messages.
pub trait CanTx {
    fn can_tx(&self) -> bool;
    fn tx(msg: CanMessage) -> Result<(), CanError>;
}

/// Operations on a device that can receive CAN messages.
pub trait CanRx {
    fn can_rx(&self) -> bool;
    fn rx() -> Result<CanMessage, CanError>;
}