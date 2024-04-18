use std::fs::File;

use crate::core::traits::Locatable;
use crate::core::traits::TsdfHashable;
use crate::core::{
    enums::LinkPtr,
    traits::{DistDictShardTrait, FileSerializable, Link},
};

use super::{Addr, IoMetadata, TsdfHash};

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
pub(crate) struct DistDictShard<'a, 'b> {
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
}

impl DistDictShard<'_, '_> {
    //
}

// Implement the locatable trait for DistDictShard.
impl Locatable for DistDictShard<'_, '_> {
    fn get_loc(&self) -> &Addr {
        &self.loc
    }
}

// Implement the Link trait for DistDictShard.
impl Link for DistDictShard<'_, '_> {
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
impl<TKey, TVal: FileSerializable> DistDictShardTrait<TKey, TVal>
    for DistDictShard<'_, '_>
where
    TKey: TsdfHashable,
    TVal: FileSerializable,
{
    fn get_hash_loc(&self, n: usize) -> Addr {
        // The location of the nth hash is the location of the shard plus the
        // size of the next LinkPtr, plus the size of each hash and value up to
        // the nth hash.
        let size_of_next = self.get_next().get_size_on_disk(self.io_metadata);

        // The size of each hash is the size of a TsdfHash.
        // let size_of_hash = self
        todo!()
    }

    fn get_val_loc(&self, n: usize) -> Addr {
        todo!()
    }

    fn get_num_keys(&self) -> usize {
        // The number of keys in the shard should be equal to 8 times the link
        // number raised to the power of 2.
        (8 * self.link_number.pow(2)) as usize
    }

    fn contains(&self, hash: TsdfHash) -> bool {
        // Now
        todo!()
    }

    fn add(&self, key: &TKey, val: &TVal, hash: TsdfHash) {
        todo!()
    }

    fn remove(&self, key: &TKey, hash: TsdfHash) {
        todo!()
    }
}
