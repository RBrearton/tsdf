use std::io;
use std::os::unix::fs::FileExt;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use crate::core::enums::{FileFormat, IoMode, ReadMode, WriteMode};
use crate::core::traits::TsdfFileTrait;

use super::{Dir, IoMetadata, TsdfMetadata};

/// The central TsdfFile struct. This struct is used to interact with tsdf
/// files.
pub struct TsdfFile<'a> {
    /// The actual operating system path to the file.
    path: &'a Path,

    /// All metadata used in I/O operations.
    io_metadata: IoMetadata,

    /// The open file handle.
    file: File,
}

// Implement private methods for TsdfFile.
impl TsdfFile<'_> {}

// Implement the TsdfFileTrait for TsdfFile.
impl TsdfFileTrait for TsdfFile<'_> {
    fn get_version(&self) -> &str {
        self.io_metadata.get_tsdf_metadata().get_version()
    }

    fn get_path(&self) -> &Path {
        self.path
    }

    fn get_io_mode(&self) -> &IoMode {
        &self.io_metadata.get_io_mode()
    }

    fn get_file_format(&self) -> &FileFormat {
        &self.get_io_metadata().get_tsdf_metadata().get_file_format()
    }

    fn get_io_metadata(&self) -> &IoMetadata {
        &self.io_metadata
    }

    fn get_tsdf_metadata(&self) -> &TsdfMetadata {
        self.io_metadata.get_tsdf_metadata()
    }

    fn get_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    fn get_root_dir(&self) -> &Dir {
        unimplemented!()
    }

    fn new_reader(path: &'static Path) -> io::Result<Box<Self>> {
        // Open the file. If the file doesn't exist, we're perfectly happy to
        // panic - we can't read from a file that doesn't exist.
        let mut file = File::open(path)?;

        // Deserialize the metadata from the header of the file.
        let metadata = TsdfMetadata::read_from_tsdf(&mut file)?;
        let io_mode = IoMode::Read(ReadMode::LocklessRead);
        let io_metadata = IoMetadata::new(metadata, io_mode);

        // Return the TsdfFile.
        Ok(Box::new(TsdfFile {
            path,
            file,
            io_metadata,
        }))
    }

    fn new_writer(
        path: &'static Path,
        write_mode: Option<WriteMode>,
        file_format: Option<FileFormat>,
    ) -> io::Result<Box<Self>> {
        // First of all, if the file doesn't exist, we can just use
        // new_overwriting_writer, as the behaviour will be identical.
        if !path.exists() {
            return Self::new_overwriting_writer(path, write_mode, file_format);
        }

        // If execution reaches here, we know that the file already exists.
        // Deserialize the metadata from the file.
        let mut file = File::open(path)?;
        let metadata = TsdfMetadata::read_from_tsdf(&mut file)?;

        // Make sure that the file format in the metadata matches the file
        // format passed in.
        if let Some(file_format) = file_format {
            if metadata.get_file_format() != &file_format {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "File format does not match existing file format.",
                ));
            }
        }

        // If we weren't passed a write mode, default to lockless write.
        let write_mode = write_mode.unwrap_or(WriteMode::LocklessWrite);
        let io_mode = IoMode::Write(write_mode);

        // If execution reaches here, we know that the write mode and file
        // format match the existing file's write mode and file format. Return
        // the TsdfFile.
        Ok(Box::new(TsdfFile {
            path,
            file: File::open(path)?,
            io_metadata: IoMetadata::new(metadata, io_mode),
        }))
    }

    fn new_overwriting_writer(
        path: &'static Path,
        write_mode: Option<crate::core::enums::WriteMode>,
        file_format: Option<FileFormat>,
    ) -> io::Result<Box<Self>> {
        // Get the parent directory of the file.
        if let Some(parent) = path.parent() {
            // Create the parent directory if it doesn't exist.
            create_dir_all(parent)?;
        }

        // Delete the file if it exists.
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        // Now create the file.
        let file = File::create(path)?;

        // Get the version from cargo.
        let version = env!("CARGO_PKG_VERSION");

        // Make a new TsdfMetadata.
        let metadata = {
            let file_format = file_format.unwrap_or(FileFormat::Binary);
            TsdfMetadata::new(version.to_string(), file_format)
        };

        // Write the metadata to the beginning of the file.
        let metadata_json = serde_json::to_vec(&metadata)?;
        file.write_at(metadata_json.as_slice(), 0)?;

        // Flush the file to disk.
        file.sync_all()?;

        // If we weren't passed a write mode, default to lockless write.
        let write_mode = write_mode.unwrap_or(WriteMode::LocklessWrite);
        let io_mode = IoMode::Write(write_mode);

        // Return the TsdfFile.
        Ok(Box::new(TsdfFile {
            path,
            file,
            io_metadata: IoMetadata::new(metadata, io_mode),
        }))
    }
}
