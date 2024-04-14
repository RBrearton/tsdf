use serde::{Deserialize, Serialize};

/// Enum to specify the file format of a file. Please note that, for all production use cases, the
/// file format should be set to `Binary`. The `Text` mode is only for debugging and development.
#[derive(Debug, Serialize, Deserialize)]
pub enum FileFormat {
    /// Write the file in binary mode. This is the recommended mode for all production use cases.
    Binary,

    /// Write the file in plain text mode. This is only for debugging and development.
    Text,
}
