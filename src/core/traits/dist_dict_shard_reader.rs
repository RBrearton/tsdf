use std::os::unix::fs::FileExt;

use crate::core::{
    enums::LinkPtr,
    structs::{Addr, TsdfHash},
};

use super::{FileSerializable, Link, SizedOnDisk};

/// The DistDictShard is part of a distributed dictionary. A DistDict is made up
/// of multiple DistDictShards, each of which is responsible for a subset of the
/// keys.
///
/// The structure of the DistDictShard on disk is as follows:
///
/// | is_next_written (1 byte) | next (8 bytes) |
/// | hash_0 (8 bytes) | val_0 (var bytes) | is_hash_written_0 (1 byte) |
/// ...
/// | hash_n (8 bytes) | val_n (var bytes) | is_hash_written_n (1 byte) |
pub(crate) trait DistDictShardReader<TVal: FileSerializable>:
    Link + SizedOnDisk
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
    fn get_hash_addr(&self, n: usize) -> Addr {
        // The location of the nth hash is the location of the shard plus the
        // size of the is_next_written boolean, plus the size of the next
        // LinkPtr, plus the size of each hash, value and is_hash_written
        // boolean up to the nth hash.
        let size_of_next = LinkPtr::get_size_on_disk(self.get_io_metadata());

        // The size of each hash is the size of a TsdfHash.
        let size_of_hash = TsdfHash::get_size_on_disk(self.get_io_metadata());

        // The size of each value is the size of a T.
        let size_of_val = TVal::get_size_on_disk(self.get_io_metadata());

        // The size of the boolean is 1 byte.
        let size_of_bool = 1;

        // Determine the location of the nth hash.
        let addr = self.get_addr().get_loc()
            + size_of_bool // is_next_written
            + size_of_next // next
            + (size_of_hash + size_of_val + size_of_bool) * n as u64;

        Addr::new(addr)
    }

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

    /// Gets the location of the nth value in the shard.
    fn get_val_addr(&self, n: usize) -> Addr {
        // The location of the nth value is the location of the nth hash plus
        // the size of the hash.
        let size_of_hash = TsdfHash::get_size_on_disk(self.get_io_metadata());
        let addr = self.get_hash_addr(n).get_loc() + size_of_hash;

        Addr::new(addr)
    }

    /// Gets the location of the nth is_hash_written boolean in the shard.
    fn get_is_hash_written_addr(&self, n: usize) -> Addr {
        // The location of the nth is_hash_written boolean is the location of
        // the nth value plus the size of the value.
        let size_of_val = TVal::get_size_on_disk(self.get_io_metadata());
        let addr = self.get_val_addr(n).get_loc() + size_of_val;

        Addr::new(addr)
    }

    /// Gets the boolean that says whether the key value pair at the given index
    /// has been written.
    ///
    /// The reason for the is_hash_written field is related to atomicity.
    /// Imagine that we have a hash table on disk, and we want to add a new hash
    /// to it. If we have multiple threads reading the hash table when it is
    /// written, one thread could read the hash table before when not all of the
    /// hash value has been written (e.g., in binary mode, maybe only 2/8 bytes
    /// have been written). This would cause the thread to read a hash value
    /// that is not valid. To avoid this, we write the hash value in two steps:
    /// first, we write the hash value's 8 bytes. Then after that, we write a
    /// boolean flag that says that the hash value has been written. This way,
    /// before reading the hash value, we can check the boolean flag to see if
    /// the hash value has been written. Because binary boolean writes are
    /// atomic, we can be sure that if the boolean flag is true, the hash value
    /// is valid.
    fn is_hash_written(&self, n: usize) -> bool {
        // Read the boolean from the file.
        let loc = self.get_is_hash_written_addr(n).get_loc();
        let mut bytes = vec![0; 1];
        self.get_file().read_at(&mut bytes, loc).unwrap();

        // Convert the bytes to a boolean.
        bytes[0] == 1
    }

    /// Gets the boolean that says whether the next pointer has been written.
    /// This is similar to the is_hash_written boolean, but for the next
    /// pointer.
    fn is_next_written(&self) -> bool {
        // Read the boolean from the file.
        let loc = self.get_is_next_written_addr().get_loc();
        let mut bytes = vec![0; 1];
        self.get_file().read_at(&mut bytes, loc).unwrap();

        // Convert the bytes to a boolean.
        bytes[0] == 1
    }

    /// Gets the next pointer in the shard.
    fn get_next_ptr(&self) -> LinkPtr {
        // If the next pointer has not been written, we return a null pointer.
        if !self.is_next_written() {
            return LinkPtr::Null(Addr::null());
        }

        // Since LinkPtr is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the next pointer from the file.
        LinkPtr::from_addr(
            self.get_next_addr(),
            self.get_file(),
            self.get_io_metadata(),
        )
    }

    /// Gets the hash of the nth key in the shard.
    fn get_hash(&self, n: usize) -> TsdfHash {
        // If the hash has not been written, we return a null hash.
        if !self.is_hash_written(n) {
            return TsdfHash::null();
        }

        // Since TsdfHash is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the hash from the file.
        TsdfHash::from_addr(
            self.get_hash_addr(n),
            self.get_file(),
            self.get_io_metadata(),
        )
    }

    /// Gets the value of the nth hash in the shard.
    fn get_val(&self, n: usize) -> TVal {
        // If the hash has not been written, we return a null value.
        if !self.is_hash_written(n) {
            return TVal::null();
        }

        // Since TVal is guaranteed to be FileSerializable, we can use the
        // from_addr method to read the value from the file.
        TVal::from_addr(
            self.get_val_addr(n),
            self.get_file(),
            self.get_io_metadata(),
        )
    }
}
