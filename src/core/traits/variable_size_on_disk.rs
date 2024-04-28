use crate::core::{enums::FileFormat, structs::IoMetadata};

/// This trait is the same as FixedSizeOnDisk, but every method is an instance
/// method, rather than a static method.
pub(crate) trait VariableSizeOnDisk {
    /// Get's the size of the object on disk, according to the current
    /// IoMetadata.
    fn get_size_on_disk(&self, io_metadata: &IoMetadata) -> u64 {
        match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => self.get_bin_size_on_disk(),
            FileFormat::Text => self.get_json_size_on_disk(),
        }
    }

    /// Returns the size of the object once serialized to binary, in bytes.
    fn get_bin_size_on_disk(&self) -> u64;

    /// Returns the size of the object once serialized to json, in bytes.
    /// # BE CAREFUL
    /// Please make sure that you consider the maximum possible size of the json
    /// string when implementing this method. If you aren't sure, bigger is
    /// better.
    ///
    /// Don't worry about performance in implementations. A good example of how
    /// to implement this can be found in the TsdfHash struct.
    fn get_json_size_on_disk(&self) -> u64;
}
