use std::fs::File;
use std::marker::PhantomData;

use crate::core::traits::{DistDictShardWriter, Locatable};
use crate::core::{
    enums::LinkPtr,
    traits::{DistDictShardReader, FileSerializable, Link},
};

use super::{Addr, IoMetadata};

/// The DistDictShard struct is a shard of a distributed dictionary. It is a
/// collection of keys and values that lives on disk, which is why the
/// DistDictShard struct itself doesn't contain the keys and values (as, for
/// large dictionaries, it would take far too long to deserialize the entire
/// shard). Instead, the DistDictShard is very good at answering questions like
/// "does this shard contain this key?" and "what is the value of this key?",
/// deserializing only minimal information from disk to answer these questions.
///
/// # Serialized structure
/// To locate an individual hash or value, we need to understand the structure
/// of the shard on disk. The shard looks like this:
///
/// | next: LinkPtr | hash1: TsdfHash | val1: T | ... | hashN | valN |
///
/// where T is the type of the value, and N is the number of keys in the
/// shard. The next LinkPtr comes first, and is followed by the hashes and
/// values. The hashes and values are interleaved, so that the first hash is
/// followed by the first value, the second hash is followed by the second
/// value, and so on.
pub(crate) struct DistDictShard<'a, 'b, TVal>
where
    TVal: FileSerializable,
{
    /// A throwaway variable to store the type of the value. We need this to
    /// make sure that every instance of DistDictShard is only capable of
    /// storing one kind of value. If we don't make this binding, then one
    /// instance of DistDictShard would, in theory, be able to store any kind of
    /// value, which would be a disaster.
    ///
    /// Specifically, the syntax for using functions defined in the
    /// DistDictShardTrait would be ambiguous, as the compiler wouldn't know
    /// which implementation of DistDictShardTrait to use. This way, when we
    /// make a DistDictShard we constrain its value type, and the compiler can
    /// work out which implementation of the generic DistDictShardTrait to use.
    val: PhantomData<TVal>,

    /// The LinkPtr to the next shard.
    next: LinkPtr,

    /// The link number of this shard.
    link_number: i32,

    /// The location of this shard in the file.
    loc: Addr,

    /// All metadata that is needed to read/write to the file.
    io_metadata: &'a IoMetadata,

    /// A reference to the file that the shard is stored in.
    file: &'b File,

    /// Whether the shard has been initialized.
    initialized: bool,
}

impl<TVal> DistDictShard<'_, '_, TVal>
where
    TVal: FileSerializable,
{
    //
}

// Implement the locatable trait for DistDictShard.
impl<TVal> Locatable for DistDictShard<'_, '_, TVal>
where
    TVal: FileSerializable,
{
    fn get_addr(&self) -> &Addr {
        &self.loc
    }
}

// Implement the Link trait for DistDictShard.
impl<TVal> Link for DistDictShard<'_, '_, TVal>
where
    TVal: FileSerializable,
{
    fn get_next(&self) -> &LinkPtr {
        &self.next
    }

    fn get_link_number(&self) -> i32 {
        self.link_number
    }

    fn get_file(&self) -> &File {
        self.file
    }

    fn get_io_metadata(&self) -> &IoMetadata {
        self.io_metadata
    }
}

// Implement the DistDictShardTrait for DistDictShard.
impl<TVal> DistDictShardReader<TVal> for DistDictShard<'_, '_, TVal>
where
    TVal: FileSerializable,
{
    fn get_num_keys(&self) -> usize {
        // The number of keys in the shard should be equal to 8 times the link
        // number raised to the power of 2.
        (8 * self.link_number.pow(2)) as usize
    }
}

// Implement the DistDictShardWriter trait for DistDictShard.
impl<TVal> DistDictShardWriter<TVal> for DistDictShard<'_, '_, TVal>
where
    TVal: FileSerializable,
{
    fn set_initialization_state(&mut self, initialized: bool) {
        self.initialized = initialized;
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek};

    use tempfile::tempfile;

    use crate::core::{
        enums::{IoMode, WriteMode},
        structs::TsdfMetadata,
    };

    use super::super::TsdfHash;
    use super::*;

    /// Test that we can add a key-value pair to the shard and then remove it.
    /// This test uses a text file format.
    #[test]
    fn test_add_remove() {
        // Define the io metadata.
        let io_metadata = IoMetadata::new(
            TsdfMetadata::new(
                "no_version".to_string(),
                crate::core::enums::FileFormat::Text,
            ),
            IoMode::Write(WriteMode::LocklessWrite),
        );
        let file = tempfile().unwrap();

        // Make a DistDictShard.
        let mut shard: DistDictShard<'_, '_, Addr> = DistDictShard {
            next: LinkPtr::Null(Addr::new(0)),
            link_number: 1,
            loc: Addr::new(0),
            io_metadata: &io_metadata,
            file: &file,
            val: std::marker::PhantomData,
            initialized: false,
        };

        // Now get the entire file as a string.
        let mut file_clone = file.try_clone().unwrap();

        // Initialize the shard.
        shard.init();
        let mut file_contents = String::new();
        file_clone.read_to_string(&mut file_contents).unwrap();
        file_clone.seek(std::io::SeekFrom::Start(0)).unwrap();
        println!("{}", file_contents);
        println!("\n\n\n\n\n\n\n\n");

        // Create a key-value pair.
        let key = "test_key".to_string();
        let hashed_key = TsdfHash::new(&key);
        let val = Addr::new(123);

        // Add the key-value pair to the shard.
        shard.add(&hashed_key, val);
        let mut file_contents = String::new();
        file_clone.read_to_string(&mut file_contents).unwrap();
        file_clone.seek(std::io::SeekFrom::Start(0)).unwrap();
        println!("{}", file_contents);
        println!("\n\n\n\n\n\n\n\n");

        // Check that the shard contains the key.
        assert!(shard.contains(&hashed_key));

        // Remove the key-value pair from the shard.
        // TODO: working here.
        shard.remove(&hashed_key);
        let mut file_contents = String::new();
        file_clone.read_to_string(&mut file_contents).unwrap();
        file_clone.seek(std::io::SeekFrom::Start(0)).unwrap();
        println!("{}", file_contents);
        println!("\n\n\n\n\n\n\n\n");

        // Check that the shard no longer contains the key.
        assert!(!shard.contains(&hashed_key));
    }
}
