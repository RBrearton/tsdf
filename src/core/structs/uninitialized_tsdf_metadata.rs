use crate::core::enums::{FileFormat, IoMode};

/// An uninitialized version of the TsdfMetadata struct. This struct contains only the information
/// that a user would expect to need to know about a file before it is opened.
pub(super) struct UninitializedTsdfMetadata<'a> {
    /// The current version of the tsdf crate. This may or may not end up matching the version that
    /// could be found in a file.
    current_version: &'a str,

    /// The file format of the file. This may or may not be provided, so it is an Option.
    file_format: Option<FileFormat>,

    /// The IoMode used to open the file.
    io_mode: IoMode,
}
