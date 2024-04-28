use crate::core::{enums::FileFormat, structs::IoMetadata};

/// This trait should be implemented by anything that can be written to disk in
/// any way. It is used to determine how much space the object will directly
/// take up on disk.
pub(crate) trait FixedSizeOnDisk {
    /// Get's the size of the object on disk, according to the current
    /// IoMetadata.
    fn get_size_on_disk(io_metadata: &IoMetadata) -> u64 {
        match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => Self::get_bin_size_on_disk(),
            FileFormat::Text => Self::get_json_size_on_disk(),
        }
    }

    /// Returns the size of the object once serialized to binary, in bytes.
    fn get_bin_size_on_disk() -> u64;

    /// Returns the size of the object once serialized to json, in bytes.
    /// # BE CAREFUL
    /// Please make sure that you consider the maximum possible size of the json
    /// string when implementing this method. If you aren't sure, bigger is
    /// better.
    ///
    /// Don't worry about performance in implementations. A good example of how
    /// to implement this can be found in the TsdfHash struct.
    fn get_json_size_on_disk() -> u64;
}
