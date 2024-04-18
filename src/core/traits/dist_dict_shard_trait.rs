use std::{fs::File, hash::Hash};

use crate::core::structs::{Addr, IoMetadata};

use super::{FileSerializable, Link};

/// The DistDictShard is part of a distributed dictionary. A DistDict is made up
/// of multiple DistDictShards, each of which is responsible for a subset of the
/// keys.
pub(crate) trait DistDictShardTrait<TKey: Hash, TVal: FileSerializable>:
    Link
{
    /// Returns whether the shard contains the given hash.
    fn contains(&self, hash: u64) -> bool;

    /// Adds a key-value pair to the shard. Note that we take the hash of the
    /// key as an argument to avoid recomputing it.
    /// You must first make sure that the shard doesn't already contain the
    /// key's hash, or this function will overwrite the existing value.
    fn add(&self, key: &TKey, val: &TVal, hash: u64);

    /// Removes a key-value pair from the shard. Note that we take the hash of
    /// the key as an argument to avoid recomputing it.
    fn remove(&self, key: &TKey, hash: u64);

    /// Gets the location of the nth hash in the shard.
    fn get_hash_loc(&self, n: usize) -> Addr;

    /// Gets the location of the nth value in the shard.
    fn get_val_loc(&self, n: usize) -> Addr;

    /// Gets the hash of the nth key in the shard.
    fn get_hash(&self, n: usize, file: &File, io_metadata: &IoMetadata) -> u64 {
        let addr = self.get_hash_loc(n);

        // The hash is stored as a u64, so we can read it directly from the
        // file.
        let mut bytes = vec![0; 8];
        file.read_at(&mut bytes, addr.get_loc()).unwrap();

        // Convert the bytes to a u64.
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Gets the value of the nth hash in the shard.
    fn get_val(&self, n: usize, file: &File, io_metadata: &IoMetadata) -> TVal {
        // Since TVal is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the value from the file.
        TVal::from_addr(self.get_val_loc(n), &file, io_metadata)
    }

    /// Returns the number of keys in the shard.
    fn get_num_keys(&self) -> usize;
}
