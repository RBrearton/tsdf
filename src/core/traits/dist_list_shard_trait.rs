use super::{FileSerializable, Link};

/// The DistListShard is part of a distributed list. A DistList is made up of
/// multiple DistListShards, each of which is responsible for a subset of the
/// elements.
pub(crate) trait DistListShardTrait<T: FileSerializable>: Link {
    /// Adds an element to the shard.
    fn add(&self, elem: T);

    /// Removes an element from the shard.
    fn remove(&self, elem: T);

    /// Gets the element at the given index.
    fn get(&self, index: i32) -> T;
}
