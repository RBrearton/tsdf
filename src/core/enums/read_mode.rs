use serde::{Deserialize, Serialize};

/// Enum for the different IO modes.
#[derive(Debug, Serialize, Deserialize)]
pub enum ReadMode {
    /// Read the file. This never places a lock on the file.
    LocklessRead,
}
