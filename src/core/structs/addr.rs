use crate::core::traits::FileSerializable;

/// The Addr struct is a simple struct that holds an integer location. This location is
/// an offset within the file, and can be used to uniquely locate any byte in the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Addr {
    loc: u64,
}

impl Addr {
    pub(crate) fn new(loc: u64) -> Self {
        Self { loc }
    }

    pub(crate) fn get_loc(&self) -> u64 {
        self.loc
    }
}

impl FileSerializable for Addr {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = self.loc.to_le_bytes().to_vec();
        bytes
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let loc = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Self::new(loc)
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    fn get_size_on_disk() -> u64 {
        std::mem::size_of::<u64>() as u64
    }
}

// A simple test to make sure that we can convert an Addr to bytes/json and back.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can convert an Addr to bytes and back.
    #[test]
    fn test_addr_to_bytes() {
        let addr = Addr::new(123);
        let bytes = addr.to_bytes();
        let addr2 = Addr::from_bytes(&bytes);
        assert_eq!(addr.get_loc(), addr2.get_loc());
    }

    /// Test that we can convert an Addr to json and back.
    #[test]
    fn test_addr_to_json() {
        let addr = Addr::new(123);
        let json = addr.to_json();
        let addr2 = Addr::from_json(json);
        assert_eq!(addr.get_loc(), addr2.get_loc());
    }
}
