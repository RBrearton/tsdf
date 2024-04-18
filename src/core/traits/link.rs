use std::fs::File;

use crate::core::{enums::LinkPtr, structs::IoMetadata};

use super::Locatable;

pub(crate) trait Link: Locatable {
    /// Returns a reference to the next link.
    fn get_next(&self) -> &LinkPtr;

    /// Returns the link number of this link.
    fn get_link_number(&self) -> i32;

    /// Returns a reference to the file that the shard is stored in.
    fn get_file(&self) -> &File;

    /// Returns the metadata needed to read/write to the file.
    fn get_io_metadata(&self) -> &IoMetadata;
}
