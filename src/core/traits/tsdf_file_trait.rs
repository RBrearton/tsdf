use std::io;
use std::path::Path;

use crate::core::enums::{FileFormat, IoMode};
use crate::core::structs::Dir;

pub trait TsdfFileTrait {
    /// Returns the version of the file.
    fn get_version(&self) -> &str;

    /// Returns the path to the file.
    fn get_path(&self) -> &Path;

    /// Returns the io mode of the file.
    fn get_io_mode(&self) -> &IoMode;

    /// Returns the file format of the file.
    fn get_file_format(&self) -> &FileFormat;

    /// Returns the size of the file, in bytes.
    fn get_size(&self) -> u64;

    /// Returns the root Dir of the file.
    fn get_root_dir(&self) -> &Dir;

    /// Constructs a new TsdfFileTrait, taking a path, IoMode and FileFormat as arguments.
    fn new(path: &'static Path, io_mode: IoMode, file_format: FileFormat) -> io::Result<Box<Self>>;
}
