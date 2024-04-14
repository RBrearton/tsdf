use std::path::Path;

use crate::core::enums::FileFormat;
use crate::core::structs::Dir;

pub(crate) trait TsdfFileTrait {
    /// Returns the version of the file.
    fn get_version(&self) -> &str;

    /// Returns the path to the file.
    fn get_path(&self) -> &Path;

    /// Returns the write mode of the file.
    fn get_write_mode(&self) -> FileFormat;

    /// Returns the size of the file, in bytes.
    fn get_size(&self) -> u64;

    /// Returns the root Dir of the file.
    fn get_root_dir(&self) -> &Dir;

    /// Constructs a new TsdfFileTrait reader from the path provided as an argument.
    fn new_reader(path: &Path) -> Self;

    /// Constructs a new TsdfFileTrait writer from the path provided as an argument. Please note
    /// that, if the file already exists, the WriteMode must match the mode used to write the file,
    /// if specified.
    fn new_writer(path: &Path, write_mode: FileFormat) -> Self;
}
