use std::{
    fs::File,
    io::{self, Read, Seek},
};

use serde::{Deserialize, Serialize};

use crate::core::enums::{FileFormat, IoMode};
use crate::core::well_known_values::metadata_strings::HEADER_END_STR;

/// The core metadata for a tsdf file. This is written at the very beginning of every tsdf file as
/// a json blob.
#[derive(Serialize, Deserialize)]
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

    /// Deserializes a TsdfMetadata from the top of a tsdf file.
    fn read_from_tsdf(mut file: File) -> Result<Self, io::Error> {
        // Seek to the beginning of the file.
        file.seek(std::io::SeekFrom::Start(0))?;

        // Read the file up until the header end string.
        let mut metadata_json = Vec::new();
        let mut reader = std::io::BufReader::new(file);
        loop {
            let mut buffer = [0; 1];
            reader.read_exact(&mut buffer)?;
            metadata_json.push(buffer[0]);
            if metadata_json.ends_with(HEADER_END_STR.as_bytes()) {
                break;
            }
        }

        // Remove the header end string from the metadata json.
        metadata_json.truncate(metadata_json.len() - HEADER_END_STR.len());

        // Deserialize the metadata json.
        let metadata: TsdfMetadata = serde_json::from_slice(&metadata_json)?;

        Ok(metadata)
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
