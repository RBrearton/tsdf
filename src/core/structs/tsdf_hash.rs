use crate::core::traits::FileSerializable;

/// The TsdfHash struct is a simple struct that holds a hash value. This hash
pub(crate) struct TsdfHash {
    hash_value: u64,
}

impl FileSerializable for TsdfHash {
    fn to_bin(&self) -> Vec<u8> {
        let bytes = self.hash_value.to_le_bytes().to_vec();
        bytes
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let hash_value = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Self::new(hash_value)
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    fn get_bin_size_on_disk() -> u64 {
        std::mem::size_of::<u64>() as u64
    }
}
