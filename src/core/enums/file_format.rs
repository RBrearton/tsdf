/// Enum to specify the write mode of a file. Please note that, for all production use cases, the
/// write mode should be set to `Binary`. The `Text` mode is only for debugging and development.
pub enum FileFormat {
    /// Use the default write mode if creating a file. If the file already exists, the write mode
    /// will be inferred from the file.
    Default,

    /// Write the file in binary mode. This is the recommended mode for all production use cases.
    Binary,

    /// Write the file in plain text mode. This is only for debugging and development.
    Text,
}
