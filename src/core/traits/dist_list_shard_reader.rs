use crate::core::structs::Addr;

use super::{FileSerializable, ShardTrait, VariableSizeOnDisk};

/// This trait defines everything we need to be able to do to read from a
/// shard of a distributed list in a file.
pub(crate) trait DistListShardReader<T: FileSerializable>:
    VariableSizeOnDisk + ShardTrait<T>
{
    /// Returns the nth element in the shard.
    fn get_element(&self, n: usize) -> T;

    /// Returns the nth is_element_written boolean in the shard.
    fn is_element_written(&self, n: usize) -> bool;

    /// Returns the address of the nth element in the shard.
    fn get_element_addr(&self, n: usize) -> Addr;

    /// Returns the address of the nth is_element_written boolean in the shard.
    fn get_is_element_written_addr(&self, n: usize) -> Addr;

    /// Returns whether the shard is full.
    fn is_full(&self) -> bool {
        // To find out if the shard is full, we check the final
        // is_element_written boolean in the shard.
        self.is_element_written(self.get_capacity() - 1)
    }
}
