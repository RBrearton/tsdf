use std::path::Path;

use crate::core::enums::WriteMode;
use crate::core::structs::Dir;

pub(crate) trait TsdfFileTrait {
    /// Returns the version of the file.
    fn get_version(&self) -> &str;

    /// Returns the path to the file.
    fn get_path(&self) -> &Path;

    /// Returns the write mode of the file.
    fn get_write_mode(&self) -> WriteMode;

    /// Returns the size of the file, in bytes.
    fn get_size(&self) -> u64;

    /// Returns the root Dir of the file.
    fn get_root_dir(&self) -> &Dir;
}
