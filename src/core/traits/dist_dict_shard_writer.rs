use crate::core::structs::TsdfHash;

use super::{DistDictShardReader, FileSerializable};

/// A DistDictShardWriter can do everything a DistDictShardReader can do, but
/// also has the ability to write to disk.
pub(crate) trait DistDictShardWriter<TVal: FileSerializable>:
    DistDictShardReader<TVal>
{
    /// Initializes the DistDictShardWriter. This function should be called
    /// before any other functions are called on the DistDictShardWriter.
    fn init(&self) -> bool;

    /// Checks whether the DistDictShardWriter has been initialized.
    fn is_initialized(&self) -> bool;

    /// Removes a key-value pair from the shard. Note that we take the hash of
    /// the key as an argument to avoid recomputing it.
    fn remove(&self, hashed_key: &TsdfHash) {
        if !self.is_initialized() {
            // If the shard hasn't been initialized, we can't remove anything.
            return;
        }

        // Get the location of the hash in the shard.
        let num_keys = self.get_num_keys();
        let hash_table_idx = hashed_key.get_hash_table_idx(num_keys as u64);

        // Get the location of the hash and value in the shard.
        let hash_loc = self.get_hash_loc(hash_table_idx as usize);

        let val_loc = self.get_val_loc(hash_table_idx as usize);

        // Write the hash and value to the file.
        TsdfHash::remove(hash_loc, self.get_file(), self.get_io_metadata());
        TVal::remove(val_loc, self.get_file(), self.get_io_metadata());
    }

    /// Adds a key-value pair to the shard. Note that we take the hash of the
    /// key as an argument to avoid recomputing it.
    /// You must first make sure that the shard doesn't already contain the
    /// key's hash, or this function will overwrite the existing value.
    fn add(&self, hashed_key: &TsdfHash, val: TVal) {
        if !self.is_initialized() {
            // If the shard hasn't been initialized, we must initialize it
            // before adding anything.
            self.init();
        }

        // Get the location of the hash in the shard.
        let num_keys = self.get_num_keys();
        let hash_table_idx = hashed_key.get_hash_table_idx(num_keys as u64);

        // Get the location of the hash and value in the shard.
        let hash_loc = self.get_hash_loc(hash_table_idx as usize);

        let val_loc = self.get_val_loc(hash_table_idx as usize);

        // Write the hash and value to the file.
        hashed_key.write(hash_loc, self.get_file(), self.get_io_metadata());
        val.write(val_loc, self.get_file(), self.get_io_metadata());
    }
}
