use std::hash::DefaultHasher;

use crate::core::traits::FileSerializable;

/// The TsdfHash struct is a simple struct that holds a hash value. This hash

#[derive(PartialEq, Eq)]
pub(crate) struct TsdfHash {
    hash_value: u64,
}

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
}

impl FileSerializable for TsdfHash {
    fn to_bin(&self) -> Vec<u8> {
        let bytes = self.hash_value.to_le_bytes().to_vec();
        bytes
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_bin(bytes: &[u8]) -> Self {
        let hash_value = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Self { hash_value }
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    fn get_bin_size_on_disk() -> u64 {
        std::mem::size_of::<u64>() as u64
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
