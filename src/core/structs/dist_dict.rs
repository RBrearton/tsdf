use std::{fs::File, marker::PhantomData};

use crate::core::traits::{
    DistDictTrait, FileSerializable, Locatable, TsdfHashable,
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

    /// The location of the first shard of this distributed dictionary.
    first_shard_addr: Addr,

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

impl<TKey, TVal> DistDictTrait<TKey, TVal> for DistDict<'_, '_, TKey, TVal>
where
    TKey: TsdfHashable,
    TVal: FileSerializable,
{
    fn get_first_shard_addr(&self) -> Addr {
        self.first_shard_addr
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn set_initialization_state(&mut self, initialized: bool) {
        self.initialized = initialized;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::structs::TsdfHash;

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

    /// Test that we can add a single key value pair to the distributed
    /// dictionary. This should involve adding a shard to the distributed dict,
    /// initializing the shard, and then adding the key-value pair to the shard.
    /// This test uses human readable crate::core::enums::FileFormat::Text
    /// format.
    #[test]
    fn test_add_text() {
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
}
