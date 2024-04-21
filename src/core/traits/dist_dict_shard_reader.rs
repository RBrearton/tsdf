use crate::core::{
    enums::LinkPtr,
    structs::{Addr, TsdfHash},
};

use super::{FileSerializable, Link};

/// The DistDictShard is part of a distributed dictionary. A DistDict is made up
/// of multiple DistDictShards, each of which is responsible for a subset of the
/// keys.
pub(crate) trait DistDictShardReader<TVal: FileSerializable>:
    Link
{
    /// Returns the number of keys in the shard.
    fn get_num_keys(&self) -> usize;

    /// Returns whether the shard contains the given hash.
    fn contains(&self, hashed_key: &TsdfHash) -> bool {
        // To check if the shard contains a hash, we need to calculate the hash
        // modulo the number of keys to work out the hash's position in the
        // shard.
        let num_keys = self.get_num_keys();
        let n = hashed_key.get_hash_value() % num_keys as u64;

        // Now we need to check if the hash at position n is equal to the hash
        // we're looking for.
        let hash_n = self.get_hash(n as usize);

        *hashed_key == hash_n
    }

    /// Gets the location of the nth hash in the shard.
    fn get_hash_loc(&self, n: usize) -> Addr {
        // The location of the nth hash is the location of the shard plus the
        // size of the next LinkPtr, plus the size of each hash and value up to
        // the nth hash.
        let size_of_next = LinkPtr::get_size_on_disk(self.get_io_metadata());

        // The size of each hash is the size of a TsdfHash.
        let size_of_hash = TsdfHash::get_size_on_disk(self.get_io_metadata());

        // The size of each value is the size of a T.
        let size_of_val = TVal::get_size_on_disk(self.get_io_metadata());

        // The location of the nth hash is the location of the shard plus the
        // size of the next LinkPtr, plus the size of each hash and value up to
        // the nth hash.
        let loc = self.get_loc().get_loc()
            + size_of_next
            + (size_of_hash + size_of_val) * n as u64;

        Addr::new(loc)
    }

    /// Gets the location of the nth value in the shard.
    fn get_val_loc(&self, n: usize) -> Addr {
        // The location of the nth value is the location of the nth hash plus
        // the size of the hash.
        let size_of_hash = TsdfHash::get_size_on_disk(self.get_io_metadata());
        let loc = self.get_hash_loc(n).get_loc() + size_of_hash;

        Addr::new(loc)
    }

    /// Gets the hash of the nth key in the shard.
    fn get_hash(&self, n: usize) -> TsdfHash {
        // Since TsdfHash is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the hash from the file.
        TsdfHash::from_addr(
            self.get_hash_loc(n),
            self.get_file(),
            self.get_io_metadata(),
        )
    }

    /// Gets the value of the nth hash in the shard.
    fn get_val(&self, n: usize) -> TVal {
        // Since TVal is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the value from the file.
        TVal::from_addr(
            self.get_val_loc(n),
            self.get_file(),
            self.get_io_metadata(),
        )
    }
}
