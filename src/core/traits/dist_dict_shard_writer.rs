use std::os::unix::fs::FileExt;

use crate::core::{enums::LinkPtr, structs::TsdfHash};

use super::{DistDictShardReader, FileSerializable};

/// A DistDictShardWriter can do everything a DistDictShardReader can do, but
/// also has the ability to write to disk.
pub(crate) trait DistDictShardWriter<TVal: FileSerializable>:
    DistDictShardReader<TVal>
{
    /// Initializes the DistDictShardWriter. This function should be called
    /// before any other functions are called on the DistDictShardWriter.
    fn init(&mut self) {
        // Iterate from 0 to the number of keys in the shard.
        for i in 0..self.get_num_keys() {
            // Get the location of the hash and value in the shard.
            let hash_loc = self.get_hash_addr(i);
            let val_loc = self.get_val_addr(i);
            let is_written_loc = self.get_is_hash_written_addr(i);

            // Write a null hash and null value to the file.
            TsdfHash::null().write(
                hash_loc,
                self.get_file(),
                self.get_io_metadata(),
            );
            TVal::null().write(
                val_loc,
                self.get_file(),
                self.get_io_metadata(),
            );

            // Finally, write a false boolean to the file to indicate that the
            // hash and value are not yet written.
            self.get_file()
                .write_at(&[0], is_written_loc.get_loc())
                .unwrap();
        }

        // Now that the shard is initialized, set the initialized flag to true.
        self.set_initialization_state(true);
    }

    /// Sets the next pointer of the shard to the given address.
    fn set_next(&mut self, next: LinkPtr);

    /// Checks whether the DistDictShardWriter has been initialized.
    fn is_initialized(&self) -> bool;

    /// Sets the initialized flag.
    fn set_initialization_state(&mut self, initialized: bool);

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
        let is_written_addr =
            self.get_is_hash_written_addr(hash_table_idx as usize);
        let hash_loc = self.get_hash_addr(hash_table_idx as usize);

        let val_loc = self.get_val_addr(hash_table_idx as usize);

        // Set the is_written boolean to false.
        self.get_file()
            .write_at(&[0], is_written_addr.get_loc())
            .unwrap();

        // Write the hash and value to the file.
        TsdfHash::remove(hash_loc, self.get_file(), self.get_io_metadata());
        TVal::remove(val_loc, self.get_file(), self.get_io_metadata());
    }

    /// Adds a key-value pair to the shard. Note that we take the hash of the
    /// key as an argument to avoid recomputing it.
    /// You must first make sure that the shard doesn't already contain the
    /// key's hash, or this function will overwrite the existing value.
    fn add(&mut self, hashed_key: &TsdfHash, val: &TVal) {
        if !self.is_initialized() {
            // If the shard hasn't been initialized, we must initialize it
            // before adding anything.
            self.init();
        }

        // Get the location of the hash in the shard.
        let num_keys = self.get_num_keys();
        let hash_table_idx = hashed_key.get_hash_table_idx(num_keys as u64);

        // Get the location of the hash and value in the shard.
        let is_written_addr =
            self.get_is_hash_written_addr(hash_table_idx as usize);
        let hash_loc = self.get_hash_addr(hash_table_idx as usize);
        let val_loc = self.get_val_addr(hash_table_idx as usize);

        // Write the hash and value to the file.
        hashed_key.write(hash_loc, self.get_file(), self.get_io_metadata());
        val.write(val_loc, self.get_file(), self.get_io_metadata());

        // Finally, write a true boolean to the file to indicate that the hash
        // and value are written. This order of writing is absolutely
        // fundamental to the whole file format, as boolean writes are atomic.
        // We can guarantee that, for any number of readers, the readers will
        // either see the hash and value as written or not written, but never
        // partially written.
        self.get_file()
            .write_at(&[1], is_written_addr.get_loc())
            .unwrap();
    }
}
