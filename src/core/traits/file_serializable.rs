use std::{fs::File, os::unix::fs::FileExt};

use crate::core::{
    enums::FileFormat,
    structs::{Addr, IoMetadata},
};

use super::SizedOnDisk;

/// The FileSerializable trait is used to define objects that can be written to
/// and read from files. This trait is designed to be used with objects whose
/// state is written to disk, and read from disk, in one go. This makes it
/// useful for something like an individual key in a dictionary, but not for
/// the dictionary itself (where writes will be on a per-key/value basis).
pub(crate) trait FileSerializable: SizedOnDisk
where
    Self: serde::Serialize + serde::de::DeserializeOwned,
{
    /// Converts the object to a binary byte array. This should be hand
    /// optimized for performance.
    fn to_bin(&self) -> Vec<u8>;

    /// Creates a null representation of the object. This is written to the file
    /// to represent the absence of the object.
    fn null() -> Self;

    /// Converts the object to a json string. We provide a default
    /// implementation as this isn't performance critical and is only present
    /// for debugging purposes.
    fn to_json(&self) -> String {
        let mut json_string = serde_json::to_string(self).unwrap();

        // Pad the json string with spaces so that it is the same length as the
        // largest possible json string.
        let max_json_size = Self::get_json_size_on_disk();
        let json_size = json_string.len() as u64;
        let padding_size = max_json_size - json_size;
        for _ in 0..padding_size {
            json_string.push(' ');
        }

        json_string
    }

    /// Constructs the object from a byte array.
    fn from_bin(bytes: &[u8]) -> Self;

    /// Constructs the object from a json string.
    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    /// Writes the object to the file at the given location.
    fn write(&self, addr: Addr, file: &File, io_metadata: &IoMetadata) {
        // Depending on whether we're in binary or text mode, we'll write the
        // object differently.
        let bytes = match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => {
                // Convert the object to bytes.
                self.to_bin()
            }
            FileFormat::Text => {
                // Convert the object to a json string.
                let json = self.to_json();
                json.as_bytes().to_vec()
            }
        };

        file.write_all_at(&bytes, addr.get_loc()).unwrap();
    }

    /// Removes the object from the file at the given location. Removal is
    /// implemented by writing a null representation of the object to the file.
    fn remove(addr: Addr, file: &File, io_metadata: &IoMetadata) {
        // To remove an object, we just add a null representation of the object
        // to the file at the given location.
        let null = Self::null();
        null.write(addr, file, io_metadata);
    }

    /// Reads the object from the file at the given location. Returns None if
    /// the object is null.
    fn from_addr(addr: Addr, file: &File, io_metadata: &IoMetadata) -> Self
    where
        Self: Sized,
    {
        // Read the bytes from the file at the given location.
        let mut bytes = vec![0; Self::get_size_on_disk(io_metadata) as usize];
        file.read_at(&mut bytes, addr.get_loc()).unwrap();

        // Depending on whether we're in binary or text mode, we'll read the
        // object differently.
        match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => {
                // Convert the bytes to the object.
                Self::from_bin(bytes.as_slice())
            }
            FileFormat::Text => {
                // Convert the bytes to a json string.
                let json = String::from_utf8(bytes).unwrap();
                Self::from_json(json)
            }
        }
    }
}
