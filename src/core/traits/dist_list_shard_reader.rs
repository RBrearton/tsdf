use super::{FileSerializable, Link, VariableSizeOnDisk};

/// TThis trait defines everything we need to be able to do to read from a
/// shard of a distributed list in a file.
pub(crate) trait DistListShardReader<T: FileSerializable>:
    Link + VariableSizeOnDisk
{
    /// Returns the number of elements in the shard.
    fn get_num_elements(&self) -> usize;
}
