use std::fs::File;

use crate::core::structs::{Addr, IoMetadata};

/// A trait for objects that have a location in a file.
pub(crate) trait Locatable {
    /// Returns a reference to the address of the object in the file.
    fn get_addr(&self) -> &Addr;

    /// Returns a reference to the file that the shard is stored in.
    fn get_file(&self) -> &File;

    /// Returns the metadata needed to read/write to the file.
    fn get_io_metadata(&self) -> &IoMetadata;
}
