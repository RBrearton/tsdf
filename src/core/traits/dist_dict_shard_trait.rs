use super::{FileSerializable, Link};

/// The DistDictShard is part of a distributed dictionary. A DistDict is made up
/// of multiple DistDictShards, each of which is responsible for a subset of the
/// keys.
pub(crate) trait DistDictShardTrait<
    TKey: FileSerializable,
    TVal: FileSerializable,
>: Link
{
    /// Returns whether the shard contains the given hash.
    fn contains(&self, hash: u64) -> bool;

    /// Adds a key-value pair to the shard. Note that we take the hash of the
    /// key as an argument to avoid recomputing it.
    /// You must first make sure that the shard doesn't already contain the
    /// key's hash, or this function will overwrite the existing value.
    fn add(&self, key: TKey, val: TVal, hash: u64);

    /// Removes a key-value pair from the shard. Note that we take the hash of
    /// the key as an argument to avoid recomputing it.
    fn remove(&self, key: TKey, hash: u64);
}
