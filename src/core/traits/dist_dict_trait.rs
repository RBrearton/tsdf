use crate::core::{
    enums::LinkPtr,
    structs::{addr::Addr, DistDictShard},
};

use super::{
    DistDictShardReader, DistDictShardWriter, FileSerializable, Link,
    Locatable, TsdfHashable,
};

/// A distributed dictionary is a key-value store that is distributed across
/// multiple shards. Anything that implements this pub(crate) trait can behave
/// as a distributed dictionary.
pub(crate) trait DistDictTrait<TKey: TsdfHashable, TVal: FileSerializable>:
    Locatable
{
    /// Returns the first shard in the distributed dictionary.
    fn get_first_shard(&self) -> DistDictShard<'_, '_, TVal> {
        DistDictShard::<TVal>::new(
            0, // The first shard has link number 0.
            self.get_first_shard_addr(),
            self.get_io_metadata(),
            self.get_file(),
            // If the distributed dictionary is initialized, the first shard is
            // initialized.
            self.is_initialized(),
        )
    }

    /// Returns the address of the first shard in the distributed dictionary.
    fn get_first_shard_addr(&self) -> Addr;

    /// Initializes the distributed dictionary.
    fn init(&mut self) {
        // First we write the distributed dictionary's internal data to the
        // file.
        // Currently, this is just the first shard's address.
        self.get_first_shard_addr().write(
            self.get_addr().clone(),
            self.get_file(),
            self.get_io_metadata(),
        );

        // Initialize the first shard.
        let mut shard = self.get_first_shard();
        shard.init();

        // Set the initialized flag to true.
        self.set_initialization_state(true);
    }

    /// Returns whether the distributed dictionary has been initialized.
    fn is_initialized(&self) -> bool;

    /// Sets the initialized flag to the given value.
    fn set_initialization_state(&mut self, initialized: bool);

    /// Adds a key-value pair to the dictionary.
    fn add(&self, key: &TKey, val: &TVal) {
        // Start by hashing the key.
        let hashed_key = key.hash();

        // Now that we've got the key, we need to find the first shard in the
        // distributed dictionary that has space for the key.
        let mut shard = self.get_first_shard();

        loop {
            // Work out which index the key should be stored at in the shard.
            let num_keys = shard.get_num_keys();
            let hash_table_idx = hashed_key.get_hash_table_idx(num_keys as u64);

            // Check to see if there's already a value at that index.
            let is_written = shard.is_hash_written(hash_table_idx as usize);

            // If there's no value at that index, we can write the key-value
            // pair to the shard.
            if !is_written {
                shard.add(&hashed_key, val);
                return;
            }

            // If there is a value at that index, we need to check if the key
            // is already in the shard.
            let hash = shard.get_hash(hash_table_idx as usize);
            if hashed_key == hash {
                // If the key is already in the shard, we can update the value
                // associated with the key.
                shard.add(&hashed_key, val);
                return;
            }

            // If the key isn't in the shard, we need to move to the next shard.
            let next = shard.get_next();
            let link_number = shard.get_link_number();

            match next {
                // If the next pointer is null, we need to create a new shard.
                LinkPtr::Null(_) => {
                    // Create a new shard at the end of the file.
                    let file_end = self.get_file().metadata().unwrap().len();
                    let new_shard_addr = Addr::new(file_end);
                    let new_shard = DistDictShard::<TVal>::new(
                        link_number + 1,
                        new_shard_addr,
                        self.get_io_metadata(),
                        self.get_file(),
                        false,
                    );

                    // Set the next pointer of the current shard to point to the
                    // new shard.
                    shard.set_next(&LinkPtr::Addr(new_shard_addr));

                    // Set the new shard as the current shard.
                    shard = new_shard;
                }

                // If the next pointer is an address, we need to load the next
                // shard, which is stored at that address.
                LinkPtr::Addr(addr) => {
                    // Load the next shard.
                    shard = DistDictShard::new(
                        link_number + 1,
                        Addr::new(addr.get_loc()),
                        self.get_io_metadata(),
                        self.get_file(),
                        true,
                    )
                }
            }
        }
    }

    /// Removes a key-value pair from the dictionary.
    fn remove(&self, key: &TKey) {
        // Start by hashing the key.
        let hashed_key = key.hash();

        // Now that we've got the key, we need to find the first shard in the
        // distributed dictionary that has space for the key.
        let mut shard = self.get_first_shard();

        loop {
            // Check if the key is in the shard.
            if shard.contains(&hashed_key) {
                shard.remove(&hashed_key);
                return;
            }

            // If the key isn't in the shard, we need to move to the next shard.
            match shard.get_next() {
                // If the next pointer is null, we failed to find the key. This
                // is fine, it just means that we have nothing to delete.
                LinkPtr::Null(_) => {
                    return;
                }

                // If the next pointer is an address, we need to load the next
                // shard, which is stored at that address.
                LinkPtr::Addr(addr) => {
                    // Load the next shard.
                    shard = DistDictShard::new(
                        shard.get_link_number() + 1,
                        Addr::new(addr.get_loc()),
                        self.get_io_metadata(),
                        self.get_file(),
                        true,
                    )
                }
            }
        }
    }

    /// Gets the value associated with the given key. Returns None if the key is
    /// not in the dictionary.
    fn get(&self, key: &TKey) -> Option<TVal> {
        // Start by hashing the key.
        let hashed_key = key.hash();

        // Now that we've got the key, we need to find the first shard in the
        // distributed dictionary that has space for the key.
        let mut shard = self.get_first_shard();

        loop {
            // Check if the key is in the shard.
            let num_keys = shard.get_num_keys();
            let idx = hashed_key.get_hash_table_idx(num_keys as u64);
            if shard.contains(&hashed_key) {
                return Some(shard.get_val(idx as usize));
            }

            // If the key isn't in the shard, we need to move to the next shard.
            match shard.get_next() {
                // If the next pointer is null, we failed to find the key. In
                // this case, we panic.
                LinkPtr::Null(_) => return None,

                // If the next pointer is an address, we need to load the next
                // shard, which is stored at that address.
                LinkPtr::Addr(addr) => {
                    // Load the next shard.
                    shard = DistDictShard::new(
                        shard.get_link_number() + 1,
                        Addr::new(addr.get_loc()),
                        self.get_io_metadata(),
                        self.get_file(),
                        true,
                    )
                }
            }
        }
    }
}
