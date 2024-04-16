use std::fs::File;

use crate::core::structs::{Addr, IoMetadata};

pub(crate) trait FileSerializable {
    /// Writes the object to the file at the given location.
    fn write(&self, addr: Addr, file: &mut File, io_metadata: &IoMetadata);

    /// Reads the object from the file at the given location.
    fn from_addr(addr: Addr, file: &mut File, io_metadata: &IoMetadata) -> Self;
}
