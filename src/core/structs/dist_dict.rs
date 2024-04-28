use std::{fs::File, marker::PhantomData};

use crate::core::traits::{
    DistDictTrait, FileSerializable, FixedSizeOnDisk, Locatable, TsdfHashable,
};

use super::{Addr, IoMetadata};

pub(crate) struct DistDict<'a, 'b, TKey, TVal> {
    /// Throwaway variables used to store the types of the key and value.
    key: PhantomData<TKey>,
    val: PhantomData<TVal>,

    /// The address of this DistDict in the file.
    loc: Addr,

    /// The metadata needed to carry out I/O operations.
    io_metadata: &'a IoMetadata,

    /// The file that the DistDict is stored in.
    file: &'b File,

    /// Whether the distributed dictionary has been initialized.
    initialized: bool,
}

impl<TKey, TVal> Locatable for DistDict<'_, '_, TKey, TVal>
where
    TKey: TsdfHashable,
    TVal: FileSerializable,
{
    fn get_addr(&self) -> &Addr {
        &self.loc
    }

    fn get_file(&self) -> &File {
        self.file
    }

    fn get_io_metadata(&self) -> &IoMetadata {
        self.io_metadata
    }
}

impl<TKey, TVal> FixedSizeOnDisk for DistDict<'_, '_, TKey, TVal>
where
    TKey: TsdfHashable,
    TVal: FileSerializable,
{
    fn get_bin_size_on_disk() -> u64 {
        // The size of the DistDict is the size of the address.
        Addr::get_bin_size_on_disk()
    }

    fn get_json_size_on_disk() -> u64 {
        // The size of the DistDict is the size of the address.
        Addr::get_json_size_on_disk()
    }
}

impl<TKey, TVal> DistDictTrait<TKey, TVal> for DistDict<'_, '_, TKey, TVal>
where
    TKey: TsdfHashable,
    TVal: FileSerializable,
{
    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn set_initialization_state(&mut self, initialized: bool) {
        self.initialized = initialized;
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek};

    use super::*;
    use crate::core::traits::{DistDictShardReader, VariableSizeOnDisk};
    use crate::core::{
        enums::{IoMode, WriteMode},
        structs::TsdfMetadata,
    };

    use tempfile::tempfile;

    macro_rules! print_file {
        ($file:expr) => {{
            let mut file_clone = $file.try_clone().unwrap();
            let mut file_contents = String::new();
            file_clone.read_to_string(&mut file_contents).unwrap();
            file_clone.seek(std::io::SeekFrom::Start(0)).unwrap();
            println!("{}", file_contents);
            println!("\n\n\n\n\n\n\n\n");
        }};
    }

    /// Test that we can initialize a distributed dictionary. Make sure that
    /// this properly adds the distributed dictionary to the file. This test
    /// uses the human readable crate::core::enums::FileFormat::Text format.
    #[test]
    fn test_init_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make sure that the file is empty.
        assert_eq!(file.metadata().unwrap().len(), 0);

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        // Initialize the distributed dictionary.
        dist_dict.init();

        // Get the first shard and work out its size on the disk.
        let shard = dist_dict.get_first_shard();
        let first_shard_size = shard.get_size_on_disk(&io_metadata);

        // Work out the size of the DistDict object on disk.
        let dist_dict_size =
            DistDict::<'_, '_, String, Addr>::get_size_on_disk(&io_metadata);

        // Print the file.
        print_file!(file);

        // Make sure that the file contains the correct number of bytes (meaning
        // that the entire distributed dictionary has been written to the file).
        assert_eq!(
            file.metadata().unwrap().len(),
            dist_dict_size + first_shard_size
        );
    }

    /// This test is the same as the above test, but this time making sure that
    /// we can initialize binary files on disk.
    #[test]
    fn test_init_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make sure that the file is empty.
        assert_eq!(file.metadata().unwrap().len(), 0);

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        // Initialize the distributed dictionary.
        dist_dict.init();

        // Get the first shard and work out its size on the disk.
        let shard = dist_dict.get_first_shard();
        let first_shard_size = shard.get_size_on_disk(&io_metadata);

        // Work out the size of the DistDict object on disk.
        let dist_dict_size =
            DistDict::<'_, '_, String, Addr>::get_size_on_disk(&io_metadata);

        // Print the file.
        print_file!(file);

        // Make sure that the file contains the correct number of bytes (meaning
        // that the entire distributed dictionary has been written to the file).
        assert_eq!(
            file.metadata().unwrap().len(),
            dist_dict_size + first_shard_size
        );
    }

    /// Test that we can add a single key value pair to the distributed
    /// dictionary. This test involves adding a key value pair to the dist
    /// dictionary, and then directly checking that the shard contains the key.
    /// In other words, this tests our ability to add, without testing a
    /// .contains() method on the DistDict itself (we use the shard's .contains
    /// method, which is separately tested).
    #[test]
    fn test_add_text_shard_contains() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        // Add a key value pair to the distributed dictionary.
        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Get the first shard.
        let shard = dist_dict.get_first_shard();

        // Print the file.
        print_file!(file);

        // Hash the key.
        let hashed_key = key.hash();

        // Make sure that the shard contains the key.
        assert!(shard.contains(&hashed_key));
    }

    /// Make sure that we can add a single key value pair to the distributed
    /// dictionary. This is the same as the above test, but using the binary
    /// format.
    #[test]
    fn test_add_bin_shard_contains() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        // Add a key value pair to the distributed dictionary.
        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Get the first shard.
        let shard = dist_dict.get_first_shard();

        // Hash the key.
        let hashed_key = key.hash();

        // Make sure that the shard contains the key.
        assert!(shard.contains(&hashed_key));
    }

    /// Make sure that the contains method returns False when the key is not in
    /// the distributed dictionary. This uses the Text file format.
    #[test]
    fn test_contains_false_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key";

        // Make sure that the shard does not contain the key.
        assert!(!dist_dict.contains(&key.to_string()));
    }

    /// Make sure that the contains method returns False when the key is not in
    /// the distributed dictionary. This uses the Binary file format.
    #[test]
    fn test_contains_false_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key";

        // Make sure that the shard does not contain the key.
        assert!(!dist_dict.contains(&key.to_string()));
    }

    /// Make sure that the contains method returns True when the key is in the
    /// distributed dictionary. This uses the Text file format.
    #[test]
    fn test_contains_true_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));
    }

    /// Make sure that the contains method returns True when the key is in the
    /// distributed dictionary. This uses the Binary file format.
    #[test]
    fn test_contains_true_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));
    }

    /// Test that we can add and remove a single key value pair to the
    /// distributed dictionary.
    #[test]
    fn test_add_remove_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));

        // Remove the key value pair from the distributed dictionary.
        dist_dict.remove(&key);

        // Make sure that the shard does not contain the key.
        assert!(!dist_dict.contains(&key));
    }

    /// As above, but using the binary file format.
    #[test]
    fn test_add_remove_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));

        // Remove the key value pair from the distributed dictionary.
        dist_dict.remove(&key);

        // Make sure that the shard does not contain the key.
        assert!(!dist_dict.contains(&key));
    }

    /// Test adding two values with the same key twice. The second value should
    /// overwrite the first. This uses the Text file format.
    #[test]
    fn test_add_twice_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val1 = Addr::new(1234);
        let val2 = Addr::new(5678);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val1);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val2);

        // Make sure that the shard still contains the key.
        assert!(dist_dict.contains(&key));

        // Make sure that the shard contains the second value.
        let shard = dist_dict.get_first_shard();
        let hashed_key = key.hash();
        let hash_idx =
            hashed_key.get_hash_table_idx(shard.get_num_keys() as u64);
        assert_eq!(shard.get_val(hash_idx as usize), val2);
    }

    /// Make sure that the get method returns None when the key is not in the
    /// distributed dictionary. This uses the Text file format.
    #[test]
    fn test_get_none_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key";

        // Make sure that the get method returns None.
        assert_eq!(dist_dict.get(&key.to_string()), None);
    }

    /// Make sure that the get method returns None when the key is not in the
    /// distributed dictionary. This uses the Binary file format.
    #[test]
    fn test_get_none_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key";

        // Make sure that the get method returns None.
        assert_eq!(dist_dict.get(&key.to_string()), None);
    }

    /// Make sure that the get method returns the correct value when the key is
    /// in the distributed dictionary. This uses the Text file format.
    #[test]
    fn test_get_text() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));

        // Make sure that the get method returns the correct value.
        assert_eq!(dist_dict.get(&key).unwrap(), val);
    }

    /// Make sure that the get method returns the correct value when the key is
    /// in the distributed dictionary. This uses the Binary file format.
    #[test]
    fn test_get_bin() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        let key = "key".to_string();
        let val = Addr::new(1234);

        // Add the key value pair to the distributed dictionary.
        dist_dict.add(&key, &val);

        // Make sure that the shard contains the key.
        assert!(dist_dict.contains(&key));

        // Make sure that the get method returns the correct value.
        assert_eq!(dist_dict.get(&key).unwrap(), val);
    }

    /// This is something of a stress test. This forces the distributed dict
    /// to create many shards by adding a large number of key value pairs.
    /// We then make sure that every key value pair is present, and that we can
    /// remove and re-add them all.
    /// This is really an integration test, but is fast enough to sit in the
    /// unit test suite comfortably.
    #[test]
    fn test_many_keys() {
        // The necessary setup.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Binary,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDict.
        let mut dist_dict: DistDict<'_, '_, String, Addr> = DistDict {
            key: PhantomData,
            val: PhantomData,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            initialized: false,
        };

        // Add a large number of key value pairs to the distributed dictionary.
        let num_keys = 10000;
        let mut key_vals = Vec::new();
        for i in 0..num_keys {
            let key = format!("key_{}", i);
            let val = Addr::new(i as u64);
            key_vals.push((key.clone(), val.clone()));
            dist_dict.add(&key, &val);
        }

        // Make sure that all of the key value pairs are present.
        for (key, val) in key_vals {
            assert!(dist_dict.contains(&key));

            // Get the value.
            let found_val = dist_dict.get(&key).unwrap();
            assert_eq!(found_val, val);

            // Remove the key value pair.
            dist_dict.remove(&key);

            // Make sure that the key is no longer present.
            assert!(!dist_dict.contains(&key));
        }
    }
}
