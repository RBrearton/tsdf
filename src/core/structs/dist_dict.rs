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
    use std::ops::Add;

    use super::*;
    use crate::core::traits::VariableSizeOnDisk;
    use crate::core::{
        enums::{IoMode, WriteMode},
        structs::{TsdfHash, TsdfMetadata},
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
    /// dictionary. This should involve adding a shard to the distributed dict,
    /// initializing the shard, and then adding the key-value pair to the shard.
    /// This test uses human readable crate::core::enums::FileFormat::Text
    /// format.
    #[test]
    fn test_add_text() {
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

        // Not yet implemented.
        unimplemented!()
    }

    /// Test that we can add and remove a single key value pair to the
    /// distributed dictionary. This follows the logic in the above test, but
    /// also removes the key-value pair after adding it. This test uses human
    /// readable FileFormat::Text format.
    #[test]
    fn test_add_remove_text() {
        // Not yet implemented.
        unimplemented!()
    }

    /// Test that we can add a single key value pair to the distributed
    /// dictionary. This test uses the production binary format.
    #[test]
    fn test_add_bin() {
        // Not yet implemented.
        unimplemented!()
    }

    /// Test that we can add and remove a single key value pair to the
    /// distributed dictionary. This test uses the production binary format.
    /// This test uses the production binary format.
    #[test]
    fn test_add_remove_bin() {
        // Not yet implemented.
        unimplemented!()
    }

    /// This is something of a stress test. This forces the distributed dict
    /// to create many shards by adding a large number of key value pairs.
    /// We then make sure that every key value pair is present, and that we can
    /// remove and re-add them all.
    /// This is really an integration test, but is fast enough to sit in the
    /// unit test suite comfortably.
    #[test]
    fn test_many_keys() {
        // Not yet implemented.
        unimplemented!()
    }
}
