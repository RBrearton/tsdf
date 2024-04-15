use serde::{Deserialize, Serialize};

use super::{ReadMode, WriteMode};

/// Enum for the different IO modes.
#[derive(Debug, Serialize, Deserialize)]
pub enum IoMode {
    /// Read the file. This may or may not lock the file, depending on which ReadMode is being used.
    Read(ReadMode),

    /// Write to the file. This may be a lockless writer, or a locking writer, depending on which
    /// WriteMode is being used.
    Write(WriteMode),
}
