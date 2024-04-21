use std::hash::{DefaultHasher, Hasher};

use crate::core::traits::FileSerializable;

/// The TsdfHash struct is a simple struct that holds a hash value. This hash

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct TsdfHash {
    hash_value: u64,
}

/// The value that we use to represent a null TsdfHash.
const NULL_HASH: u64 = 0;

impl TsdfHash {
    /// Creates a new TsdfHash from a hashable value.
    pub(crate) fn new<T: std::hash::Hash>(hashable: &T) -> Self {
        // Create a new hasher.
        let mut hasher = DefaultHasher::new();

        // Hash the value.
        hashable.hash(&mut hasher);
        let hash_value = hasher.finish();

        Self { hash_value }
    }

    /// Returns the hash value.
    pub(crate) fn get_hash_value(&self) -> u64 {
        self.hash_value
    }

    /// Works out what index this hash would be in a hash table of size
    ///`hash_table_size`.
    pub(crate) fn get_hash_table_idx(&self, hash_table_size: u64) -> u64 {
        self.hash_value % hash_table_size
    }
}

impl FileSerializable for TsdfHash {
    fn null() -> Self {
        Self {
            hash_value: NULL_HASH,
        }
    }

    fn to_bin(&self) -> Vec<u8> {
        let bytes = self.hash_value.to_le_bytes().to_vec();
        bytes
    }

    fn from_bin(bytes: &[u8]) -> Self {
        let hash_value = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Self { hash_value }
    }

    fn get_bin_size_on_disk() -> u64 {
        std::mem::size_of::<u64>() as u64
    }

    fn get_json_size_on_disk() -> u64 {
        // To get the size of the json string, we make the largest possible
        // TsdfHash, convert it to json, and get the length of the json string.
        // This is far from optimized, but the whole point of the json
        // representation is to be debug-friendly, not performant.
        let hash = TsdfHash {
            hash_value: u64::MAX,
        };
        let json = serde_json::to_string(&hash).unwrap();
        json.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can convert a TsdfHash to bytes and back.
    #[test]
    fn test_tsdf_hash_to_bytes() {
        let hash = TsdfHash::new(&123);
        let bytes = hash.to_bin();
        let hash2 = TsdfHash::from_bin(&bytes);
        assert_eq!(hash, hash2);
    }

    /// Test that we can convert a TsdfHash to json and back.
    #[test]
    fn test_tsdf_hash_to_json() {
        let hash = TsdfHash::new(&123);
        let json = hash.to_json();
        let hash2 = TsdfHash::from_json(json);
        assert_eq!(hash, hash2);
    }
}
