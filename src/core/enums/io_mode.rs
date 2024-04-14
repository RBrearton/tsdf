use serde::{Deserialize, Serialize};

/// Enum for the different IO modes.
#[derive(Debug, Serialize, Deserialize)]
pub enum IoMode {
    /// Read the file. This never places a lock on the file.
    Read,

    /// Write to the file without locking it. Using this mode removes some functionality, such as
    /// data deletion. Otherwise, data can be written to the file as normal.
    LocklessWrite,

    /// Write to the file with a lock. This mode allows for all functionality, including data
    /// deletion, at the cost of not allowing readers to concurrently access the file.
    LockingWrite,
}

// Now add a function to the IoMode enum that returns true if the IoMode is a write mode.
impl IoMode {
    pub fn is_a_writing_mode(&self) -> bool {
        match self {
            IoMode::Read => false,
            IoMode::LocklessWrite => true,
            IoMode::LockingWrite => true,
        }
    }
}
