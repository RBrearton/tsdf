use crate::core::enums::{FileFormat, IoMode};

/// The core metadata for a tsdf file. This is written at the very beginning of every tsdf file as
/// a json blob.
pub(super) struct TsdfMetadata<'a> {
    /// The semantic version of tsdf used to write the file.
    version: &'a str,

    /// The mode used to write the file.
    file_format: FileFormat,

    /// The IoMode used to open the file.
    io_mode: IoMode,
}

impl<'a> TsdfMetadata<'a> {
    /// Constructs a new TsdfMetadata.
    pub fn new(version: &'a str, file_format: FileFormat, io_mode: IoMode) -> Self {
        Self {
            version,
            file_format,
            io_mode,
        }
    }

    /// Returns the version of the file.
    pub fn get_version(&self) -> &str {
        self.version
    }

    /// Returns the file format of the file.
    pub fn get_file_format(&self) -> &FileFormat {
        &self.file_format
    }

    /// Returns the io mode of the file.
    pub fn get_io_mode(&self) -> &IoMode {
        &self.io_mode
    }
}
