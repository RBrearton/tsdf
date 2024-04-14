use std::io;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use crate::core::enums::{FileFormat, IoMode};
use crate::core::traits::TsdfFileTrait;

use super::Dir;

pub struct TsdfFile<'a, 'b> {
    /// The actual operating system path to the file.
    path: &'a Path,

    /// The semantic version of tsdf used to write the file.
    version: &'b str,

    /// The mode used to write the file.
    file_format: FileFormat,

    /// The IoMode used to open the file.
    io_mode: IoMode,

    /// The open file handle.
    file: File,
}

impl TsdfFileTrait for TsdfFile<'_, '_> {
    fn get_version(&self) -> &str {
        self.version
    }

    fn get_path(&self) -> &Path {
        self.path
    }

    fn get_io_mode(&self) -> &IoMode {
        &self.io_mode
    }

    fn get_file_format(&self) -> &FileFormat {
        &self.file_format
    }

    fn get_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    fn get_root_dir(&self) -> &Dir {
        unimplemented!()
    }

    fn new(path: &'static Path, io_mode: IoMode, file_format: FileFormat) -> io::Result<Box<Self>> {
        // If we're expecting to have to write to the file, make sure that its directory exists, so
        // that File::create doesn't panic if the directory doesn't exist.
        if io_mode.is_write_mode() {
            // Get the parent directory of the file.
            if let Some(parent) = path.parent() {
                // Create the parent directory if it doesn't exist.
                create_dir_all(parent)?;
            }
        }

        // Now either open or create the file, depending on the IoMode.
        let file = match io_mode {
            IoMode::Read => File::open(path).unwrap(),
            IoMode::LockingWrite => File::create(path).unwrap(),
            IoMode::LocklessWrite => File::create(path).unwrap(),
        };

        // Get the version from cargo.
        let version = env!("CARGO_PKG_VERSION");

        Ok(Box::new(TsdfFile {
            path,
            version,
            file,
            file_format,
            io_mode,
        }))
    }
}
