use std::{
    fs::File,
    io::{self, Read, Seek},
};

use serde::{Deserialize, Serialize};

use crate::core::enums::FileFormat;
use crate::core::well_known_values::metadata_strings::HEADER_END_STR;

/// The core metadata for a tsdf file. This is written at the very beginning of every tsdf file as
/// a json blob.
#[derive(Serialize, Deserialize)]
pub(super) struct TsdfMetadata {
    /// The semantic version of tsdf used to write the file.
    version: String,

    /// The mode used to write the file.
    file_format: FileFormat,
}

impl TsdfMetadata {
    /// Constructs a new TsdfMetadata.
    pub(crate) fn new(version: String, file_format: FileFormat) -> Self {
        Self {
            version,
            file_format,
        }
    }

    /// Deserializes a TsdfMetadata from the top of a tsdf file.
    pub(crate) fn read_from_tsdf(file: &mut File) -> Result<Self, io::Error> {
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
        &self.version
    }

    /// Returns the file format of the file.
    pub fn get_file_format(&self) -> &FileFormat {
        &self.file_format
    }
}
