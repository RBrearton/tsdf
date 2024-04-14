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

impl<'a> UninitializedTsdfMetadata<'a> {
    /// Constructs a new UninitializedTsdfMetadata.
    pub fn new(current_version: &'a str, file_format: Option<FileFormat>, io_mode: IoMode) -> Self {
        Self {
            current_version,
            file_format,
            io_mode,
        }
    }

    /// Returns the current version of the tsdf crate.
    pub fn get_current_version(&self) -> &str {
        self.current_version
    }

    /// Returns the file format of the file.
    pub fn get_file_format(&self) -> &Option<FileFormat> {
        &self.file_format
    }

    /// Returns the IoMode used to open the file.
    pub fn get_io_mode(&self) -> &IoMode {
        &self.io_mode
    }
}
