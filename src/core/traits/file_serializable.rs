use std::{fs::File, os::unix::fs::FileExt};

use crate::core::{
    enums::FileFormat,
    structs::{Addr, IoMetadata},
};

/// The FileSerializable trait is used to define objects that can be written to
/// and read from files.
pub(crate) trait FileSerializable {
    /// Converts the object to a byte array.
    fn to_bytes(&self) -> Vec<u8>;

    /// Converts the object to a json string.
    fn to_json(&self) -> String;

    /// Constructs the object from a byte array.
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Constructs the object from a json string.
    fn from_json(json: String) -> Self;

    /// Returns the size of the object once serialized, in bytes.
    fn get_size_on_disk() -> u64;

    /// Writes the object to the file at the given location.
    fn write(&self, addr: Addr, file: &File, io_metadata: &IoMetadata) {
        // Depending on whether we're in binary or text mode, we'll write the
        // object differently.
        let bytes = match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => {
                // Convert the object to bytes.
                self.to_bytes()
            }
            FileFormat::Text => {
                // Convert the object to a json string.
                let json = self.to_json();
                json.as_bytes().to_vec()
            }
        };

        file.write_all_at(&bytes, addr.get_loc()).unwrap();
    }

    /// Reads the object from the file at the given location.
    fn from_addr(addr: Addr, file: &mut File, io_metadata: &IoMetadata) -> Self
    where
        Self: Sized,
    {
        // Read the bytes from the file at the given location.
        let mut bytes = vec![0; Self::get_size_on_disk() as usize];
        file.read_at(&mut bytes, addr.get_loc()).unwrap();

        // Depending on whether we're in binary or text mode, we'll read the
        // object differently.
        match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => {
                // Convert the bytes to the object.
                Self::from_bytes(bytes.as_slice())
            }
            FileFormat::Text => {
                // Convert the bytes to a json string.
                let json = String::from_utf8(bytes).unwrap();
                Self::from_json(json)
            }
        }
    }
}
