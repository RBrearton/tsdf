use crate::core::enums::IoMode;

use super::TsdfMetadata;

/// All metadata required to carry out an I/O operation in the tsdf library. This includes the
/// TsdfMetadata stored in the file, as well as the IoMode that is being used to interact with the
/// file.
pub(super) struct IoMetadata {
    /// The core metadata for the tsdf file.
    tsdf_metadata: TsdfMetadata,

    /// The IoMode used to interact with the file.
    io_mode: IoMode,
}

impl IoMetadata {
    /// Returns the TsdfMetadata stored in the file.
    pub fn get_tsdf_metadata(&self) -> &TsdfMetadata {
        &self.tsdf_metadata
    }

    /// Returns the IoMode used to interact with the file.
    pub fn get_io_mode(&self) -> &IoMode {
        &self.io_mode
    }
}
