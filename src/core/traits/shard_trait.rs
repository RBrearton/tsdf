use crate::core::structs::Addr;

use super::{FileSerializable, Link, VariableSizeOnDisk};

pub(crate) trait ShardTrait<TVal: FileSerializable>:
    Link + VariableSizeOnDisk
{
    /// Returns the capacity of the shard. This is the number of slots for
    /// storing things in the shard. For a distributed list's shard, this is
    /// the number of elements that we could store in this shard of the list.
    /// For a distributed dictionary's shard, this is the number of key-value
    /// pairs that we could store in this shard of the dictionary.
    fn get_capacity(&self) -> usize;

    /// Returns the number of things currently stored in the shard.
    fn get_count(&self) -> usize;

    /// Gets the location of the is_next_written boolean in the shard.
    fn get_is_next_written_addr(&self) -> Addr {
        // The location of the is_next_written boolean is the location of the
        // shard plus the size of the is_next_written boolean.
        let addr = self.get_addr().get_loc();

        Addr::new(addr)
    }

    /// Gets the location of the next LinkPtr in the shard.
    fn get_next_addr(&self) -> Addr {
        // The location of the next LinkPtr is the location of the shard plus
        // the size of the is_next_written boolean.
        let size_of_bool = 1;
        let addr = self.get_is_next_written_addr().get_loc() + size_of_bool;

        Addr::new(addr)
    }
}
