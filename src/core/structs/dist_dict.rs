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
