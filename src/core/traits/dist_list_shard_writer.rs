use super::{FileSerializable, Link};

pub(crate) trait DistListShardWriter<T: FileSerializable>: Link {
    /// Adds an element to the shard.
    fn add(&self, elem: T);

    /// Removes an element from the shard.
    fn remove(&self, elem: T);

    /// Gets the element at the given index.
    fn get(&self, index: i32) -> T;
}
