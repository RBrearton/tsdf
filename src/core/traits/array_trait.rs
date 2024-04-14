use super::{FileSerializable, HasMetadataTags, HasName};

/// The high level array trait. This is a generally multi-dimensional array that is stored in the
/// tsdf file. This is where the bulk of your tsdf data will be stored.
pub(crate) trait ArrayTrait<T: FileSerializable>: HasMetadataTags + HasName {
    fn shape(&self) -> Vec<i32>;

    /// Gets the data at the given indices. The indices should be a list of integers, one for each
    /// dimension of the array.
    fn get(&self, indices: Vec<i32>) -> std::vec::Vec<T>;

    /// Appends the given data to the end of the array.
    fn append(&self, data: std::vec::Vec<T>);
}
