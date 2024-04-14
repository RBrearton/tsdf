use std::{fs::File, path::Path};

use crate::core::enums::WriteMode;
use crate::core::traits::TsdfFileTrait;

use super::Dir;

pub struct TsdfFile<'a, 'b> {
    /// The actual operating system path to the file.
    path: &'a Path,

    /// The semantic version of tsdf used to write the file.
    version: &'b str,

    /// The mode used to write the file.
    write_mode: WriteMode,

    /// The open file handle.
    file: File,
}
