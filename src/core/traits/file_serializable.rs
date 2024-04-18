use std::{fs::File, os::unix::fs::FileExt};

use crate::core::{
    enums::FileFormat,
    structs::{Addr, IoMetadata},
};

/// The FileSerializable trait is used to define objects that can be written to
/// and read from files.
pub(crate) trait FileSerializable
where
    Self: serde::Serialize + serde::de::DeserializeOwned,
{
    /// Converts the object to a binary byte array. This should be hand
    /// optimized for performance.
    fn to_bin(&self) -> Vec<u8>;

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
    fn from_json(json: String) -> Self;

    /// Get's the size of the object on disk, according to the current
    /// IoMetadata.
    fn get_size_on_disk(&self, io_metadata: &IoMetadata) -> u64 {
        match io_metadata.get_tsdf_metadata().get_file_format() {
            FileFormat::Binary => Self::get_bin_size_on_disk(),
            FileFormat::Text => Self::get_json_size_on_disk(),
        }
    }

    /// Returns the size of the object once serialized to binary, in bytes.
    fn get_bin_size_on_disk() -> u64;

    /// Returns the size of the object once serialized to json, in bytes.
    /// # BE CAREFUL
    /// Please make sure that you consider the maximum possible size of the json
    /// string when implementing this method. If you aren't sure, bigger is
    /// better.
    ///
    /// Don't worry about performance in implementations. A good example of how
    /// to implement this can be found in the TsdfHash struct.
    fn get_json_size_on_disk() -> u64;

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

    /// Reads the object from the file at the given location.
    fn from_addr(addr: Addr, file: &File, io_metadata: &IoMetadata) -> Self
    where
        Self: Sized,
    {
        // Read the bytes from the file at the given location.
        let mut bytes = vec![0; Self::get_bin_size_on_disk() as usize];
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
