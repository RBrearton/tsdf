use std::{fs::File, path::Path};

use crate::core::enums::FileFormat;
use crate::core::traits::TsdfFileTrait;

use super::Dir;

pub struct TsdfFile<'a, 'b> {
    /// The actual operating system path to the file.
    path: &'a Path,

    /// The semantic version of tsdf used to write the file.
    version: &'b str,

    /// The mode used to write the file.
    file_format: FileFormat,

    /// The open file handle.
    file: File,
}
