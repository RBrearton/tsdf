use std::io;
use std::path::Path;

use crate::core::enums::{FileFormat, IoMode, WriteMode};
use crate::core::structs::{Dir, IoMetadata, TsdfMetadata};

pub trait TsdfFileTrait {
    /// Returns the version of the file.
    fn get_version(&self) -> &str;

    /// Returns the path to the file.
    fn get_path(&self) -> &Path;

    /// Returns the io mode of the file.
    fn get_io_mode(&self) -> &IoMode;

    /// Returns the file format of the file.
    fn get_file_format(&self) -> &FileFormat;

    /// Returns the file's IoMetadata object.
    fn get_io_metadata(&self) -> &IoMetadata;

    /// Returns the file's TsdfMetadata object.
    fn get_tsdf_metadata(&self) -> &TsdfMetadata;

    /// Returns the size of the file, in bytes.
    fn get_size(&self) -> u64;

    /// Returns the root Dir of the file.
    fn get_root_dir(&self) -> &Dir;

    /// Constructs a new TsdfFileTrait as a reader, taking a path as an argument.
    fn new_reader(path: &'static Path) -> io::Result<Box<Self>>;

    /// Constructs a new TsdfFileTrait as a writer.
    ///
    /// # Usage
    /// There are four options to consider:
    /// 1. If the file exists, and you don't pass a write_mode/file_format, theses will be read from
    ///   the file.
    /// 2. If the file exists, and you pass a write_mode/file_format, these must match the existing
    ///  file's write_mode/file_format, or the function will return an error.
    /// 3. If the file doesn't exist, and you don't pass a write_mode/file_format, these will
    ///  default to WriteMode::LocklessWrite and FileFormat::Binary.
    /// 4. If the file doesn't exist, and you pass a write_mode/file_format, these will be used.
    fn new_writer(
        path: &'static Path,
        write_mode: Option<WriteMode>,
        file_format: Option<FileFormat>,
    ) -> io::Result<Box<Self>>;

    /// Constructs a new TsdfFileTrait as a writer. This function will always create a new file,
    /// completely overwriting any existing file.
    fn new_overwriting_writer(
        path: &'static Path,
        write_mode: Option<WriteMode>,
        file_format: Option<FileFormat>,
    ) -> io::Result<Box<Self>>;
}
